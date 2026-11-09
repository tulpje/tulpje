use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sqlx::{PgExecutor, types::Json};
use std::hash::{DefaultHasher, Hash, Hasher as _};
use uuid::Uuid;

use tulpje_framework::Error;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "job_status")]
#[sqlx(rename_all = "lowercase")]
pub(crate) enum JobStatus {
    Pending,
    Busy,
    Dead,
    Processed,
}
impl Copy for JobStatus {}

pub(crate) struct DbJob<T> {
    pub(crate) uuid: Uuid,
    pub(crate) queue: String,
    pub(crate) name: String,
    pub(crate) data: Json<T>,
    pub(crate) hash: Option<i64>,
    pub(crate) status: JobStatus,
    pub(crate) priority: i16,
    pub(crate) attempts_remaining: i16,
    pub(crate) created_at: jiff_sqlx::DateTime,
    pub(crate) last_retrieved_at: Option<jiff_sqlx::DateTime>,
    pub(crate) finished_at: Option<jiff_sqlx::DateTime>,
}

#[derive(Debug, Clone)]
pub(crate) struct Job<T> {
    pub(crate) uuid: Uuid,
    pub(crate) queue: String,
    pub(crate) name: String,
    pub(crate) data: T,
    pub(crate) hash: Option<u64>,
    pub(crate) status: JobStatus,
    pub(crate) priority: u8,
    pub(crate) attempts_remaining: u8,
    pub(crate) created_at: jiff_sqlx::DateTime,
    pub(crate) last_retrieved_at: Option<jiff_sqlx::DateTime>,
    pub(crate) finished_at: Option<jiff_sqlx::DateTime>,
}

impl<T> From<DbJob<T>> for Job<T> {
    fn from(value: DbJob<T>) -> Self {
        Self {
            uuid: value.uuid,
            queue: value.queue,
            name: value.name,
            data: value.data.into_inner(),
            hash: value.hash.map(|v| v as u64),
            status: value.status,
            priority: value.priority as u8,
            attempts_remaining: value.attempts_remaining as u8,
            created_at: value.created_at,
            last_retrieved_at: value.last_retrieved_at,
            finished_at: value.finished_at,
        }
    }
}

pub(crate) struct JobRunData<T> {
    pub(crate) uuid: Uuid,
    pub(crate) data: T,
    pub(crate) attempts_remaining: u8,
}

pub async fn create<T>(
    exec: impl PgExecutor<'_>,
    queue: impl Into<String>,
    name: impl Into<String>,
    data: T,
    priority: u8,
    attempts_remaining: u8,
    unique: bool,
) -> Result<Job<T>, Error>
where
    T: Hash + Serialize + DeserializeOwned + Send + Sync + Unpin,
{
    let job_hash = unique.then(|| {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish() as i64
    });

    Ok(sqlx::query_file_as!(
        DbJob,
        "src/queries/insert_job.sql",
        queue.into(),
        name.into(),
        Json(data) as _,
        job_hash,
        i16::from(priority),
        i16::from(attempts_remaining),
    )
    .fetch_one(exec)
    .await?
    .into())
}

pub(crate) async fn update(
    exec: impl PgExecutor<'_>,
    uuid: Uuid,
    status: JobStatus,
) -> Result<bool, Error> {
    Ok(
        sqlx::query_file!("src/queries/update_job.sql", uuid, status as JobStatus)
            .execute(exec)
            .await?
            .rows_affected()
            > 0,
    )
}

pub(crate) async fn store_result(
    exec: impl PgExecutor<'_>,
    job_uuid: Uuid,
    success: bool,
    duration: u64,
    additional_info: impl Into<String>,
) -> Result<Uuid, Error> {
    Ok(sqlx::query_file_scalar!(
        "src/queries/insert_job_result.sql",
        job_uuid,
        success,
        duration as i64,
        additional_info.into()
    )
    .fetch_one(exec)
    .await?)
}

pub(crate) async fn find_by_uuid<T>(
    exec: impl PgExecutor<'_>,
    uuid: Uuid,
) -> Result<Option<Job<T>>, Error>
where
    T: DeserializeOwned + Send + Sync + Unpin,
{
    Ok(
        sqlx::query_file_as!(DbJob, "src/queries/select_job_by_uuid.sql", uuid)
            .map(|v| v.into())
            .fetch_optional(exec)
            .await?,
    )
}

pub(crate) async fn fetch_and_lock<T>(
    exec: impl PgExecutor<'_>,
    queue: &str,
) -> Result<Option<JobRunData<T>>, Error>
where
    T: for<'de> Deserialize<'de> + Send + Unpin,
{
    sqlx::query_file!("src/queries/fetch_and_lock_job.sql", queue)
        .fetch_optional(exec)
        .await?
        .map(|rec| -> Result<JobRunData<T>, Error> {
            Ok(JobRunData {
                uuid: rec.uuid,
                data: serde_json::from_value(rec.data)?,
                attempts_remaining: rec.attempts_remaining as u8,
            })
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[sqlx::test(migrations = "../../migrations")]
    async fn test_create(db: sqlx::PgPool) -> Result<(), tulpje_framework::Error> {
        let job = create(&db, "test", "job", String::from("foo"), 5, 3, false).await?;

        assert_eq!(job.queue, "test");
        assert_eq!(job.name, "job");
        assert_eq!(job.data, "foo");
        assert!(job.hash.is_none());

        Ok(())
    }

    #[ignore]
    #[sqlx::test(migrations = "../../migrations")]
    async fn test_create_unique(db: sqlx::PgPool) -> Result<(), tulpje_framework::Error> {
        let job = create(&db, "test", "job", String::from("foo"), 5, 3, true).await?;

        println!("{job:?}");
        assert_eq!(job.queue, "test");
        assert_eq!(job.name, "job");
        assert_eq!(job.data, "foo");
        assert_eq!(job.hash.expect("job hash missing"), 4506850079084802999u64);

        Ok(())
    }

    #[ignore]
    #[sqlx::test(migrations = "../../migrations")]
    async fn test_find_by_uuid(db: sqlx::PgPool) -> Result<(), tulpje_framework::Error> {
        let uuid = create(&db, "test", "job", String::from("foo"), 5, 3, true)
            .await?
            .uuid;

        let job = find_by_uuid::<String>(&db, uuid)
            .await?
            .expect("job not found");
        assert_eq!(job.queue, "test");

        Ok(())
    }

    async fn test_update(db: sqlx::PgPool) -> Result<(), tulpje_framework::Error> {
        let uuid = create(&db, "test", "job", String::from("foo"), 5, 3, true)
            .await?
            .uuid;

        assert!(update(&db, uuid, JobStatus::Busy).await?);
        let job = find_by_uuid::<String>(&db, uuid)
            .await?
            .expect("job not found");
        assert_eq!(job.status, JobStatus::Busy);

        Ok(())
    }

    async fn test_store_result(db: sqlx::PgPool) -> Result<(), tulpje_framework::Error> {
        let uuid = create(&db, "test", "job", String::from("foo"), 5, 3, true)
            .await?
            .uuid;

        store_result(&db, uuid, true, 17, "done").await?;
        Ok(())
    }
}

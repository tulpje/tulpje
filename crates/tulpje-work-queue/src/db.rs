use serde::Deserialize;
use std::time::Instant;

use tulpje_framework::Error;

use crate::job::{self, JobStatus};

async fn process_job<T, F, Fut>(db: &sqlx::PgPool, queue: &str, func: F) -> Result<(), Error>
where
    T: for<'de> Deserialize<'de> + Send + Unpin,
    F: FnOnce(T) -> Fut,
    Fut: Future<Output = Result<String, Error>>,
{
    let mut tx = db.begin().await?;

    let Some(job) = job::fetch_and_lock(&mut *tx, queue).await? else {
        return Ok(());
    };

    let instant_before = Instant::now();
    let (job_status, additional_info) = match func(job.data).await {
        Ok(additional_info) => (JobStatus::Processed, additional_info),
        Err(err) if job.attempts_remaining == 0 => (JobStatus::Dead, err.to_string()),
        Err(err) => (JobStatus::Pending, err.to_string()),
    };
    let duration_ms = instant_before.elapsed().as_millis();

    job::update(&mut *tx, job.uuid, job_status).await?;
    job::store_result(
        &mut *tx,
        job.uuid,
        job_status == JobStatus::Processed,
        duration_ms.clamp(u64::MIN.into(), u64::MAX.into()) as u64,
        additional_info,
    )
    .await?;

    tx.commit().await?;
    Ok(())
}

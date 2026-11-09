SELECT
  uuid,
  queue,
  name,
  data AS "data: Json<T>",
  hash,
  status AS "status: JobStatus",
  priority AS "priority: i8",
  attempts_remaining AS "attempts_remaining: i8",
  created_at AS "created_at: jiff_sqlx::DateTime",
  last_retrieved_at AS "last_retrieved_at: jiff_sqlx::DateTime",
  finished_at AS "finished_at: jiff_sqlx::DateTime"
FROM
  jobs
WHERE
  uuid = $1

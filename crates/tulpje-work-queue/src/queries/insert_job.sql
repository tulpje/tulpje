INSERT INTO jobs (
  queue,
  name,
  data,
  hash,
  priority,
  attempts_remaining
) VALUES (
  $1, $2, $3, $4, $5, $6
)
RETURNING
  uuid,
  queue,
  name,
  data AS "data: Json<T>",
  hash,
  status AS "status: JobStatus",
  priority,
  attempts_remaining,
  created_at AS "created_at: jiff_sqlx::DateTime",
  last_retrieved_at AS "last_retrieved_at: jiff_sqlx::DateTime",
  finished_at AS "finished_at: jiff_sqlx::DateTime"
;

INSERT INTO job_results (
  job_uuid,
  success,
  duration,
  additional_info
) VALUES (
  $1, $2, $3, $4
)
RETURNING
  uuid
;

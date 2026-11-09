UPDATE jobs
SET
	status = 'busy',
	attempts_remaining = attempts_remaining - 1,
	last_retrieved_at = NOW()
WHERE uuid IN (
	SELECT
		uuid
	FROM
		jobs
	WHERE
		status = 'pending'
		AND	attempts_remaining > 0
		AND	queue = $1
	ORDER BY
		priority DESC
	LIMIT 1
	FOR UPDATE SKIP LOCKED
)
RETURNING
	uuid,
  name,
	data,
	attempts_remaining AS "attempts_remaining: i8";

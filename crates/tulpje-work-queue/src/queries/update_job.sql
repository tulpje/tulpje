UPDATE jobs
SET
	status = $2
WHERE
	uuid = $1;

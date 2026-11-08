--- status that a job can have
CREATE TYPE job_status AS ENUM (
	'pending',  -- job needs to be picked up
	'busy',     -- job is currently running
	'dead',     -- job exhausted all retry attempts
	'processed' -- job completed succesfully
);


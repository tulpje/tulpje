--- primary jobs table
CREATE TABLE jobs (
  uuid UUID PRIMARY KEY DEFAULT uuidv7(),
  queue TEXT NOT NULL,    -- name of queue, separate ones can have different concurrency
  name TEXT NOT NULL,     -- name of the job
  data JSONB NOT NULL,    -- job data
  hash BIGINT,            -- optional unique hash for jobs that need to be unique
  status job_status NOT NULL DEFAULT 'pending',
  priority SMALLINT NOT NULL CHECK (priority >= 0 AND priority <= 255), -- enforce uint8 bounds
  attempts_remaining SMALLINT NOT NULL CHECK (attempts_remaining >= 0 AND attempts_remaining <= 255), -- enforce uint8 bounds
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),  -- when the job was created
  last_retrieved_at TIMESTAMP NULL,             -- last time the job was acquired
  finished_at TIMESTAMP NULL,                   -- when the job was finished (dead or processed)

  -- finished jobs should always have `finished_at` set
  CONSTRAINT valid_finished_job CHECK (
    (
	  finished_at IS NOT NULL
	  AND
	  status IN ('dead', 'processed')
	)
	  OR
	(
	  finished_at IS NULL
	  AND
	  status IN ('pending', 'busy')
	)
  ),

  -- `last_retrieved_at` needs to be later than `created_at`
  constraint last_retrieved_at__after__created_at CHECK (
	  last_retrieved_at IS NULL
	  OR
	  last_retrieved_at >= created_at
  ),

  -- `finished_at` needs to be later than `created_at` and `last_retried_at`
  constraint finished_at__after__created_at__and__last_retrieved_at CHECK (
	  finished_at IS NULL OR (
	    finished_at >= created_at
	    AND
	    finished_at >= last_retrieved_at
	  )
  ),
  -- make sure unique jobs can only exist onces
  -- NOTE: postgresql considers NULL values not equal, so this works without issues
  CONSTRAINT unique_job UNIQUE (queue, name, hash)
);

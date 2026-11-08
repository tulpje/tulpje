--- job results table
CREATE TABLE job_results (
	uuid UUID PRIMARY KEY DEFAULT uuidv7(),
	job_uuid UUID NOT NULL REFERENCES jobs(uuid) ON DELETE CASCADE,
	success BOOL NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	duration INTEGER NOT NULL CHECK (duration > 0),
	additional_info TEXT NOT NULL
);

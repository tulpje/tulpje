-- set all updated_at timestamps to 0 so that all systems will get fresh data
UPDATE pk_systems SET updated_at = TO_TIMESTAMP(0);

-- a view containing uuids of systems that are actively being tracked
-- (used for fronter categories or notifications)
CREATE VIEW
  pk_tracked_systems
AS
SELECT
  uuid
FROM
  pk_systems
WHERE
  uuid IN (
    SELECT
      system_uuid
    FROM
      pk_notify_systems
    INNER JOIN
      guilds
    ON
      guilds.guild_id = pk_notify_systems.guild_id
    WHERE
      guilds.deleted_at IS NULL
    UNION
      SELECT
        system_uuid
      FROM
        pk_guilds
      INNER JOIN
        pk_fronters
      ON
        pk_guilds.guild_id = pk_fronters.guild_id
      INNER JOIN
        guilds
      ON
        pk_guilds.guild_id = guilds.guild_id
      WHERE
        guilds.deleted_at IS NULL
  )

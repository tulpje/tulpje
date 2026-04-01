CREATE TABLE pk_guild_roles (
  id SERIAL PRIMARY KEY,

  guild_id BIGINT NOT NULL REFERENCES pk_guilds(guild_id) ON DELETE CASCADE,
  role_id BIGINT NOT NULL,
  member_uuid UUID NOT NULL,

  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,

  UNIQUE (guild_id, role_id, member_uuid)
);

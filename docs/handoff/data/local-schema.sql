-- Executable SQLite cache reference. NOT an encrypted production client store.
-- Account identifiers remain present as defense in depth even with per-account files.
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
CREATE TABLE IF NOT EXISTS cache_meta (
  key TEXT PRIMARY KEY, value TEXT NOT NULL
);
INSERT OR IGNORE INTO cache_meta(key,value) VALUES ('schema_version','1');
CREATE TABLE IF NOT EXISTS cached_channels (
  account_id TEXT NOT NULL, workspace_id TEXT NOT NULL, channel_id TEXT NOT NULL,
  name TEXT NOT NULL, kind TEXT NOT NULL CHECK(kind IN ('public','private','dm','group_dm')),
  revision INTEGER NOT NULL CHECK(revision > 0), visible INTEGER NOT NULL DEFAULT 1 CHECK(visible IN (0,1)),
  PRIMARY KEY(account_id,workspace_id,channel_id)
);
CREATE TABLE IF NOT EXISTS cached_messages (
  row_id INTEGER PRIMARY KEY,
  account_id TEXT NOT NULL, workspace_id TEXT NOT NULL, channel_id TEXT NOT NULL,
  message_id TEXT NOT NULL, author_id TEXT NOT NULL, thread_root_id TEXT,
  body_json TEXT NOT NULL CHECK(json_valid(body_json)), body_text TEXT NOT NULL,
  revision INTEGER NOT NULL CHECK(revision > 0),
  server_sequence TEXT NOT NULL CHECK(length(server_sequence)>0 AND server_sequence NOT GLOB '*[^0-9]*'),
  created_at TEXT NOT NULL,
  deleted INTEGER NOT NULL DEFAULT 0 CHECK(deleted IN (0,1)),
  CHECK(deleted=0 OR (body_text='' AND body_json='null')),
  UNIQUE(account_id,workspace_id,message_id),
  FOREIGN KEY(account_id,workspace_id,channel_id) REFERENCES cached_channels(account_id,workspace_id,channel_id) ON DELETE CASCADE
);
-- thread_root_id may refer to an uncached root; the server enforces thread tenancy.
CREATE INDEX IF NOT EXISTS cached_messages_channel ON cached_messages(account_id,workspace_id,channel_id,length(server_sequence),server_sequence);
CREATE VIRTUAL TABLE IF NOT EXISTS message_fts USING fts5(
  account_id UNINDEXED, workspace_id UNINDEXED, message_id UNINDEXED, body_text,
  tokenize='unicode61'
);
CREATE TRIGGER IF NOT EXISTS cached_messages_insert AFTER INSERT ON cached_messages WHEN NEW.deleted=0 BEGIN
  INSERT INTO message_fts(rowid,account_id,workspace_id,message_id,body_text)
  VALUES(NEW.row_id,NEW.account_id,NEW.workspace_id,NEW.message_id,NEW.body_text);
END;
CREATE TRIGGER IF NOT EXISTS cached_messages_update AFTER UPDATE ON cached_messages BEGIN
  DELETE FROM message_fts WHERE rowid=OLD.row_id;
  INSERT INTO message_fts(rowid,account_id,workspace_id,message_id,body_text)
    SELECT NEW.row_id,NEW.account_id,NEW.workspace_id,NEW.message_id,NEW.body_text WHERE NEW.deleted=0;
END;
CREATE TRIGGER IF NOT EXISTS cached_messages_delete AFTER DELETE ON cached_messages BEGIN
  DELETE FROM message_fts WHERE rowid=OLD.row_id;
END;
CREATE TABLE IF NOT EXISTS sync_cursors (
  account_id TEXT NOT NULL, workspace_id TEXT NOT NULL, subscription_id TEXT NOT NULL,
  opaque_cursor TEXT NOT NULL, authorization_epoch INTEGER NOT NULL CHECK(authorization_epoch>0),
  updated_at TEXT NOT NULL,
  PRIMARY KEY(account_id,workspace_id,subscription_id)
);
CREATE TABLE IF NOT EXISTS pending_operations (
  account_id TEXT NOT NULL, workspace_id TEXT NOT NULL, operation_id TEXT NOT NULL,
  method TEXT NOT NULL, resource_id TEXT NOT NULL, expected_revision INTEGER,
  request_json TEXT NOT NULL CHECK(json_valid(request_json)), request_digest TEXT NOT NULL,
  state TEXT NOT NULL CHECK(state IN ('pending','sending','accepted','rejected','unknown')),
  created_at TEXT NOT NULL, last_error_code TEXT,
  PRIMARY KEY(account_id,workspace_id,operation_id)
);
CREATE TABLE IF NOT EXISTS drafts (
  account_id TEXT NOT NULL, workspace_id TEXT NOT NULL, channel_id TEXT NOT NULL,
  thread_key TEXT NOT NULL DEFAULT '', body_json TEXT NOT NULL CHECK(json_valid(body_json)), updated_at TEXT NOT NULL,
  PRIMARY KEY(account_id,workspace_id,channel_id,thread_key),
  FOREIGN KEY(account_id,workspace_id,channel_id) REFERENCES cached_channels(account_id,workspace_id,channel_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS cached_tasks (
  account_id TEXT NOT NULL, workspace_id TEXT NOT NULL, task_id TEXT NOT NULL, channel_id TEXT NOT NULL,
  revision INTEGER NOT NULL CHECK(revision>0), state TEXT NOT NULL,
  payload_json TEXT NOT NULL CHECK(json_valid(payload_json)),
  PRIMARY KEY(account_id,workspace_id,task_id),
  FOREIGN KEY(account_id,workspace_id,channel_id) REFERENCES cached_channels(account_id,workspace_id,channel_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS cached_artifact_versions (
  account_id TEXT NOT NULL, workspace_id TEXT NOT NULL, artifact_version_id TEXT NOT NULL,
  task_id TEXT NOT NULL, digest TEXT NOT NULL, protected_local_ref TEXT,
  payload_json TEXT NOT NULL CHECK(json_valid(payload_json)),
  PRIMARY KEY(account_id,workspace_id,artifact_version_id),
  FOREIGN KEY(account_id,workspace_id,task_id) REFERENCES cached_tasks(account_id,workspace_id,task_id) ON DELETE CASCADE
);

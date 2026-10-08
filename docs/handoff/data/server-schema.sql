-- PostgreSQL reference design, not executed in the handoff preparation environment.
-- Apply only to a disposable development database after review.
-- Includes tenant RLS defense in depth; NOT full resource authorization or identity implementation.
-- Runtime DB role must not be owner/superuser/BYPASSRLS. SET LOCAL context only after verified auth.
BEGIN;
CREATE TABLE workspaces (
  id text PRIMARY KEY, name text NOT NULL, home_region text NOT NULL,
  policy_revision integer NOT NULL DEFAULT 1 CHECK(policy_revision>0), created_at timestamptz NOT NULL DEFAULT now()
);
CREATE TABLE principals (
  workspace_id text NOT NULL REFERENCES workspaces(id), id text NOT NULL,
  kind text NOT NULL CHECK(kind IN ('human','agent','integration','system')),
  display_name text NOT NULL, status text NOT NULL CHECK(status IN ('active','deactivated')),
  identity_subject_ref text, accountable_human_id text,
  PRIMARY KEY(workspace_id,id),
  CHECK(kind<>'agent' OR accountable_human_id IS NOT NULL),
  FOREIGN KEY(workspace_id,accountable_human_id) REFERENCES principals(workspace_id,id)
);
-- Verify accountable_human_id refers to an active human in application/constraint trigger logic.
CREATE TABLE workspace_memberships (
  workspace_id text NOT NULL, principal_id text NOT NULL,
  role text NOT NULL CHECK(role IN ('owner','admin','member','guest','agent')),
  state text NOT NULL CHECK(state IN ('active','revoked')),
  revision integer NOT NULL DEFAULT 1 CHECK(revision>0),
  PRIMARY KEY(workspace_id,principal_id), FOREIGN KEY(workspace_id,principal_id) REFERENCES principals(workspace_id,id)
);
CREATE TABLE channels (
  workspace_id text NOT NULL REFERENCES workspaces(id), id text NOT NULL,
  kind text NOT NULL CHECK(kind IN ('public','private','dm','group_dm')),
  name text NOT NULL, revision integer NOT NULL DEFAULT 1 CHECK(revision>0), archived boolean NOT NULL DEFAULT false,
  PRIMARY KEY(workspace_id,id)
);
CREATE TABLE channel_memberships (
  workspace_id text NOT NULL, channel_id text NOT NULL, principal_id text NOT NULL,
  can_post boolean NOT NULL DEFAULT true, authorization_epoch integer NOT NULL DEFAULT 1 CHECK(authorization_epoch>0),
  PRIMARY KEY(workspace_id,channel_id,principal_id),
  FOREIGN KEY(workspace_id,channel_id) REFERENCES channels(workspace_id,id),
  FOREIGN KEY(workspace_id,principal_id) REFERENCES principals(workspace_id,id)
);
CREATE TABLE projects (
  workspace_id text NOT NULL, id text NOT NULL, name text NOT NULL, channel_id text NOT NULL,
  PRIMARY KEY(workspace_id,id), FOREIGN KEY(workspace_id,channel_id) REFERENCES channels(workspace_id,id)
);
CREATE TABLE messages (
  workspace_id text NOT NULL, id text NOT NULL, channel_id text NOT NULL, author_id text NOT NULL,
  thread_root_id text, body jsonb, revision integer NOT NULL DEFAULT 1 CHECK(revision>0),
  sequence numeric(20,0) NOT NULL CHECK(sequence>=0),
  created_at timestamptz NOT NULL DEFAULT now(), edited_at timestamptz, deleted_at timestamptz,
  PRIMARY KEY(workspace_id,id), UNIQUE(workspace_id,channel_id,id),
  FOREIGN KEY(workspace_id,channel_id) REFERENCES channels(workspace_id,id),
  FOREIGN KEY(workspace_id,author_id) REFERENCES principals(workspace_id,id),
  FOREIGN KEY(workspace_id,channel_id,thread_root_id) REFERENCES messages(workspace_id,channel_id,id),
  CHECK((deleted_at IS NULL AND body IS NOT NULL) OR (deleted_at IS NOT NULL AND body IS NULL))
);
CREATE INDEX messages_timeline ON messages(workspace_id,channel_id,sequence);
CREATE TABLE reactions (
  workspace_id text NOT NULL, message_id text NOT NULL, principal_id text NOT NULL, reaction_key text NOT NULL,
  PRIMARY KEY(workspace_id,message_id,principal_id,reaction_key),
  FOREIGN KEY(workspace_id,message_id) REFERENCES messages(workspace_id,id),
  FOREIGN KEY(workspace_id,principal_id) REFERENCES principals(workspace_id,id)
);
CREATE TABLE read_states (
  workspace_id text NOT NULL, principal_id text NOT NULL, channel_id text NOT NULL,
  last_read_sequence numeric(20,0) NOT NULL DEFAULT 0 CHECK(last_read_sequence>=0), mark_unread_message_id text,
  PRIMARY KEY(workspace_id,principal_id,channel_id),
  FOREIGN KEY(workspace_id,principal_id) REFERENCES principals(workspace_id,id),
  FOREIGN KEY(workspace_id,channel_id) REFERENCES channels(workspace_id,id)
);
CREATE TABLE tasks (
  workspace_id text NOT NULL, id text NOT NULL, channel_id text NOT NULL, owner_id text NOT NULL,
  title text NOT NULL, state text NOT NULL CHECK(state IN ('draft','ready','active','blocked','awaiting_review','accepted','cancelled')),
  revision integer NOT NULL DEFAULT 1 CHECK(revision>0), contract_revision integer CHECK(contract_revision>0),
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY(workspace_id,id), FOREIGN KEY(workspace_id,channel_id) REFERENCES channels(workspace_id,id),
  FOREIGN KEY(workspace_id,owner_id) REFERENCES principals(workspace_id,id),
  CHECK(state IN ('draft','cancelled') OR contract_revision IS NOT NULL)
);
CREATE TABLE contract_revisions (
  workspace_id text NOT NULL, task_id text NOT NULL, revision integer NOT NULL CHECK(revision>0),
  contract jsonb NOT NULL, digest text NOT NULL, created_by text NOT NULL, created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY(workspace_id,task_id,revision), FOREIGN KEY(workspace_id,task_id) REFERENCES tasks(workspace_id,id),
  FOREIGN KEY(workspace_id,created_by) REFERENCES principals(workspace_id,id)
);
ALTER TABLE tasks ADD CONSTRAINT task_current_contract FOREIGN KEY(workspace_id,id,contract_revision)
  REFERENCES contract_revisions(workspace_id,task_id,revision) DEFERRABLE INITIALLY DEFERRED;
CREATE TABLE environments (
  workspace_id text NOT NULL, id text NOT NULL, location text NOT NULL CHECK(location IN ('local','hosted','company_managed')),
  owner_id text NOT NULL, isolation_profile text NOT NULL, definition_ref text NOT NULL,
  PRIMARY KEY(workspace_id,id), FOREIGN KEY(workspace_id,owner_id) REFERENCES principals(workspace_id,id)
);
CREATE TABLE runs (
  workspace_id text NOT NULL, id text NOT NULL, task_id text NOT NULL, contract_revision integer NOT NULL,
  agent_id text NOT NULL, environment_id text NOT NULL, model_destination text NOT NULL,
  state text NOT NULL CHECK(state IN ('queued','preparing','running','awaiting_approval','paused','reconciling','cancelling','succeeded','failed','cancelled')),
  revision integer NOT NULL DEFAULT 1 CHECK(revision>0), generation integer NOT NULL DEFAULT 1 CHECK(generation>0),
  cancellation_requested boolean NOT NULL DEFAULT false, authority_kind text NOT NULL CHECK(authority_kind IN ('local_private','cloud_shared')),
  PRIMARY KEY(workspace_id,id), UNIQUE(workspace_id,id,task_id,contract_revision),
  FOREIGN KEY(workspace_id,task_id,contract_revision) REFERENCES contract_revisions(workspace_id,task_id,revision),
  FOREIGN KEY(workspace_id,agent_id) REFERENCES principals(workspace_id,id),
  FOREIGN KEY(workspace_id,environment_id) REFERENCES environments(workspace_id,id)
);
CREATE TABLE run_leases (
  workspace_id text NOT NULL, run_id text NOT NULL, coordinator_id text NOT NULL,
  generation integer NOT NULL CHECK(generation>0), expires_at timestamptz NOT NULL,
  PRIMARY KEY(workspace_id,run_id), FOREIGN KEY(workspace_id,run_id) REFERENCES runs(workspace_id,id)
);
CREATE TABLE artifacts (
  workspace_id text NOT NULL, id text NOT NULL, task_id text NOT NULL, current_version integer NOT NULL DEFAULT 0 CHECK(current_version>=0),
  PRIMARY KEY(workspace_id,id), UNIQUE(workspace_id,id,task_id), FOREIGN KEY(workspace_id,task_id) REFERENCES tasks(workspace_id,id)
);
CREATE TABLE artifact_versions (
  workspace_id text NOT NULL, id text NOT NULL, artifact_id text NOT NULL, task_id text NOT NULL, run_id text NOT NULL,
  version integer NOT NULL CHECK(version>0), kind text NOT NULL, digest text NOT NULL,
  blob_ref text NOT NULL, mime_type text NOT NULL, size_bytes numeric(20,0) NOT NULL CHECK(size_bytes>=0),
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY(workspace_id,id), UNIQUE(workspace_id,artifact_id,version),
  FOREIGN KEY(workspace_id,artifact_id,task_id) REFERENCES artifacts(workspace_id,id,task_id),
  FOREIGN KEY(workspace_id,run_id) REFERENCES runs(workspace_id,id)
);
-- Application must verify the run belongs to the same task before upload finalization.
CREATE TABLE artifact_audiences (
  workspace_id text NOT NULL, artifact_version_id text NOT NULL, channel_id text NOT NULL,
  PRIMARY KEY(workspace_id,artifact_version_id,channel_id),
  FOREIGN KEY(workspace_id,artifact_version_id) REFERENCES artifact_versions(workspace_id,id),
  FOREIGN KEY(workspace_id,channel_id) REFERENCES channels(workspace_id,id)
);
CREATE TABLE action_intents (
  workspace_id text NOT NULL, id text NOT NULL, task_id text NOT NULL, run_id text NOT NULL,
  contract_revision integer NOT NULL, generation integer NOT NULL CHECK(generation>0),
  kind text NOT NULL, semantic_payload jsonb NOT NULL, digest text NOT NULL,
  policy_revision integer NOT NULL CHECK(policy_revision>0), artifact_version_id text,
  status text NOT NULL CHECK(status IN ('proposed','awaiting_approval','ready','dispatching','succeeded','failed','unknown','cancelled')),
  requires_approval boolean NOT NULL, expires_at timestamptz NOT NULL, provider_operation_ref text,
  PRIMARY KEY(workspace_id,id),
  FOREIGN KEY(workspace_id,run_id,task_id,contract_revision) REFERENCES runs(workspace_id,id,task_id,contract_revision),
  FOREIGN KEY(workspace_id,artifact_version_id) REFERENCES artifact_versions(workspace_id,id)
);
CREATE TABLE approval_decisions (
  workspace_id text NOT NULL, id text NOT NULL, action_id text NOT NULL, action_digest text NOT NULL,
  artifact_version_id text, contract_revision integer NOT NULL, policy_revision integer NOT NULL,
  decider_id text NOT NULL, decision text NOT NULL CHECK(decision IN ('approved','rejected')),
  expires_at timestamptz NOT NULL, consumed_at timestamptz, recorded_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY(workspace_id,id),
  FOREIGN KEY(workspace_id,action_id) REFERENCES action_intents(workspace_id,id),
  FOREIGN KEY(workspace_id,decider_id) REFERENCES principals(workspace_id,id),
  FOREIGN KEY(workspace_id,artifact_version_id) REFERENCES artifact_versions(workspace_id,id)
);
-- A human-only/check-quorum constraint requires verified application logic and transactional consumption.
CREATE TABLE evidence_records (
  workspace_id text NOT NULL, id text NOT NULL, task_id text NOT NULL, artifact_version_id text NOT NULL,
  criterion_id text NOT NULL, kind text NOT NULL, result text NOT NULL CHECK(result IN ('passed','failed','not_run','inconclusive')),
  protected_payload_ref text NOT NULL, recorded_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY(workspace_id,id), FOREIGN KEY(workspace_id,task_id) REFERENCES tasks(workspace_id,id),
  FOREIGN KEY(workspace_id,artifact_version_id) REFERENCES artifact_versions(workspace_id,id)
);
CREATE TABLE acceptance_records (
  workspace_id text NOT NULL, id text NOT NULL, task_id text NOT NULL, contract_revision integer NOT NULL,
  reviewer_id text NOT NULL, created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY(workspace_id,id),
  FOREIGN KEY(workspace_id,task_id,contract_revision) REFERENCES contract_revisions(workspace_id,task_id,revision),
  FOREIGN KEY(workspace_id,reviewer_id) REFERENCES principals(workspace_id,id)
);
CREATE TABLE acceptance_artifacts (
  workspace_id text NOT NULL, acceptance_id text NOT NULL, artifact_version_id text NOT NULL,
  PRIMARY KEY(workspace_id,acceptance_id,artifact_version_id),
  FOREIGN KEY(workspace_id,acceptance_id) REFERENCES acceptance_records(workspace_id,id),
  FOREIGN KEY(workspace_id,artifact_version_id) REFERENCES artifact_versions(workspace_id,id)
);
CREATE TABLE context_packages (
  workspace_id text NOT NULL, id text NOT NULL, run_id text NOT NULL, manifest jsonb NOT NULL,
  authorization_epoch integer NOT NULL CHECK(authorization_epoch>0), invalidated_at timestamptz,
  PRIMARY KEY(workspace_id,id), FOREIGN KEY(workspace_id,run_id) REFERENCES runs(workspace_id,id)
);
CREATE TABLE checkpoints (
  workspace_id text NOT NULL, id text NOT NULL, run_id text NOT NULL, generation integer NOT NULL CHECK(generation>0),
  manifest jsonb NOT NULL, digest text NOT NULL, verified boolean NOT NULL DEFAULT false,
  PRIMARY KEY(workspace_id,id), FOREIGN KEY(workspace_id,run_id) REFERENCES runs(workspace_id,id)
);
CREATE TABLE budget_accounts (
  workspace_id text NOT NULL, id text NOT NULL, currency text NOT NULL DEFAULT 'USD' CHECK(currency='USD'),
  limit_micros numeric(20,0) NOT NULL CHECK(limit_micros>=0), spent_micros numeric(20,0) NOT NULL DEFAULT 0 CHECK(spent_micros>=0),
  reserved_micros numeric(20,0) NOT NULL DEFAULT 0 CHECK(reserved_micros>=0),
  PRIMARY KEY(workspace_id,id), FOREIGN KEY(workspace_id) REFERENCES workspaces(id)
);
CREATE TABLE usage_reservations (
  workspace_id text NOT NULL, id text NOT NULL, budget_id text NOT NULL, run_id text NOT NULL,
  amount_micros numeric(20,0) NOT NULL CHECK(amount_micros>=0), state text NOT NULL CHECK(state IN ('reserved','settled','released')),
  PRIMARY KEY(workspace_id,id), FOREIGN KEY(workspace_id,budget_id) REFERENCES budget_accounts(workspace_id,id),
  FOREIGN KEY(workspace_id,run_id) REFERENCES runs(workspace_id,id)
);
CREATE TABLE usage_charges (
  workspace_id text NOT NULL, id text NOT NULL, reservation_id text NOT NULL,
  provider_charge_key text NOT NULL, actual_micros numeric(20,0) NOT NULL CHECK(actual_micros>=0), rate_card_ref text NOT NULL,
  PRIMARY KEY(workspace_id,id), UNIQUE(workspace_id,provider_charge_key),
  FOREIGN KEY(workspace_id,reservation_id) REFERENCES usage_reservations(workspace_id,id)
);
CREATE TABLE idempotency_records (
  workspace_id text NOT NULL, actor_id text NOT NULL, operation text NOT NULL, key text NOT NULL,
  request_digest text NOT NULL, status text NOT NULL CHECK(status IN ('pending','complete','unknown')),
  owner_token text NOT NULL, response_ref text, expires_at timestamptz NOT NULL,
  PRIMARY KEY(workspace_id,actor_id,operation,key), FOREIGN KEY(workspace_id,actor_id) REFERENCES principals(workspace_id,id)
);
CREATE TABLE workspace_event_counters (
  workspace_id text PRIMARY KEY REFERENCES workspaces(id), last_sequence numeric(20,0) NOT NULL DEFAULT 0 CHECK(last_sequence>=0)
);
CREATE TABLE outbox_events (
  workspace_id text NOT NULL, id text NOT NULL, sequence numeric(20,0) NOT NULL CHECK(sequence>0),
  type text NOT NULL, schema_version integer NOT NULL CHECK(schema_version>0),
  aggregate_id text NOT NULL, aggregate_revision integer NOT NULL CHECK(aggregate_revision>0),
  payload jsonb NOT NULL, occurred_at timestamptz NOT NULL DEFAULT now(), published_at timestamptz,
  PRIMARY KEY(workspace_id,id), UNIQUE(workspace_id,sequence), FOREIGN KEY(workspace_id) REFERENCES workspaces(id)
);
CREATE INDEX outbox_unpublished ON outbox_events(workspace_id,sequence) WHERE published_at IS NULL;
CREATE TABLE integrations (
  workspace_id text NOT NULL, id text NOT NULL, provider text NOT NULL, installed_by text NOT NULL,
  secret_handle text NOT NULL, scopes jsonb NOT NULL, revoked_at timestamptz,
  PRIMARY KEY(workspace_id,id), FOREIGN KEY(workspace_id,installed_by) REFERENCES principals(workspace_id,id)
);
CREATE TABLE audit_events (
  workspace_id text NOT NULL, id text NOT NULL, actor_id text NOT NULL, action text NOT NULL,
  resource_id text NOT NULL, safe_metadata jsonb NOT NULL, occurred_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY(workspace_id,id), FOREIGN KEY(workspace_id,actor_id) REFERENCES principals(workspace_id,id)
);
-- Tenant-only RLS. A separate resource-authorization layer is MANDATORY for channels, artifacts, and context.
ALTER TABLE workspaces ENABLE ROW LEVEL SECURITY;
ALTER TABLE workspaces FORCE ROW LEVEL SECURITY;
CREATE POLICY workspace_scope ON workspaces USING(id=current_setting('app.workspace_id',true)) WITH CHECK(id=current_setting('app.workspace_id',true));
DO $$
DECLARE t text;
BEGIN
  FOREACH t IN ARRAY ARRAY['principals','workspace_memberships','channels','channel_memberships','projects','messages','reactions','read_states','tasks','contract_revisions','environments','runs','run_leases','artifacts','artifact_versions','artifact_audiences','action_intents','approval_decisions','evidence_records','acceptance_records','acceptance_artifacts','context_packages','checkpoints','budget_accounts','usage_reservations','usage_charges','idempotency_records','workspace_event_counters','outbox_events','integrations','audit_events'] LOOP
    EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY',t);
    EXECUTE format('ALTER TABLE %I FORCE ROW LEVEL SECURITY',t);
    EXECUTE format('CREATE POLICY tenant_scope ON %I USING (workspace_id=current_setting(''app.workspace_id'',true)) WITH CHECK (workspace_id=current_setting(''app.workspace_id'',true))',t);
  END LOOP;
END $$;
COMMIT;

//! Account/workspace-scoped cache reference, not encryption or authentication.
use rusqlite::{params, Connection, Result};
const SCHEMA: &str = include_str!("../../../docs/handoff/data/local-schema.sql");
pub struct Cache {
    connection: Connection,
}
impl Cache {
    pub fn in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;
        connection.execute_batch(SCHEMA)?;
        Ok(Self { connection })
    }
    pub fn add_channel(&self, account: &str, workspace: &str, channel: &str) -> Result<()> {
        self.connection.execute("INSERT OR IGNORE INTO cached_channels(account_id,workspace_id,channel_id,name,kind,revision) VALUES(?1,?2,?3,'demo','public',1)",params![account,workspace,channel])?;
        Ok(())
    }
    /// Fixture-only. Validate real ingestion against versioned wire contracts.
    pub fn add_demo_message(
        &self,
        account: &str,
        workspace: &str,
        channel: &str,
        id: &str,
        text: &str,
    ) -> Result<()> {
        self.connection.execute("INSERT INTO cached_messages(account_id,workspace_id,channel_id,message_id,author_id,body_json,body_text,revision,server_sequence,created_at) VALUES(?1,?2,?3,?4,'human_demo','{}',?5,1,'1','2026-10-08T00:00:00Z')",params![account,workspace,channel,id,text])?;
        Ok(())
    }
    pub fn search(&self, account: &str, workspace: &str, query: &str) -> Result<Vec<String>> {
        let mut statement=self.connection.prepare("SELECT message_id FROM message_fts WHERE message_fts MATCH ?1 AND account_id=?2 AND workspace_id=?3 ORDER BY rank LIMIT 50")?;
        let rows = statement.query_map(params![query, account, workspace], |row| row.get(0))?;
        rows.collect()
    }
    pub fn delete_message(&self, account: &str, workspace: &str, id: &str) -> Result<()> {
        self.connection.execute(
            "DELETE FROM cached_messages WHERE account_id=?1 AND workspace_id=?2 AND message_id=?3",
            params![account, workspace, id],
        )?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scoped_search() -> Result<()> {
        let c = Cache::in_memory()?;
        for (a, w, id) in [
            ("a", "w", "one"),
            ("b", "w", "two"),
            ("a", "other", "three"),
        ] {
            c.add_channel(a, w, "c")?;
            c.add_demo_message(a, w, "c", id, "checkout failed")?;
        }
        assert_eq!(c.search("a", "w", "checkout")?, vec!["one"]);
        Ok(())
    }
    #[test]
    fn deletion_updates_search() -> Result<()> {
        let c = Cache::in_memory()?;
        c.add_channel("a", "w", "c")?;
        c.add_demo_message("a", "w", "c", "m", "checkout")?;
        c.delete_message("a", "w", "m")?;
        assert!(c.search("a", "w", "checkout")?.is_empty());
        Ok(())
    }
    #[test]
    fn wrong_account_cannot_delete() -> Result<()> {
        let c = Cache::in_memory()?;
        c.add_channel("a", "w", "c")?;
        c.add_demo_message("a", "w", "c", "m", "checkout")?;
        c.delete_message("b", "w", "m")?;
        assert_eq!(c.search("a", "w", "checkout")?, vec!["m"]);
        Ok(())
    }
}

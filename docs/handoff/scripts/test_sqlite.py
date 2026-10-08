#!/usr/bin/env python3
"""Run cache/FTS reference tests. Does not test encryption or production synchronization."""
from pathlib import Path
import sqlite3, unittest, json
ROOT=Path(__file__).resolve().parents[1]
SQL=(ROOT/'data/local-schema.sql').read_text()
class CacheTests(unittest.TestCase):
    def setUp(self):
        self.db=sqlite3.connect(':memory:'); self.db.executescript(SQL)
        for a,w,c in [('acct_a','ws_alpha','chn_one'),('acct_a','ws_beta','chn_one'),('acct_b','ws_alpha','chn_one')]:
            self.db.execute('INSERT INTO cached_channels VALUES(?,?,?,?,?,?,?)',(a,w,c,'General','private',1,1))
        self.db.commit()
    def tearDown(self): self.db.close()
    def add(self,mid='msg_first',text='checkout regression',a='acct_a',w='ws_alpha',seq='1'):
        self.db.execute('INSERT INTO cached_messages(account_id,workspace_id,channel_id,message_id,author_id,body_json,body_text,revision,server_sequence,created_at) VALUES(?,?,?,?,?,?,?,?,?,?)',(a,w,'chn_one',mid,'usr_owner',json.dumps({'text':text}),text,1,seq,'2026-10-08T15:00:00Z'))
    def search(self,q='checkout',a='acct_a',w='ws_alpha'):
        return self.db.execute("""SELECT m.message_id FROM message_fts f JOIN cached_messages m ON m.row_id=f.rowid
         JOIN cached_channels c ON c.account_id=m.account_id AND c.workspace_id=m.workspace_id AND c.channel_id=m.channel_id
         WHERE message_fts MATCH ? AND m.account_id=? AND m.workspace_id=? AND c.visible=1 AND m.deleted=0""",(q,a,w)).fetchall()
    def test_initial_index(self):
        self.add(); self.assertEqual(self.search(),[('msg_first',)])
    def test_update_removes_old_terms(self):
        self.add();self.db.execute("UPDATE cached_messages SET body_text='payment wording',body_json='{}',revision=2")
        self.assertEqual(self.search(),[]);self.assertEqual(self.search('payment'),[('msg_first',)])
    def test_tombstone_removes_fts(self):
        self.add();self.db.execute("UPDATE cached_messages SET deleted=1,body_text='',body_json='null',revision=2")
        self.assertEqual(self.search(),[]);self.assertEqual(self.db.execute('SELECT count(*) FROM message_fts').fetchone()[0],0)
    def test_physical_delete_removes_fts(self):
        self.add();self.db.execute('DELETE FROM cached_messages');self.assertEqual(self.search(),[])
    def test_workspace_isolation(self):
        self.add();self.add(mid='msg_other',w='ws_beta');self.assertEqual(self.search(),[('msg_first',)])
    def test_account_isolation(self):
        self.add();self.add(mid='msg_other',a='acct_b');self.assertEqual(self.search(),[('msg_first',)])
    def test_revoked_channel_hidden_before_purge(self):
        self.add();self.db.execute("UPDATE cached_channels SET visible=0 WHERE account_id='acct_a' AND workspace_id='ws_alpha'");self.assertEqual(self.search(),[])
    def test_purge_cascades_into_index(self):
        self.add();self.db.execute("DELETE FROM cached_channels WHERE account_id='acct_a' AND workspace_id='ws_alpha'")
        self.assertEqual(self.db.execute('SELECT count(*) FROM cached_messages').fetchone()[0],0)
        self.assertEqual(self.db.execute('SELECT count(*) FROM message_fts').fetchone()[0],0)
    def test_duplicate_message_rejected(self):
        self.add()
        with self.assertRaises(sqlite3.IntegrityError):self.add()
    def test_unknown_channel_rejected(self):
        with self.assertRaises(sqlite3.IntegrityError):self.add(w='ws_unknown')
    def test_index_and_row_rollback_together(self):
        self.db.execute('BEGIN');self.add();self.db.rollback();self.assertEqual(self.search(),[])
    def test_draft_survives_ordinary_query(self):
        self.db.execute('INSERT INTO drafts VALUES(?,?,?,?,?,?)',('acct_a','ws_alpha','chn_one','','{"text":"draft"}','2026-10-08T15:00:00Z'))
        self.search();self.assertEqual(self.db.execute('SELECT count(*) FROM drafts').fetchone()[0],1)
    def test_invalid_json_rejected(self):
        with self.assertRaises(sqlite3.IntegrityError):self.db.execute('INSERT INTO drafts VALUES(?,?,?,?,?,?)',('acct_a','ws_alpha','chn_one','','invalid','2026-10-08T15:00:00Z'))
    def test_deleted_row_cannot_keep_source_text(self):
        self.add()
        with self.assertRaises(sqlite3.IntegrityError):self.db.execute('UPDATE cached_messages SET deleted=1')
    def test_decimal_sequence_sort_without_float(self):
        self.add(mid='msg_big01',seq='9007199254740993');self.add(mid='msg_big02',seq='9007199254740992')
        rows=self.db.execute('SELECT message_id FROM cached_messages ORDER BY length(server_sequence),server_sequence').fetchall()
        self.assertEqual(rows,[('msg_big02',),('msg_big01',)])
    def test_pending_operation_id_is_unique(self):
        row=('acct_a','ws_alpha','op_first','message.send','chn_one',None,'{}','synthetic-digest','pending','2026-10-08T15:00:00Z',None)
        self.db.execute('INSERT INTO pending_operations VALUES(?,?,?,?,?,?,?,?,?,?,?)',row)
        with self.assertRaises(sqlite3.IntegrityError):self.db.execute('INSERT INTO pending_operations VALUES(?,?,?,?,?,?,?,?,?,?,?)',row)
if __name__=='__main__':unittest.main(verbosity=2)

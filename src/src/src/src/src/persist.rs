use rusqlite::{params, Connection};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersistError {
    #[error("db error: {0}")]
    Db(#[from] rusqlite::Error),
}

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn open(path: &str) -> Result<Self, PersistError> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS records (
                id TEXT PRIMARY KEY,
                sequence INTEGER NOT NULL,
                digest TEXT NOT NULL,
                payload BLOB NOT NULL,
                signature BLOB,
                applied INTEGER DEFAULT 0
            );",
        )?;
        Ok(Self { conn })
    }

    pub fn upsert(
        &self,
        id: &str,
        sequence: i64,
        digest: &str,
        payload: &[u8],
        signature: Option<&[u8]>,
    ) -> Result<(), PersistError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO records (id, sequence, digest, payload, signature, applied)
             VALUES (?1, ?2, ?3, ?4, ?5, 0)",
            params![id, sequence, digest, payload, signature],
        )?;
        Ok(())
    }

    pub fn mark_applied(&self, id: &str) -> Result<(), PersistError> {
        self.conn
            .execute("UPDATE records SET applied=1 WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn list_pending(
        &self,
    ) -> Result<Vec<(String, i64, String, Vec<u8>, Option<Vec<u8>>)>, PersistError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, sequence, digest, payload, signature
             FROM records WHERE applied=0 ORDER BY sequence ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Vec<u8>>(3)?,
                row.get::<_, Option<Vec<u8>>>(4)?,
            ))
        })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }
                     }

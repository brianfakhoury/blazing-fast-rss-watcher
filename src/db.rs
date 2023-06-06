use crate::model::ArticleInfo;
use rusqlite::{params, Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS items (
                 title TEXT NOT NULL,
                 description TEXT,
                 link TEXT NOT NULL UNIQUE
             )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn insert_item(&mut self, item: &ArticleInfo) -> Result<usize> {
        self.conn.execute(
            "INSERT INTO items (title, description, link) VALUES (?, ?, ?)",
            params![&item.title, &item.description, &item.link],
        )
    }
}

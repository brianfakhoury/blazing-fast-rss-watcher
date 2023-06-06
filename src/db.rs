use crate::model::MyItem;
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
                 link TEXT NOT NULL UNIQUE
             )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn insert_item(&mut self, item: &MyItem) -> Result<usize> {
        self.conn.execute(
            "INSERT INTO items (title, link) VALUES (?, ?)",
            params![&item.title, &item.link],
        )
    }
}

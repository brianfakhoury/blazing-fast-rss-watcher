use crate::model::Article;
use rusqlite::{params, Connection, Result};
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
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
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn insert_item(&self, item: &Article) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO items (title, link) VALUES (?, ?)",
            params![&item.title, &item.link],
        )
    }
}

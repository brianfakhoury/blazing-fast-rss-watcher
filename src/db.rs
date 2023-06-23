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

#[cfg(test)]
mod tests {
    use crate::db::Database;
    use crate::model::Article;
    use tempfile::tempdir;

    #[test]
    fn test_new_database() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let db_file = dir.path().join("test.db");
        let _db = Database::new(db_file.to_str().unwrap())?;
        // if no error is thrown, then database initialization passed
        assert!(db_file.exists());
        Ok(())
    }

    #[test]
    fn test_insert_item() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let db_file = dir.path().join("test2.db");
        let db = Database::new(db_file.to_str().unwrap())?;
        let article = Article {
            title: "Test Article".to_string(),
            description: Some("".into()),
            link: "https://example.com".to_string(),
        };
        let rows = db.insert_item(&article)?;
        // if no error is thrown, then item insertion passed
        assert_eq!(rows, 1);
        Ok(())
    }
}

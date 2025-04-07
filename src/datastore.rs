use rusqlite::{Connection, Result, Row};

pub struct Datastore {
    pub conn: Connection,
}

impl Datastore {
    pub fn new(conn: Connection) -> Result<Self> {
        let ds = Datastore { conn };
        ds.init()?;
        Ok(ds)
    }

    fn init(&self) -> Result<()> {
        self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                idx INTEGER NOT NULL UNIQUE,
                balance INTEGER NOT NULL
            );
            "#,
            [],
        )?;

        self.conn.execute(
            r#"
            INSERT OR IGNORE INTO users (id, idx, balance)
            VALUES (1, 0, 1111),
                   (2, 1, 2222),
                   (3, 2, 3333),
                   (4, 3, 4444),
                   (5, 4, 5555),
                   (6, 5, 6666),
                   (7, 6, 7777),
                   (8, 7, 8888);
            "#,
            [],
        )?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub idx: u32,
    pub balance: u32,
}

impl User {
    fn from_row(row: &Row) -> Result<Self> {
        Ok(User {
            id: row.get("id")?,
            idx: row.get("idx")?,
            balance: row.get("balance")?,
        })
    }

    pub fn all(conn: &Connection) -> Result<Vec<User>> {
        let mut stmt = conn.prepare("SELECT id, idx, balance FROM users ORDER BY idx ASC")?;
        let user_iter = stmt.query_map([], |row| User::from_row(row))?;

        let mut users = Vec::new();
        for user in user_iter {
            users.push(user?);
        }

        Ok(users)
    }

    pub fn get_by_id(conn: &Connection, id: u32) -> Result<User> {
        let mut stmt = conn.prepare("SELECT id, idx, balance FROM users WHERE id = ?")?;
        let user_iter = stmt.query_map([id], |row| User::from_row(row))?;

        for user in user_iter {
            return Ok(user?);
        }

        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

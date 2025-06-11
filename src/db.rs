// filepath: /netpulse/netpulse/src/db.rs
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{ToSql};

struct DbConnection {
    pub conn: PooledConnection<SqliteConnectionManager>,
}

impl DbConnection {
    pub fn new(pool: &Pool<SqliteConnectionManager>) -> Result<Self, r2d2::Error> {
        let conn = pool.get()?;
        Ok(DbConnection { conn })
    }

    pub fn execute(&self, query: &str, params: &[&dyn ToSql]) -> Result<usize, rusqlite::Error> {
        self.conn.execute(query, params)
    }

    pub fn query<T, F>(&self, query: &str, params: &[&dyn ToSql], map_fn: F) -> Result<Vec<T>, rusqlite::Error>
    where
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
    {
        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map(params, map_fn)?;
        rows.collect()
    }
}
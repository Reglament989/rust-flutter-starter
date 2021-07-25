use lazy_static::lazy_static;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;

pub struct State {
  pub pool: r2d2::Pool<SqliteConnectionManager>,
}

lazy_static! {
  pub static ref STATE: State = {
    State {
      pool: r2d2::Pool::new(SqliteConnectionManager::file("file.db")).unwrap(),
    }
  };
}

pub fn init() -> Result<()> {
  let conn: r2d2::PooledConnection<SqliteConnectionManager> = STATE.pool.get().unwrap();
  conn.execute(
    "CREATE TABLE users (
              id              INTEGER PRIMARY KEY,
              name            TEXT NOT NULL,
              )",
    [],
  )?;
  Ok(())
}

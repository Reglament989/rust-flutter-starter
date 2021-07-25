use super::ffi::ExternUser;
use crate::db::init::STATE;
use rusqlite::{params, Result};
use std::ffi::CString;

#[derive(Debug)]
struct User {
  id: i32,
  name: String,
}

pub fn create_user(name: &str) -> Result<usize> {
  let me = User {
    id: 0,
    name: name.to_owned(),
  };
  let conn = STATE.pool.get().unwrap();
  conn.execute("INSERT INTO users (name) VALUES (?1)", params![me.name])
}

pub fn get_all_users() -> Result<Vec<ExternUser>> {
  let mut extern_users: Vec<ExternUser> = vec![];
  let conn = STATE.pool.get().unwrap();

  let mut stmt = conn.prepare("SELECT * FROM users").unwrap();
  let users_iter = stmt
    .query_map([], |row| {
      Ok(User {
        id: row.get(0)?,
        name: row.get(1)?,
      })
    })
    .unwrap();
  for user in users_iter {
    let user = user.unwrap();
    extern_users.push(ExternUser {
      id: user.id,
      name: CString::new(user.name).unwrap().into_raw(),
    })
  }

  Ok(extern_users)
}

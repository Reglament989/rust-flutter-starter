use std::ffi::CStr;
use std::os::raw::c_char;

use crate::db::init::init;
use crate::db::users::{create_user, get_all_users};

#[derive(Debug)]
#[repr(C)]
pub struct ExternUser {
  pub id: i32,
  pub name: *mut c_char,
}

#[no_mangle]
pub extern "C" fn database_init() {
  init().unwrap();
}

#[no_mangle]
pub extern "C" fn database_create_user(name: *const c_char) {
  let c_str = unsafe { CStr::from_ptr(name) };
  let user_name = match c_str.to_str() {
    Err(_) => "New user",
    Ok(string) => string,
  };

  create_user(user_name).unwrap();
}

#[no_mangle]
pub extern "C" fn database_fetch_users() -> Box<Vec<ExternUser>> {
  Box::new(get_all_users().unwrap())
}

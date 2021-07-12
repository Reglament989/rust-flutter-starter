pub mod http;
use std::os::raw::c_char;
use std::ffi::{CString};


#[repr(C)]
pub struct ExternTodo {
    pub user_id: i8,
    pub id: i8,
    pub title: *mut c_char,
    pub completed: bool
}

#[no_mangle]
pub extern "C" fn get_one_todo_ffi() -> ExternTodo {
    let resp = http::get_one_todo().unwrap();
    ExternTodo {
        user_id: resp.user_id,
        id: resp.id,
        title: CString::new(resp.title.to_owned()).unwrap().into_raw(),
        completed: resp.completed
    }
}
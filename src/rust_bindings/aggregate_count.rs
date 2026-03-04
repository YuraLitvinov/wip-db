//!
//! These functions are deprecated.  In order to maintain
//! backwards compatibility with older code, these functions continue
//! to be supported.  However, new applications should avoid
//! the use of these functions.  To encourage programmers to avoid
//! these functions, we will not explain what they do.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//!
use std::os::raw::*;

pub type Sqlite3Stmt = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_expired(stmt: *mut sqlite3_stmt) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_transfer_bindings(stmt: *mut sqlite3_stmt, stmt2: *mut sqlite3_stmt) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_global_recover() -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_thread_cleanup() {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_memory_alarm(db: UnknownCallback, ptr: *mut c_void, db2: sqlite3_int64) -> c_int {
    todo!()
}

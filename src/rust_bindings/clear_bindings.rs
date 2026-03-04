//!
//! Contrary to the intuition of many, sqlite3_reset() does not reset
//! the bindings on a prepared statement.
//! Use this routine to reset all host parameters to NULL.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_reset()
//!
use std::os::raw::*;

pub type Sqlite3Stmt = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_clear_bindings(stmt: *mut sqlite3_stmt) -> c_int {
    todo!()
}

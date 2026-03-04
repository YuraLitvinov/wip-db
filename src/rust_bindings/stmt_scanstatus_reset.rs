//!
//! Zero all sqlite3_stmt_scanstatus() related event counters.
//! This API is only available if the library is built with pre-processor
//! symbol SQLITE_ENABLE_STMT_SCANSTATUS defined.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_stmt_scanstatus()
//! - SQLITE_ENABLE_STMT_SCANSTATUS
//!
use std::os::raw::*;

pub type Sqlite3Stmt = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_stmt_scanstatus_reset(stmt: *mut sqlite3_stmt) {
    todo!()
}

//!
//! This function may only be called from within a call to the xUpdate method
//! of a virtual table implementation for an INSERT or UPDATE operation. The
//! value returned is one of SQLITE_ROLLBACK, SQLITE_IGNORE, SQLITE_FAIL,
//! SQLITE_ABORT, or SQLITE_REPLACE, according to the ON CONFLICT mode
//! of the SQL statement that triggered the call to the xUpdate method of the
//! virtual table.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - SQLITE_ROLLBACK
//! - SQLITE_IGNORE
//! - SQLITE_FAIL
//! - SQLITE_ABORT
//! - SQLITE_REPLACE
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_vtab_on_conflict(db: *mut Sqlite3) -> c_int {
    todo!()
}

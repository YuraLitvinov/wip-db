//!
//! The sqlite3_db_readonly(D,N) interface returns 1 if the database N
//! of connection D is read-only, 0 if it is read/write, or -1 if N is not
//! the name of a database on connection D.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_db_readonly(db: *mut Sqlite3, zDbName: *const c_char) -> c_int {
    todo!()
}

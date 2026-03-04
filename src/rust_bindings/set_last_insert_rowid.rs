//!
//! The sqlite3_set_last_insert_rowid(D, R) method allows the application to
//! set the value returned by calling sqlite3_last_insert_rowid(D) to R
//! without inserting a row into the database.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_set_last_insert_rowid(db: *mut Sqlite3, db2: sqlite3_int64) {
    todo!()
}

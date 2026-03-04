//!
//! This function is used to move an existing BLOB handle so that it points
//! to a different row of the same database table. The new row is identified
//! by the rowid value passed as the second argument. Only the row can be
//! changed. The database, table and column on which the blob handle is open
//! remain the same. Moving an existing BLOB handle to a new row is
//! faster than closing the existing handle and opening a new one.
//! The new row must meet the same criteria as for sqlite3_blob_open() -
//! it must exist and there must be either a blob or text value stored in
//! the nominated column. If the new row is not present in the table, or if
//! it does not contain a blob or text value, or if another error occurs, an
//! SQLite error code is returned and the blob handle is considered aborted.
//! All subsequent calls to sqlite3_blob_read(), sqlite3_blob_write() or
//! sqlite3_blob_reopen() on an aborted blob handle immediately return
//! SQLITE_ABORT. Calling sqlite3_blob_bytes() on an aborted blob handle
//! always returns zero.
//! This function sets the database handle error code and message.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_blob_open()
//! - sqlite3_blob_read()
//! - sqlite3_blob_write()
//! - sqlite3_blob_reopen()
//! - sqlite3_blob_bytes()
//!
use std::os::raw::*;

pub type Sqlite3Blob = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_blob_reopen(blob: *mut sqlite3_blob, db: sqlite3_int64) -> c_int {
    todo!()
}

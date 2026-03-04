//!
//! This function is used to write data into an open BLOB handle from a
//! caller-supplied buffer. N bytes of data are copied from the buffer Z
//! into the open BLOB, starting at offset iOffset.
//! On success, sqlite3_blob_write() returns SQLITE_OK.
//! Otherwise, an  error code or an extended error code is returned.
//! Unless SQLITE_MISUSE is returned, this function sets the
//! database connection error code and message accessible via
//! sqlite3_errcode() and sqlite3_errmsg() and related functions.
//! If the BLOB handle passed as the first argument was not opened for
//! writing (the flags parameter to sqlite3_blob_open() was zero),
//! this function returns SQLITE_READONLY.
//! This function may only modify the contents of the BLOB; it is
//! not possible to increase the size of a BLOB using this API.
//! If offset iOffset is less than N bytes from the end of the BLOB,
//! SQLITE_ERROR is returned and no data is written. The size of the
//! BLOB (and hence the maximum value of N+iOffset) can be determined
//! using the sqlite3_blob_bytes() interface. If N or iOffset are less
//! than zero SQLITE_ERROR is returned and no data is written.
//! An attempt to write to an expired BLOB handle fails with an
//! error code of SQLITE_ABORT.  Writes to the BLOB that occurred
//! before the BLOB handle expired are not rolled back by the
//! expiration of the handle, though of course those changes might
//! have been overwritten by the statement that expired the BLOB handle
//! or by other independent statements.
//! This routine only works on a BLOB handle which has been created
//! by a prior successful call to sqlite3_blob_open() and which has not
//! been closed by sqlite3_blob_close().  Passing any other pointer in
//! to this routine results in undefined and probably undesirable behavior.
//! See also: sqlite3_blob_read().
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_errcode()
//! - sqlite3_errmsg()
//! - sqlite3_blob_open()
//! - SQLITE_READONLY
//! - SQLITE_ERROR
//! - sqlite3_blob_bytes()
//! - SQLITE_ERROR
//! - SQLITE_ABORT
//! - sqlite3_blob_open()
//! - sqlite3_blob_close()
//!
use std::os::raw::*;

pub type Sqlite3Blob = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_blob_write(blob: *mut sqlite3_blob, z: *const c_void, n: c_int, iOffset: c_int) -> c_int {
    todo!()
}

//!
//! This function is used to read data from an open BLOB handle into a
//! caller-supplied buffer. N bytes of data are copied into buffer Z
//! from the open BLOB, starting at offset iOffset.
//! If offset iOffset is less than N bytes from the end of the BLOB,
//! SQLITE_ERROR is returned and no data is read.  If N or iOffset is
//! less than zero, SQLITE_ERROR is returned and no data is read.
//! The size of the blob (and hence the maximum value of N+iOffset)
//! can be determined using the sqlite3_blob_bytes() interface.
//! An attempt to read from an expired BLOB handle fails with an
//! error code of SQLITE_ABORT.
//! On success, sqlite3_blob_read() returns SQLITE_OK.
//! Otherwise, an error code or an extended error code is returned.
//! This routine only works on a BLOB handle which has been created
//! by a prior successful call to sqlite3_blob_open() and which has not
//! been closed by sqlite3_blob_close().  Passing any other pointer in
//! to this routine results in undefined and probably undesirable behavior.
//! See also: sqlite3_blob_write().
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - SQLITE_ERROR
//! - SQLITE_ERROR
//! - sqlite3_blob_bytes()
//! - SQLITE_ABORT
//! - sqlite3_blob_open()
//! - sqlite3_blob_close()
//! - sqlite3_blob_write()
//!
use std::os::raw::*;

pub type Sqlite3Blob = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_blob_read(blob: *mut sqlite3_blob, Z: *mut c_void, N: c_int, iOffset: c_int) -> c_int {
    todo!()
}

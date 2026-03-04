//!
//! Returns the size in bytes of the BLOB accessible via the
//! successfully opened BLOB handle in its only argument.  The
//! incremental blob I/O routines can only read or overwrite existing
//! blob content; they cannot change the size of a blob.
//! This routine only works on a BLOB handle which has been created
//! by a prior successful call to sqlite3_blob_open() and which has not
//! been closed by sqlite3_blob_close().  Passing any other pointer in
//! to this routine results in undefined and probably undesirable behavior.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_blob_open()
//! - sqlite3_blob_close()
//!
use std::os::raw::*;

pub type Sqlite3Blob = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_blob_bytes(blob: *mut sqlite3_blob) -> c_int {
    todo!()
}

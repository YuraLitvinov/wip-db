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

#[repr(C)]
struct Incrblob {
    nByte: c_int,
    iOffset: c_int,
    iCol: u16,
    pCsr: *mut c_void,
    pStmt: *mut c_void,
    db: *mut c_void,
    zDb: *mut c_char,
    pTab: *mut c_void,
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_blob_bytes(blob: *mut Sqlite3Blob) -> c_int {
    let p = blob as *mut Incrblob;
    if !p.is_null() {
        let incrblob = unsafe { &*p };
        if !incrblob.pStmt.is_null() {
            return incrblob.nByte;
        }
    }
    0
}

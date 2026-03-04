//!
//! Attempt to return the underlying operating system error code or error
//! number that caused the most recent I/O error or failure to open a file.
//! The return value is OS-dependent.  For example, on unix systems, after
//! sqlite3_open_v2() returns SQLITE_CANTOPEN, this interface could be
//! called to get back the underlying "errno" that caused the problem, such
//! as ENOSPC, EAUTH, EISDIR, and so forth.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_open_v2()
//! - SQLITE_CANTOPEN
//!
use std::os::raw::*;

pub type Sqlite3 = c_void;

#[repr(C)]
struct Sqlite3Struct {
    iSysErrno: c_int,
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_system_errno(db: *mut Sqlite3) -> c_int {
    if db.is_null() {
        0
    } else {
        let s = db as *mut Sqlite3Struct;
        unsafe { (*s).iSysErrno }
    }
}

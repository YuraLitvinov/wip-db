//!
//! These interfaces are used to retrieve runtime status information
//! about the performance of SQLite, and optionally to reset various
//! highwater marks.  The first argument is an integer code for
//! the specific parameter to measure.  Recognized integer codes
//! are of the form SQLITE_STATUS_....
//! The current value of the parameter is returned into *pCurrent.
//! The highest recorded value is returned in *pHighwater.  If the
//! resetFlag is true, then the highest record value is reset after
//! *pHighwater is written.  Some parameters do not record the highest
//! value.  For those parameters
//! nothing is written into *pHighwater and the resetFlag is ignored.
//! Other parameters record only the highwater mark and not the current
//! value.  For these latter parameters nothing is written into *pCurrent.
//! The sqlite3_status() and sqlite3_status64() routines return
//! SQLITE_OK on success and a non-zero error code on failure.
//! If either the current value or the highwater mark is too large to
//! be represented by a 32-bit integer, then the values returned by
//! sqlite3_status() are undefined.
//! See also: sqlite3_db_status()
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - SQLITE_STATUS_...
//! - sqlite3_db_status()
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_status(op: c_int, pCurrent: *mut c_int, pHighwater: *mut c_int, resetFlag: c_int) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_status64(op: c_int, pCurrent: *mut sqlite3_int64, pHighwater: *mut sqlite3_int64, resetFlag: c_int) -> c_int {
    todo!()
}

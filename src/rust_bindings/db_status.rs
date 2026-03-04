//!
//! This interface is used to retrieve runtime status information
//! about a single database connection.  The first argument is the
//! database connection object to be interrogated.  The second argument
//! is an integer constant, taken from the set of
//! SQLITE_DBSTATUS options, that
//! determines the parameter to interrogate.  The set of
//! SQLITE_DBSTATUS options is likely
//! to grow in future releases of SQLite.
//! The current value of the requested parameter is written into *pCur
//! and the highest instantaneous value is written into *pHiwtr.  If
//! the resetFlg is true, then the highest instantaneous value is
//! reset back down to the current value.
//! The sqlite3_db_status() routine returns SQLITE_OK on success and a
//! non-zero error code on failure.
//! The sqlite3_db_status64(D,O,C,H,R) routine works exactly the same
//! way as sqlite3_db_status(D,O,C,H,R) routine except that the C and H
//! parameters are pointer to 64-bit integers (type: sqlite3_int64) instead
//! of pointers to 32-bit integers, which allows larger status values
//! to be returned.  If a status value exceeds 2,147,483,647 then
//! sqlite3_db_status() will truncate the value whereas sqlite3_db_status64()
//! will return the full value.
//! See also: sqlite3_status() and sqlite3_stmt_status().
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - SQLITE_DBSTATUS options
//! - SQLITE_DBSTATUS options
//! - sqlite3_status()
//! - sqlite3_stmt_status()
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_db_status(db: *mut Sqlite3, op: c_int, pCur: *mut c_int, pHiwtr: *mut c_int, resetFlg: c_int) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_db_status64(db: *mut Sqlite3, count: c_int, db2: *mut sqlite3_int64, db3: *mut sqlite3_int64, count2: c_int) -> c_int {
    todo!()
}

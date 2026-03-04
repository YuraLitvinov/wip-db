//!
//! The sqlite3_data_count(P) interface returns the number of columns in the
//! current row of the result set of prepared statement P.
//! If prepared statement P does not have results ready to return
//! (via calls to the sqlite3_column() family of
//! interfaces) then sqlite3_data_count(P) returns 0.
//! The sqlite3_data_count(P) routine also returns 0 if P is a NULL pointer.
//! The sqlite3_data_count(P) routine returns 0 if the previous call to
//! sqlite3_step(P) returned SQLITE_DONE.  The sqlite3_data_count(P)
//! will return non-zero if previous call to sqlite3_step(P) returned
//! SQLITE_ROW, except in the case of the PRAGMA incremental_vacuum
//! where it always returns zero since each step of that multi-step
//! pragma returns 0 columns of data.
//! See also: sqlite3_column_count()
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_column()
//! - sqlite3_step
//! - SQLITE_DONE
//! - sqlite3_step
//! - SQLITE_ROW
//! - sqlite3_column_count()
//!
use std::os::raw::*;

pub type Sqlite3Stmt = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_data_count(pStmt: *mut sqlite3_stmt) -> c_int {
    todo!()
}

//!
//! The sqlite3_reset() function is called to reset a prepared statement
//! object back to its initial state, ready to be re-executed.
//! Any SQL statement variables that had values bound to them using
//! the sqlite3_bind_*() API retain their values.
//! Use sqlite3_clear_bindings() to reset the bindings.
//! The sqlite3_reset(S) interface resets the prepared statement S
//! back to the beginning of its program.
//! The return code from sqlite3_reset(S) indicates whether or not
//! the previous evaluation of prepared statement S completed successfully.
//! If sqlite3_step(S) has never before been called on S or if
//! sqlite3_step(S) has not been called since the previous call
//! to sqlite3_reset(S), then sqlite3_reset(S) will return
//! SQLITE_OK.
//! If the most recent call to sqlite3_step(S) for the
//! prepared statement S indicated an error, then
//! sqlite3_reset(S) returns an appropriate error code.
//! The sqlite3_reset(S) interface might also return an error code
//! if there were no prior errors but the process of resetting
//! the prepared statement caused a new error. For example, if an
//! INSERT statement with a RETURNING clause is only stepped one time,
//! that one call to sqlite3_step(S) might return SQLITE_ROW but
//! the overall statement might still fail and the sqlite3_reset(S) call
//! might return SQLITE_BUSY if locking constraints prevent the
//! database change from committing.  Therefore, it is important that
//! applications check the return code from sqlite3_reset(S) even if
//! no prior call to sqlite3_step(S) indicated a problem.
//! The sqlite3_reset(S) interface does not change the values
//! of any bindings on the prepared statement S.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_bind_*() API
//! - sqlite3_clear_bindings()
//! - sqlite3_reset(S)
//! - sqlite3_reset(S)
//! - sqlite3_step(S)
//! - sqlite3_step(S)
//! - sqlite3_reset(S)
//! - sqlite3_reset(S)
//! - SQLITE_OK
//! - sqlite3_step(S)
//!
use std::os::raw::*;

pub type Sqlite3Stmt = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> c_int {
    todo!()
}

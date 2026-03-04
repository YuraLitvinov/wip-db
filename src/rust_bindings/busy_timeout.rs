//!
//! This routine sets a busy handler that sleeps
//! for a specified amount of time when a table is locked.  The handler
//! will sleep multiple times until at least "ms" milliseconds of sleeping
//! have accumulated.  After at least "ms" milliseconds of sleeping,
//! the handler returns 0 which causes sqlite3_step() to return
//! SQLITE_BUSY.
//! Calling this routine with an argument less than or equal to zero
//! turns off all busy handlers.
//! There can only be a single busy handler for a particular
//! database connection at any given moment.  If another busy handler
//! was defined  (using sqlite3_busy_handler()) prior to calling
//! this routine, that other busy handler is cleared.
//! See also:  PRAGMA busy_timeout
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_step()
//! - SQLITE_BUSY
//! - sqlite3_busy_handler()
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_busy_timeout(db: *mut Sqlite3, ms: c_int) -> c_int {
    todo!()
}

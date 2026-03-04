//!
//! The sqlite3_stmt_busy(S) interface returns true (non-zero) if the
//! prepared statement S has been stepped at least once using
//! sqlite3_step(S) but has neither run to completion (returned
//! SQLITE_DONE from sqlite3_step(S)) nor
//! been reset using sqlite3_reset(S).  The sqlite3_stmt_busy(S)
//! interface returns false if S is a NULL pointer.  If S is not a
//! NULL pointer and is not a pointer to a valid prepared statement
//! object, then the behavior is undefined and probably undesirable.
//! This interface can be used in combination sqlite3_next_stmt()
//! to locate all prepared statements associated with a database
//! connection that are in need of being reset.  This can be used,
//! for example, in diagnostic routines to search for prepared
//! statements that are holding a transaction open.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_step(S)
//! - SQLITE_DONE
//! - sqlite3_step(S)
//! - sqlite3_reset(S)
//! - sqlite3_next_stmt()
//!
use std::os::raw::*;

pub type Sqlite3Stmt = c_void;

#[repr(C)]
struct Vdbe {
    eVdbeState: c_int,
}

pub extern "C" fn sqlite3_stmt_busy(stmt: *mut Sqlite3Stmt) -> c_int {
    if stmt.is_null() {
        0
    } else {
        let v = stmt as *mut Vdbe;
        let vdbe = unsafe { &*v };
        if vdbe.eVdbeState == 2 { 1 } else { 0 }
    }
}

//!
//! The sqlite3_stmt_isexplain(S) interface returns 1 if the
//! prepared statement S is an EXPLAIN statement, or 2 if the
//! statement S is an EXPLAIN QUERY PLAN.
//! The sqlite3_stmt_isexplain(S) interface returns 0 if S is
//! an ordinary statement or a NULL pointer.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//!
use std::os::raw::*;

pub type Sqlite3Stmt = c_void;

#[repr(C)]
struct Vdbe {
    explain: c_int,
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_stmt_isexplain(pStmt: *mut Sqlite3Stmt) -> c_int {
    if pStmt.is_null() {
        0
    } else {
        let vdbe = pStmt as *mut Vdbe;
        unsafe { (*vdbe).explain }
    }
}

//!
//! If the sqlite3_vtab_nochange(X) routine is called within the xColumn
//! method of a virtual table, then it might return true if the
//! column is being fetched as part of an UPDATE operation during which the
//! column value will not change.  The virtual table implementation can use
//! this hint as permission to substitute a return value that is less
//! expensive to compute and that the corresponding
//! xUpdate method understands as a "no-change" value.
//! If the xColumn method calls sqlite3_vtab_nochange() and finds that
//! the column is not changed by the UPDATE statement, then the xColumn
//! method can optionally return without setting a result, without calling
//! any of the sqlite3_result_xxxxx() interfaces.
//! In that case, sqlite3_value_nochange(X) will return true for the
//! same column in the xUpdate method.
//! The sqlite3_vtab_nochange() routine is an optimization.  Virtual table
//! implementations should continue to give a correct answer even if the
//! sqlite3_vtab_nochange() interface were to always return false.  In the
//! current implementation, the sqlite3_vtab_nochange() interface does always
//! returns false for the enhanced UPDATE FROM statement.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_result_xxxxx() interfaces
//! - sqlite3_value_nochange(X)
//!
use std::os::raw::*;

pub type Sqlite3Context = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_vtab_nochange(ctx: *mut Sqlite3Context) -> c_int {
    todo!()
}

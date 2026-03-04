//!
//! Return the number of columns in the result set returned by the
//! prepared statement. If this routine returns 0, that means the
//! prepared statement returns no data (for example an UPDATE).
//! However, just because this routine returns a positive number does not
//! mean that one or more rows of data will be returned.  A SELECT statement
//! will always have a positive sqlite3_column_count() but depending on the
//! WHERE clause constraints and the table content, it might return no rows.
//! See also: sqlite3_data_count()
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_data_count()
//!
use std::os::raw::*;

pub type Sqlite3Stmt = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> c_int {
    todo!()
}

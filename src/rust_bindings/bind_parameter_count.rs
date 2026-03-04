//!
//! This routine can be used to find the number of SQL parameters
//! in a prepared statement.  SQL parameters are tokens of the
//! form "?", "?NNN", ":AAA", "$AAA", or "@AAA" that serve as
//! placeholders for values that are bound
//! to the parameters at a later time.
//! This routine actually returns the index of the largest (rightmost)
//! parameter. For all forms except ?NNN, this will correspond to the
//! number of unique parameters.  If parameters of the ?NNN form are used,
//! there may be gaps in the list.
//! See also: sqlite3_bind(),
//! sqlite3_bind_parameter_name(), and
//! sqlite3_bind_parameter_index().
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_bind()
//! - sqlite3_bind_parameter_name()
//! - sqlite3_bind_parameter_index()
//!
use std::os::raw::*;

pub type Sqlite3Stmt = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_bind_parameter_count(stmt: *mut sqlite3_stmt) -> c_int {
    todo!()
}

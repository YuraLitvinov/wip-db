//!
//! Return the index of an SQL parameter given its name.  The
//! index value returned is suitable for use as the second
//! parameter to sqlite3_bind().  A zero
//! is returned if no matching parameter is found.  The parameter
//! name must be given in UTF-8 even if the original statement
//! was prepared from UTF-16 text using sqlite3_prepare16_v2() or
//! sqlite3_prepare16_v3().
//! See also: sqlite3_bind(),
//! sqlite3_bind_parameter_count(), and
//! sqlite3_bind_parameter_name().
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_bind()
//! - sqlite3_prepare16_v2()
//! - sqlite3_prepare16_v3()
//! - sqlite3_bind()
//! - sqlite3_bind_parameter_count()
//! - sqlite3_bind_parameter_name()
//!
use std::os::raw::*;

pub type Sqlite3Stmt = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_bind_parameter_index(stmt: *mut sqlite3_stmt, zName: *const c_char) -> c_int {
    todo!()
}

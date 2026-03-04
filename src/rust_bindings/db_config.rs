//!
//! The sqlite3_db_config() interface is used to make configuration
//! changes to a database connection.  The interface is similar to
//! sqlite3_config() except that the changes apply to a single
//! database connection (specified in the first argument).
//! The second argument to sqlite3_db_config(D,V,...)  is the
//! configuration verb - an integer code
//! that indicates what aspect of the database connection is being configured.
//! Subsequent arguments vary depending on the configuration verb.
//! Calls to sqlite3_db_config() return SQLITE_OK if and only if
//! the call is considered successful.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_config()
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_db_config(db: *mut Sqlite3, op: c_int, arg: ...) -> c_int {
    todo!()
}

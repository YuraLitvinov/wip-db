//!
//! These interfaces return the current status of an sqlite3_str object.
//! If any prior errors have occurred while constructing the dynamic string
//! in sqlite3_str X, then the sqlite3_str_errcode(X) method will return
//! an appropriate error code.  The sqlite3_str_errcode(X) method returns
//! SQLITE_NOMEM following any out-of-memory error, or
//! SQLITE_TOOBIG if the size of the dynamic string exceeds
//! SQLITE_MAX_LENGTH, or SQLITE_OK if there have been no errors.
//! The sqlite3_str_length(X) method returns the current length, in bytes,
//! of the dynamic string under construction in sqlite3_str object X.
//! The length returned by sqlite3_str_length(X) does not include the
//! zero-termination byte.
//! The sqlite3_str_value(X) method returns a pointer to the current
//! content of the dynamic string under construction in X.  The value
//! returned by sqlite3_str_value(X) is managed by the sqlite3_str object X
//! and might be freed or altered by any subsequent method on the same
//! sqlite3_str object.  Applications must not use the pointer returned by
//! sqlite3_str_value(X) after any subsequent method call on the same
//! object.  Applications may change the content of the string returned
//! by sqlite3_str_value(X) as long as they do not write into any bytes
//! outside the range of 0 to sqlite3_str_length(X) and do not read or
//! write any byte after any subsequent sqlite3_str method call.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_str
//! - sqlite3_str_errcode(X)
//! - sqlite3_str_errcode(X)
//! - SQLITE_NOMEM
//! - SQLITE_TOOBIG
//! - SQLITE_MAX_LENGTH
//! - SQLITE_OK
//! - sqlite3_str_length(X)
//! - sqlite3_str
//! - sqlite3_str_length(X)
//!
use std::os::raw::*;

pub type Sqlite3Str = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_str_errcode(db: *mut sqlite3_str) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_str_length(db: *mut sqlite3_str) -> c_int {
    todo!()
}

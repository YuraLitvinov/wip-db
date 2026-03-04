//!
//! To avoid having to register all collation sequences before a database
//! can be used, a single callback function may be registered with the
//! database connection to be invoked whenever an undefined collation
//! sequence is required.
//! If the function is registered using the sqlite3_collation_needed() API,
//! then it is passed the names of undefined collation sequences as strings
//! encoded in UTF-8. If sqlite3_collation_needed16() is used,
//! the names are passed as UTF-16 in machine native byte order.
//! A call to either function replaces the existing collation-needed callback.
//! When the callback is invoked, the first argument passed is a copy
//! of the second argument to sqlite3_collation_needed() or
//! sqlite3_collation_needed16().  The second argument is the database
//! connection.  The third argument is one of SQLITE_UTF8, SQLITE_UTF16BE,
//! or SQLITE_UTF16LE, indicating the most desirable form of the collation
//! sequence function required.  The fourth parameter is the name of the
//! required collation sequence.
//! The callback function should register the desired collation using
//! sqlite3_create_collation(), sqlite3_create_collation16(), or
//! sqlite3_create_collation_v2().
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - SQLITE_UTF8
//! - SQLITE_UTF16BE
//! - SQLITE_UTF16LE
//! - sqlite3_create_collation()
//! - sqlite3_create_collation16()
//! - sqlite3_create_collation_v2()
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_collation_needed(db: *mut Sqlite3, ptr: *mut c_void, db2: UnknownCallback) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_collation_needed16(db: *mut Sqlite3, ptr: *mut c_void, db2: UnknownCallback) -> c_int {
    todo!()
}

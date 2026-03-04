//!
//! These interfaces add content to an sqlite3_str object previously obtained
//! from sqlite3_str_new().
//! The sqlite3_str_appendf(X,F,...) and
//! sqlite3_str_vappendf(X,F,V) interfaces uses the built-in printf
//! functionality of SQLite to append formatted text onto the end of
//! sqlite3_str object X.
//! The sqlite3_str_append(X,S,N) method appends exactly N bytes from string S
//! onto the end of the sqlite3_str object X.  N must be non-negative.
//! S must contain at least N non-zero bytes of content.  To append a
//! zero-terminated string in its entirety, use the sqlite3_str_appendall()
//! method instead.
//! The sqlite3_str_appendall(X,S) method appends the complete content of
//! zero-terminated string S onto the end of sqlite3_str object X.
//! The sqlite3_str_appendchar(X,N,C) method appends N copies of the
//! single-byte character C onto the end of sqlite3_str object X.
//! This method can be used, for example, to add whitespace indentation.
//! The sqlite3_str_reset(X) method resets the string under construction
//! inside sqlite3_str object X back to zero bytes in length.
//! These methods do not return a result code.  If an error occurs, that fact
//! is recorded in the sqlite3_str object and can be recovered by a
//! subsequent call to sqlite3_str_errcode(X).
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_str_new()
//! - sqlite3_str_appendf(X,F,...)
//! - sqlite3_str_vappendf(X,F,V)
//! - sqlite3_str
//! - sqlite3_str_append(X,S,N)
//! - sqlite3_str
//! - sqlite3_str_appendall()
//! - sqlite3_str_appendall(X,S)
//! - sqlite3_str
//! - sqlite3_str_appendchar(X,N,C)
//!
use std::os::raw::*;

pub type Sqlite3Str = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_str_appendf(db: *mut sqlite3_str, zFormat: *const c_char, arg: ...) {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_str_vappendf(db: *mut sqlite3_str, zFormat: *const c_char, arg: va_list) {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_str_append(db: *mut sqlite3_str, zIn: *const c_char, N: c_int) {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_str_appendall(db: *mut sqlite3_str, zIn: *const c_char) {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_str_appendchar(db: *mut sqlite3_str, N: c_int, C: c_char) {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_str_reset(db: *mut sqlite3_str) {
    todo!()
}

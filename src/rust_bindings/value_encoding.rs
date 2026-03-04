//!
//! The sqlite3_value_encoding(X) interface returns one of SQLITE_UTF8,
//! SQLITE_UTF16BE, or SQLITE_UTF16LE according to the current text encoding
//! of the value X, assuming that X has type TEXT.  If sqlite3_value_type(X)
//! returns something other than SQLITE_TEXT, then the return value from
//! sqlite3_value_encoding(X) is meaningless.  Calls to
//! sqlite3_value_text(X), sqlite3_value_text16(X), sqlite3_value_text16be(X),
//! sqlite3_value_text16le(X), sqlite3_value_bytes(X), or
//! sqlite3_value_bytes16(X) might change the encoding of the value X and
//! thus change the return from subsequent calls to sqlite3_value_encoding(X).
//! This routine is intended for used by applications that test and validate
//! the SQLite implementation.  This routine is inquiring about the opaque
//! internal state of an sqlite3_value object.  Ordinary applications should
//! not need to know what the internal state of an sqlite3_value object is and
//! hence should not need to use this interface.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - SQLITE_UTF8
//! - SQLITE_UTF16BE
//! - SQLITE_UTF16LE
//! - sqlite3_value_text(X)
//! - sqlite3_value_text16(X)
//! - sqlite3_value_text16be(X)
//! - sqlite3_value_text16le(X)
//! - sqlite3_value_bytes(X)
//! - sqlite3_value_bytes16(X)
//! - sqlite3_value
//!
use std::os::raw::*;

pub type Sqlite3Value = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_value_encoding(val: *mut Sqlite3Value) -> c_int {
    todo!()
}

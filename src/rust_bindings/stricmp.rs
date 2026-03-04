//!
//! The sqlite3_stricmp() and sqlite3_strnicmp() APIs allow applications
//! and extensions to compare the contents of two buffers containing UTF-8
//! strings in a case-independent fashion, using the same definition of "case
//! independence" that SQLite uses internally when comparing identifiers.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_stricmp()
//! - sqlite3_strnicmp()
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_stricmp(str: *const c_char, str2: *const c_char) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_strnicmp(str: *const c_char, str2: *const c_char, count: c_int) -> c_int {
    todo!()
}

//!
//! The sqlite3_strglob(P,X) interface returns zero if and only if
//! string X matches the GLOB pattern P.
//! The definition of GLOB pattern matching used in
//! sqlite3_strglob(P,X) is the same as for the "X GLOB P" operator in the
//! SQL dialect understood by SQLite.  The sqlite3_strglob(P,X) function
//! is case sensitive.
//! Note that this routine returns zero on a match and non-zero if the strings
//! do not match, the same as sqlite3_stricmp() and sqlite3_strnicmp().
//! See also: sqlite3_strlike().
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_strglob(P,X)
//! - sqlite3_strglob(P,X)
//! - sqlite3_strglob(P,X)
//! - sqlite3_stricmp()
//! - sqlite3_strnicmp()
//! - sqlite3_strlike()
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_strglob(zGlob: *const c_char, zStr: *const c_char) -> c_int {
    todo!()
}

//!
//! The sqlite3_strlike(P,X,E) interface returns zero if and only if
//! string X matches the LIKE pattern P with escape character E.
//! The definition of LIKE pattern matching used in
//! sqlite3_strlike(P,X,E) is the same as for the "X LIKE P ESCAPE E"
//! operator in the SQL dialect understood by SQLite.  For "X LIKE P" without
//! the ESCAPE clause, set the E parameter of sqlite3_strlike(P,X,E) to 0.
//! As with the LIKE operator, the sqlite3_strlike(P,X,E) function is case
//! insensitive - equivalent upper and lower case ASCII characters match
//! one another.
//! The sqlite3_strlike(P,X,E) function matches Unicode characters, though
//! only ASCII characters are case folded.
//! Note that this routine returns zero on a match and non-zero if the strings
//! do not match, the same as sqlite3_stricmp() and sqlite3_strnicmp().
//! See also: sqlite3_strglob().
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_strlike(P,X,E)
//! - sqlite3_strlike(P,X,E)
//! - sqlite3_strlike(P,X,E)
//! - sqlite3_strlike(P,X,E)
//! - sqlite3_strlike(P,X,E)
//! - sqlite3_stricmp()
//! - sqlite3_strnicmp()
//! - sqlite3_strglob()
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_strlike(zGlob: *const c_char, zStr: *const c_char, cEsc: c_uint) -> c_int {
    todo!()
}

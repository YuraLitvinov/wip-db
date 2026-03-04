//!
//! These interfaces provide the same information as the SQLITE_VERSION,
//! SQLITE_VERSION_NUMBER, and SQLITE_SOURCE_ID C preprocessor macros
//! but are associated with the library instead of the header file.  Cautious
//! programmers might include assert() statements in their application to
//! verify that values returned by these interfaces match the macros in
//! the header, and thus ensure that the application is
//! compiled with matching library and header files.
//! assert( sqlite3_libversion_number()==SQLITE_VERSION_NUMBER );
//! assert( strncmp(sqlite3_sourceid(),SQLITE_SOURCE_ID,80)==0 );
//! assert( strcmp(sqlite3_libversion(),SQLITE_VERSION)==0 );
//! The sqlite3_version[] string constant contains the text of the
//! SQLITE_VERSION macro.  The sqlite3_libversion() function returns a
//! pointer to the sqlite3_version[] string constant.  The sqlite3_libversion()
//! function is provided for use in DLLs since DLL users usually do not have
//! direct access to string constants within the DLL.  The
//! sqlite3_libversion_number() function returns an integer equal to
//! SQLITE_VERSION_NUMBER.  The sqlite3_sourceid() function returns
//! a pointer to a string constant whose value is the same as the
//! SQLITE_SOURCE_ID C preprocessor macro.  Except if SQLite is built
//! using an edited copy of the amalgamation, then the last four characters
//! of the hash might be different from SQLITE_SOURCE_ID.
//! See also: sqlite_version() and sqlite_source_id().
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - SQLITE_VERSION
//! - SQLITE_VERSION_NUMBER
//! - SQLITE_SOURCE_ID
//! - SQLITE_VERSION
//! - SQLITE_VERSION_NUMBER
//! - SQLITE_SOURCE_ID
//! - SQLITE_SOURCE_ID
//!
use std::os::raw::*;

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_libversion_number() -> c_int {
    3051002
}

unsafe extern "C" {
    static sqlite3_version: [c_char; 0];
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_libversion() -> *const c_char {
    unsafe { sqlite3_version.as_ptr() }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_sourceid() -> *const c_char {
    b"2026-01-09 17:27:48 b270f8339eb13b504d0b2ba154ebca966b7dde08e40c3ed7d559749818cbalt1\0".as_ptr() as *const c_char
}

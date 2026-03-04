//!
//! Summary:
//! sqlite3_value_blob→BLOB value
//! sqlite3_value_double→REAL value
//! sqlite3_value_int→32-bit INTEGER value
//! sqlite3_value_int64→64-bit INTEGER value
//! sqlite3_value_pointer→Pointer value
//! sqlite3_value_text→UTF-8 TEXT value
//! sqlite3_value_text16→UTF-16 TEXT value in
//! the native byteorder
//! sqlite3_value_text16be→UTF-16be TEXT value
//! sqlite3_value_text16le→UTF-16le TEXT value
//! sqlite3_value_bytes→Size of a BLOB
//! or a UTF-8 TEXT in bytes
//! sqlite3_value_bytes16
//! →  Size of UTF-16
//! TEXT in bytes
//! sqlite3_value_type→Default
//! datatype of the value
//! sqlite3_value_numeric_type
//! →  Best numeric datatype of the value
//! sqlite3_value_nochange
//! →  True if the column is unchanged in an UPDATE
//! against a virtual table.
//! sqlite3_value_frombind
//! →  True if value originated from a bound parameter
//! Details:
//! These routines extract type, size, and content information from
//! protected sqlite3_value objects.  Protected sqlite3_value objects
//! are used to pass parameter information into the functions that
//! implement application-defined SQL functions and virtual tables.
//! These routines work only with protected sqlite3_value objects.
//! Any attempt to use these routines on an unprotected sqlite3_value
//! is not threadsafe.
//! These routines work just like the corresponding column access functions
//! except that these routines take a single protected sqlite3_value object
//! pointer instead of a sqlite3_stmt* pointer and an integer column number.
//! The sqlite3_value_text16() interface extracts a UTF-16 string
//! in the native byte-order of the host machine.  The
//! sqlite3_value_text16be() and sqlite3_value_text16le() interfaces
//! extract UTF-16 strings as big-endian and little-endian respectively.
//! If sqlite3_value object V was initialized
//! using sqlite3_bind_pointer(S,I,P,X,D) or sqlite3_result_pointer(C,P,X,D)
//! and if X and Y are strings that compare equal according to strcmp(X,Y),
//! then sqlite3_value_pointer(V,Y) will return the pointer P.  Otherwise,
//! sqlite3_value_pointer(V,Y) returns a NULL. The sqlite3_bind_pointer()
//! routine is part of the pointer passing interface added for SQLite 3.20.0.
//! The sqlite3_value_type(V) interface returns the
//! datatype code for the initial datatype of the
//! sqlite3_value object V. The returned value is one of SQLITE_INTEGER,
//! SQLITE_FLOAT, SQLITE_TEXT, SQLITE_BLOB, or SQLITE_NULL.
//! Other interfaces might change the datatype for an sqlite3_value object.
//! For example, if the datatype is initially SQLITE_INTEGER and
//! sqlite3_value_text(V) is called to extract a text value for that
//! integer, then subsequent calls to sqlite3_value_type(V) might return
//! SQLITE_TEXT.  Whether or not a persistent internal datatype conversion
//! occurs is undefined and may change from one release of SQLite to the next.
//! The sqlite3_value_numeric_type() interface attempts to apply
//! numeric affinity to the value.  This means that an attempt is
//! made to convert the value to an integer or floating point.  If
//! such a conversion is possible without loss of information (in other
//! words, if the value is a string that looks like a number)
//! then the conversion is performed.  Otherwise no conversion occurs.
//! The datatype after conversion is returned.
//! Within the xUpdate method of a virtual table, the
//! sqlite3_value_nochange(X) interface returns true if and only if
//! the column corresponding to X is unchanged by the UPDATE operation
//! that the xUpdate method call was invoked to implement and if
//! the prior xColumn method call that was invoked to extract
//! the value for that column returned without setting a result (probably
//! because it queried sqlite3_vtab_nochange() and found that the column
//! was unchanging).  Within an xUpdate method, any value for which
//! sqlite3_value_nochange(X) is true will in all other respects appear
//! to be a NULL value.  If sqlite3_value_nochange(X) is invoked anywhere other
//! than within an xUpdate method call for an UPDATE statement, then
//! the return value is arbitrary and meaningless.
//! The sqlite3_value_frombind(X) interface returns non-zero if the
//! value X originated from one of the sqlite3_bind()
//! interfaces.  If X comes from an SQL literal value, or a table column,
//! or an expression, then sqlite3_value_frombind(X) returns zero.
//! Please pay particular attention to the fact that the pointer returned
//! from sqlite3_value_blob(), sqlite3_value_text(), or
//! sqlite3_value_text16() can be invalidated by a subsequent call to
//! sqlite3_value_bytes(), sqlite3_value_bytes16(), sqlite3_value_text(),
//! or sqlite3_value_text16().
//! These routines must be called from the same thread as
//! the SQL function that supplied the sqlite3_value* parameters.
//! As long as the input parameter is correct, these routines can only
//! fail if an out-of-memory error occurs during a format conversion.
//! Only the following subset of interfaces are subject to out-of-memory
//! errors:
//! sqlite3_value_blob()
//! sqlite3_value_text()
//! sqlite3_value_text16()
//! sqlite3_value_text16le()
//! sqlite3_value_text16be()
//! sqlite3_value_bytes()
//! sqlite3_value_bytes16()
//! If an out-of-memory error occurs, then the return value from these
//! routines is the same as if the column had contained an SQL NULL value.
//! Valid SQL NULL returns can be distinguished from out-of-memory errors
//! by invoking the sqlite3_errcode() immediately after the suspect
//! return value is obtained and before any
//! other SQLite interface is called on the same database connection.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_stmt*
//! - sqlite3_value
//! - sqlite3_bind_pointer(S,I,P,X,D)
//! - sqlite3_result_pointer(C,P,X,D)
//! - sqlite3_value
//! - SQLITE_INTEGER
//! - SQLITE_FLOAT
//! - SQLITE_TEXT
//! - SQLITE_BLOB
//! - SQLITE_NULL
//!
use std::os::raw::*;

pub type Sqlite3Value = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_value_double(val: *mut Sqlite3Value) -> c_double {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_value_int(val: *mut Sqlite3Value) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_value_int64(val: *mut Sqlite3Value) -> sqlite3_int64 {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_value_bytes(val: *mut Sqlite3Value) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_value_bytes16(val: *mut Sqlite3Value) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_value_type(val: *mut Sqlite3Value) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_value_numeric_type(val: *mut Sqlite3Value) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_value_nochange(val: *mut Sqlite3Value) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_value_frombind(val: *mut Sqlite3Value) -> c_int {
    todo!()
}

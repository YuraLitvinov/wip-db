//!
//! The sqlite3_carray_bind(S,I,P,N,F,X) interface binds an array value to
//! one of the first argument of the carray() table-valued function.  The
//! S parameter is a pointer to the prepared statement that uses the carray()
//! functions.  I is the parameter index to be bound.  P is a pointer to the
//! array to be bound, and N is the number of eements in the array.  The
//! F argument is one of constants SQLITE_CARRAY_INT32, SQLITE_CARRAY_INT64,
//! SQLITE_CARRAY_DOUBLE, SQLITE_CARRAY_TEXT, or SQLITE_CARRAY_BLOB to
//! indicate the datatype of the array being bound.  The X argument is not a
//! NULL pointer, then SQLite will invoke the function X on the P parameter
//! after it has finished using P, even if the call to
//! sqlite3_carray_bind() fails. The special-case finalizer
//! SQLITE_TRANSIENT has no effect here.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - SQLITE_CARRAY_INT32
//! - SQLITE_CARRAY_INT64
//! - SQLITE_CARRAY_DOUBLE
//! - SQLITE_CARRAY_TEXT
//! - SQLITE_CARRAY_BLOB
//!
use std::os::raw::*;

pub type Sqlite3Stmt = c_void;

pub type DelCallback = Option<unsafe extern "C" fn(arg0: *mut c_void)>;

#[no_mangle]
pub extern "C" fn sqlite3_carray_bind(pStmt: *mut sqlite3_stmt, i: c_int, aData: *mut c_void, nData: c_int, mFlags: c_int, xDel: DelCallback) -> c_int {
    todo!()
}

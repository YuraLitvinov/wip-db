//!
//! These interfaces are provided for use by VFS shim implementations and
//! are not useful outside of that context.
//! The sqlite3_create_filename(D,J,W,N,P) allocates memory to hold a version of
//! database filename D with corresponding journal file J and WAL file W and
//! an array P of N URI Key/Value pairs.  The result from
//! sqlite3_create_filename(D,J,W,N,P) is a pointer to a database filename that
//! is safe to pass to routines like:
//! sqlite3_uri_parameter(),
//! sqlite3_uri_boolean(),
//! sqlite3_uri_int64(),
//! sqlite3_uri_key(),
//! sqlite3_filename_database(),
//! sqlite3_filename_journal(), or
//! sqlite3_filename_wal().
//! If a memory allocation error occurs, sqlite3_create_filename() might
//! return a NULL pointer.  The memory obtained from sqlite3_create_filename(X)
//! must be released by a corresponding call to sqlite3_free_filename(Y).
//! The P parameter in sqlite3_create_filename(D,J,W,N,P) should be an array
//! of 2*N pointers to strings.  Each pair of pointers in this array corresponds
//! to a key and value for a query parameter.  The P parameter may be a NULL
//! pointer if N is zero.  None of the 2*N pointers in the P array may be
//! NULL pointers and key pointers should not be empty strings.
//! None of the D, J, or W parameters to sqlite3_create_filename(D,J,W,N,P) may
//! be NULL pointers, though they can be empty strings.
//! The sqlite3_free_filename(Y) routine releases a memory allocation
//! previously obtained from sqlite3_create_filename().  Invoking
//! sqlite3_free_filename(Y) where Y is a NULL pointer is a harmless no-op.
//! If the Y parameter to sqlite3_free_filename(Y) is anything other
//! than a NULL pointer or a pointer previously acquired from
//! sqlite3_create_filename(), then bad things such as heap
//! corruption or segfaults may occur. The value Y should not be
//! used again after sqlite3_free_filename(Y) has been called.  This means
//! that if the sqlite3_vfs.xOpen() method of a VFS has been called using Y,
//! then the corresponding [sqlite3_module.xClose() method should also be
//! invoked prior to calling sqlite3_free_filename(Y).
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_uri_parameter()
//! - sqlite3_uri_boolean()
//! - sqlite3_uri_int64()
//! - sqlite3_uri_key()
//! - sqlite3_filename_database()
//! - sqlite3_filename_journal()
//! - sqlite3_filename_wal()
//! - sqlite3_vfs.xOpen()
//!
use std::os::raw::*;

pub type Sqlite3Filename = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_create_filename(zDatabase: *const c_char, zJournal: *const c_char, zWal: *const c_char, nParam: c_int, azParam: *mut *const c_char) -> sqlite3_filename {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_free_filename(file: sqlite3_filename) {
    todo!()
}

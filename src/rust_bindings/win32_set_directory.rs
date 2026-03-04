//!
//! These interfaces are available only on Windows.  The
//! sqlite3_win32_set_directory interface is used to set the value associated
//! with the sqlite3_temp_directory or sqlite3_data_directory variable, to
//! zValue, depending on the value of the type parameter.  The zValue parameter
//! should be NULL to cause the previous value to be freed via sqlite3_free;
//! a non-NULL value will be copied into memory obtained from sqlite3_malloc
//! prior to being used.  The sqlite3_win32_set_directory interface returns
//! SQLITE_OK to indicate success, SQLITE_ERROR if the type is unsupported,
//! or SQLITE_NOMEM if memory could not be allocated.  The value of the
//! sqlite3_data_directory variable is intended to act as a replacement for
//! the current directory on the sub-platforms of Win32 where that concept is
//! not present, e.g. WinRT and UWP.  The sqlite3_win32_set_directory8 and
//! sqlite3_win32_set_directory16 interfaces behave exactly the same as the
//! sqlite3_win32_set_directory interface except the string parameter must be
//! UTF-8 or UTF-16, respectively.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_win32_set_directory
//! - sqlite3_temp_directory
//! - sqlite3_data_directory
//! - sqlite3_free
//! - sqlite3_malloc
//! - sqlite3_win32_set_directory
//! - SQLITE_OK
//! - SQLITE_ERROR
//! - SQLITE_NOMEM
//! - sqlite3_data_directory
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_win32_set_directory(type: c_ulong, zValue: *mut c_void) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_win32_set_directory8(type: c_ulong, zValue: *const c_char) -> c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_win32_set_directory16(type: c_ulong, zValue: *const c_void) -> c_int {
    todo!()
}

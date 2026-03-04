//!
//! The sqlite3_drop_modules(D,L) interface removes all virtual
//! table modules from database connection D except those named on list L.
//! The L parameter must be either NULL or a pointer to an array of pointers
//! to strings where the array is terminated by a single NULL pointer.
//! If the L parameter is NULL, then all virtual table modules are removed.
//! See also: sqlite3_create_module()
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_create_module()
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_drop_modules(db: *mut Sqlite3, azKeep: *mut *const c_char) -> c_int {
    todo!()
}

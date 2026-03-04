//!
//! The sqlite3_db_release_memory(D) interface attempts to free as much heap
//! memory as possible from database connection D. Unlike the
//! sqlite3_release_memory() interface, this interface is in effect even
//! when the SQLITE_ENABLE_MEMORY_MANAGEMENT compile-time option is
//! omitted.
//! See also: sqlite3_release_memory()
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_release_memory()
//! - SQLITE_ENABLE_MEMORY_MANAGEMENT
//! - sqlite3_release_memory()
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_db_release_memory(db: *mut Sqlite3) -> c_int {
    todo!()
}

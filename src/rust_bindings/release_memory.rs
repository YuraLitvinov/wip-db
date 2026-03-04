//!
//! The sqlite3_release_memory() interface attempts to free N bytes
//! of heap memory by deallocating non-essential memory allocations
//! held by the database library.   Memory used to cache database
//! pages to improve performance is an example of non-essential memory.
//! sqlite3_release_memory() returns the number of bytes actually freed,
//! which might be more or less than the amount requested.
//! The sqlite3_release_memory() routine is a no-op returning zero
//! if SQLite is not compiled with SQLITE_ENABLE_MEMORY_MANAGEMENT.
//! See also: sqlite3_db_release_memory()
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - SQLITE_ENABLE_MEMORY_MANAGEMENT
//! - sqlite3_db_release_memory()
//!
use std::os::raw::*;

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_release_memory(_n: c_int) -> c_int {
    // No memory management implemented, return 0
    0
}

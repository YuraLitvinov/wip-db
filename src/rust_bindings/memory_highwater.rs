//!
//! SQLite provides these two interfaces for reporting on the status
//! of the sqlite3_malloc(), sqlite3_free(), and sqlite3_realloc()
//! routines, which form the built-in memory allocation subsystem.
//! The sqlite3_memory_used() routine returns the number of bytes
//! of memory currently outstanding (malloced but not freed).
//! The sqlite3_memory_highwater() routine returns the maximum
//! value of sqlite3_memory_used() since the high-water mark
//! was last reset.  The values returned by sqlite3_memory_used() and
//! sqlite3_memory_highwater() include any overhead
//! added by SQLite in its implementation of sqlite3_malloc(),
//! but not overhead added by any underlying system library
//! routines that sqlite3_malloc() may call.
//! The memory high-water mark is reset to the current value of
//! sqlite3_memory_used() if and only if the parameter to
//! sqlite3_memory_highwater() is true.  The value returned
//! by sqlite3_memory_highwater(1) is the high-water mark
//! prior to the reset.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_malloc()
//! - sqlite3_free()
//! - sqlite3_realloc()
//! - sqlite3_memory_used()
//! - sqlite3_memory_highwater()
//! - sqlite3_memory_used()
//! - sqlite3_memory_used()
//! - sqlite3_memory_highwater()
//! - sqlite3_malloc()
//! - sqlite3_malloc()
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_memory_used() -> sqlite3_int64 {
    todo!()
}

#[no_mangle]
pub extern "C" fn sqlite3_memory_highwater(resetFlag: c_int) -> sqlite3_int64 {
    todo!()
}

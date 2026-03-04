//!
//! The sqlite3_snapshot_free(P) interface destroys sqlite3_snapshot P.
//! The application must eventually free every sqlite3_snapshot object
//! using this routine to avoid a memory leak.
//! The sqlite3_snapshot_free() interface is only available when the
//! SQLITE_ENABLE_SNAPSHOT compile-time option is used.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_snapshot_free(P)
//! - sqlite3_snapshot
//! - sqlite3_snapshot
//! - sqlite3_snapshot_free()
//! - SQLITE_ENABLE_SNAPSHOT
//!
use std::os::raw::*;

pub type Sqlite3Snapshot = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_snapshot_free(db: *mut sqlite3_snapshot) {
    todo!()
}

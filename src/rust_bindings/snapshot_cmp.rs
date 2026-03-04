//!
//! The sqlite3_snapshot_cmp(P1, P2) interface is used to compare the ages
//! of two valid snapshot handles.
//! If the two snapshot handles are not associated with the same database
//! file, the result of the comparison is undefined.
//! Additionally, the result of the comparison is only valid if both of the
//! snapshot handles were obtained by calling sqlite3_snapshot_get() since the
//! last time the wal file was deleted. The wal file is deleted when the
//! database is changed back to rollback mode or when the number of database
//! clients drops to zero. If either snapshot handle was obtained before the
//! wal file was last deleted, the value returned by this function
//! is undefined.
//! Otherwise, this API returns a negative value if P1 refers to an older
//! snapshot than P2, zero if the two handles refer to the same database
//! snapshot, and a positive value if P1 is a newer snapshot than P2.
//! This interface is only available if SQLite is compiled with the
//! SQLITE_ENABLE_SNAPSHOT option.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - SQLITE_ENABLE_SNAPSHOT
//!
use std::os::raw::*;

pub type Sqlite3Snapshot = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_snapshot_cmp(p1: *mut sqlite3_snapshot, p2: *mut sqlite3_snapshot) -> c_int {
    todo!()
}

//!
//! The sqlite3_wal_checkpoint(D,X) is equivalent to
//! sqlite3_wal_checkpoint_v2(D,X,SQLITE_CHECKPOINT_PASSIVE,0,0).
//! In brief, sqlite3_wal_checkpoint(D,X) causes the content in the
//! write-ahead log for database X on database connection D to be
//! transferred into the database file and for the write-ahead log to
//! be reset.  See the checkpointing documentation for addition
//! information.
//! This interface used to be the only way to cause a checkpoint to
//! occur.  But then the newer and more powerful sqlite3_wal_checkpoint_v2()
//! interface was added.  This interface is retained for backwards
//! compatibility and as a convenience for applications that need to manually
//! start a callback but which do not need the full power (and corresponding
//! complication) of sqlite3_wal_checkpoint_v2().
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_wal_checkpoint_v2
//! - SQLITE_CHECKPOINT_PASSIVE
//! - sqlite3_wal_checkpoint_v2()
//! - sqlite3_wal_checkpoint_v2()
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_wal_checkpoint(db: *mut Sqlite3, zDb: *const c_char) -> c_int {
    todo!()
}

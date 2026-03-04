//!
//! The sqlite3_wal_autocheckpoint(D,N) is a wrapper around
//! sqlite3_wal_hook() that causes any database on database connection D
//! to automatically checkpoint
//! after committing a transaction if there are N or
//! more frames in the write-ahead log file.  Passing zero or
//! a negative value as the N parameter disables automatic
//! checkpoints entirely.
//! The callback registered by this function replaces any existing callback
//! registered using sqlite3_wal_hook().  Likewise, registering a callback
//! using sqlite3_wal_hook() disables the automatic checkpoint mechanism
//! configured by this function.
//! The wal_autocheckpoint pragma can be used to invoke this interface
//! from SQL.
//! Checkpoints initiated by this mechanism are
//! PASSIVE.
//! Every new database connection defaults to having the auto-checkpoint
//! enabled with a threshold of 1000 or SQLITE_DEFAULT_WAL_AUTOCHECKPOINT
//! pages.
//! The use of this interface is only necessary if the default setting
//! is found to be suboptimal for a particular application.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_wal_autocheckpoint(D,N)
//! - sqlite3_wal_hook()
//! - sqlite3_wal_hook()
//! - sqlite3_wal_hook()
//! - SQLITE_DEFAULT_WAL_AUTOCHECKPOINT
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_wal_autocheckpoint(db: *mut Sqlite3, N: c_int) -> c_int {
    todo!()
}

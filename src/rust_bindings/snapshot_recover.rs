//!
//! If a WAL file remains on disk after all database connections close
//! (either through the use of the SQLITE_FCNTL_PERSIST_WAL file control
//! or because the last process to have the database opened exited without
//! calling sqlite3_close()) and a new connection is subsequently opened
//! on that database and WAL file, the sqlite3_snapshot_open() interface
//! will only be able to open the last transaction added to the WAL file
//! even though the WAL file contains other valid transactions.
//! This function attempts to scan the WAL file associated with database zDb
//! of database handle db and make all valid snapshots available to
//! sqlite3_snapshot_open(). It is an error if there is already a read
//! transaction open on the database, or if the database is not a WAL mode
//! database.
//! SQLITE_OK is returned if successful, or an SQLite error code otherwise.
//! This interface is only available if SQLite is compiled with the
//! SQLITE_ENABLE_SNAPSHOT option.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - SQLITE_FCNTL_PERSIST_WAL
//! - sqlite3_close()
//! - sqlite3_snapshot_open()
//! - SQLITE_ENABLE_SNAPSHOT
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_snapshot_recover(db: *mut Sqlite3, zDb: *const c_char) -> c_int {
    todo!()
}

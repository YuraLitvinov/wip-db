//!
//! The sqlite3_txn_state(D,S) interface returns the current
//! transaction state of schema S in database connection D.  If S is NULL,
//! then the highest transaction state of any schema on database connection D
//! is returned.  Transaction states are (in order of lowest to highest):
//! SQLITE_TXN_NONE
//! SQLITE_TXN_READ
//! SQLITE_TXN_WRITE
//! If the S argument to sqlite3_txn_state(D,S) is not the name of
//! a valid schema, then -1 is returned.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_txn_state(db: *mut Sqlite3, zSchema: *const c_char) -> c_int {
    todo!()
}

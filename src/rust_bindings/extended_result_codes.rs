//!
//! The sqlite3_extended_result_codes() routine enables or disables the
//! extended result codes feature of SQLite. The extended result
//! codes are disabled by default for historical compatibility.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_extended_result_codes(db: *mut Sqlite3, onoff: c_int) -> c_int {
    todo!()
}

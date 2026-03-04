//!
//! The sqlite3_value_subtype(V) function returns the subtype for
//! an application-defined SQL function argument V.  The subtype
//! information can be used to pass a limited amount of context from
//! one SQL function to another.  Use the sqlite3_result_subtype()
//! routine to set the subtype for the return value of an SQL function.
//! Every application-defined SQL function that invokes this interface
//! should include the SQLITE_SUBTYPE property in the text
//! encoding argument when the function is registered.
//! If the SQLITE_SUBTYPE property is omitted, then sqlite3_value_subtype()
//! might return zero instead of the upstream subtype in some corner cases.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_result_subtype()
//! - SQLITE_SUBTYPE
//! - SQLITE_SUBTYPE
//!
use std::os::raw::*;

pub type Sqlite3Value = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_value_subtype(val: *mut Sqlite3Value) -> c_uint {
    todo!()
}

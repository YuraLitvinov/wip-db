//!
//! The xCreate and xConnect methods of a
//! virtual table module call this interface
//! to declare the format (the names and datatypes of the columns) of
//! the virtual tables they implement.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_declare_vtab(db: *mut Sqlite3, zSQL: *const c_char) -> c_int {
    todo!()
}

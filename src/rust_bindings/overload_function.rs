//!
//! Virtual tables can provide alternative implementations of functions
//! using the xFindFunction method of the virtual table module.
//! But global versions of those functions
//! must exist in order to be overloaded.
//! This API makes sure a global version of a function with a particular
//! name and number of parameters exists.  If no such function exists
//! before this API is called, a new function is created.  The implementation
//! of the new function always causes an exception to be thrown.  So
//! the new function is not good for anything by itself.  Its only
//! purpose is to be a placeholder function that can be overloaded
//! by a virtual table.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_overload_function(db: *mut Sqlite3, zFuncName: *const c_char, nArg: c_int) -> c_int {
    todo!()
}

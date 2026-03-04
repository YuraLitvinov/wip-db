//!
//! This function may be called by either the xConnect or xCreate method
//! of a virtual table implementation to configure
//! various facets of the virtual table interface.
//! If this interface is invoked outside the context of an xConnect or
//! xCreate virtual table method then the behavior is undefined.
//! In the call sqlite3_vtab_config(D,C,...) the D parameter is the
//! database connection in which the virtual table is being created and
//! which is passed in as the first argument to the xConnect or xCreate
//! method that is invoking sqlite3_vtab_config().  The C parameter is one
//! of the virtual table configuration options.  The presence and meaning
//! of parameters after C depend on which virtual table configuration option
//! is used.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_vtab_config(db: *mut Sqlite3, op: c_int, arg: ...) -> c_int {
    todo!()
}

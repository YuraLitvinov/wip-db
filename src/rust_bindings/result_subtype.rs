//!
//! The sqlite3_result_subtype(C,T) function causes the subtype of
//! the result from the application-defined SQL function with
//! sqlite3_context C to be the value T.  Only the lower 8 bits
//! of the subtype T are preserved in current versions of SQLite;
//! higher order bits are discarded.
//! The number of subtype bytes preserved by SQLite might increase
//! in future releases of SQLite.
//! Every application-defined SQL function that invokes this interface
//! should include the SQLITE_RESULT_SUBTYPE property in its
//! text encoding argument when the SQL function is
//! registered.  If the SQLITE_RESULT_SUBTYPE
//! property is omitted from the function that invokes sqlite3_result_subtype(),
//! then in some cases the sqlite3_result_subtype() might fail to set
//! the result subtype.
//! If SQLite is compiled with -DSQLITE_STRICT_SUBTYPE=1, then any
//! SQL function that invokes the sqlite3_result_subtype() interface
//! and that does not have the SQLITE_RESULT_SUBTYPE property will raise
//! an error.  Future versions of SQLite might enable -DSQLITE_STRICT_SUBTYPE=1
//! by default.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_context
//! - SQLITE_RESULT_SUBTYPE
//! - SQLITE_RESULT_SUBTYPE
//!
use std::os::raw::*;

pub type Sqlite3Context = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_result_subtype(ctx: *mut Sqlite3Context, int: unsigned) {
    todo!()
}

//!
//! This API may only be used from within the xBestIndex method
//! of a virtual table implementation. The result of calling this interface
//! from outside of an xBestIndex method are undefined and probably harmful.
//! When the sqlite3_vtab_rhs_value(P,J,V) interface is invoked from within
//! the xBestIndex method of a virtual table implementation, with P being
//! a copy of the sqlite3_index_info object pointer passed into xBestIndex and
//! J being a 0-based index into P->aConstraint[], then this routine
//! attempts to set *V to the value of the right-hand operand of
//! that constraint if the right-hand operand is known.  If the
//! right-hand operand is not known, then *V is set to a NULL pointer.
//! The sqlite3_vtab_rhs_value(P,J,V) interface returns SQLITE_OK if
//! and only if *V is set to a value.  The sqlite3_vtab_rhs_value(P,J,V)
//! inteface returns SQLITE_NOTFOUND if the right-hand side of the J-th
//! constraint is not available.  The sqlite3_vtab_rhs_value() interface
//! can return a result code other than SQLITE_OK or SQLITE_NOTFOUND if
//! something goes wrong.
//! The sqlite3_vtab_rhs_value() interface is usually only successful if
//! the right-hand operand of a constraint is a literal value in the original
//! SQL statement.  If the right-hand operand is an expression or a reference
//! to some other column or a host parameter, then sqlite3_vtab_rhs_value()
//! will probably return SQLITE_NOTFOUND.
//! Some constraints, such as SQLITE_INDEX_CONSTRAINT_ISNULL and
//! SQLITE_INDEX_CONSTRAINT_ISNOTNULL, have no right-hand operand.  For such
//! constraints, sqlite3_vtab_rhs_value() always returns SQLITE_NOTFOUND.
//! The sqlite3_value object returned in *V is a protected sqlite3_value
//! and remains valid for the duration of the xBestIndex method call.
//! When xBestIndex returns, the sqlite3_value object returned by
//! sqlite3_vtab_rhs_value() is automatically deallocated.
//! The "_rhs_" in the name of this routine is an abbreviation for
//! "Right-Hand Side".
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_index_info
//! - SQLITE_NOTFOUND
//! - SQLITE_INDEX_CONSTRAINT_ISNULL
//! - SQLITE_INDEX_CONSTRAINT_ISNOTNULL
//! - sqlite3_value
//!
use std::os::raw::*;

pub type Sqlite3IndexInfo = c_void;
pub type Sqlite3Value = c_void;

#[no_mangle]
pub extern "C" fn sqlite3_vtab_rhs_value(db: *mut sqlite3_index_info, count: c_int, ppVal: *mut *mut Sqlite3Value) -> c_int {
    todo!()
}

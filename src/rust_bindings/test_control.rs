//!
//! The sqlite3_test_control() interface is used to read out internal
//! state of SQLite and to inject faults into SQLite for testing
//! purposes.  The first parameter is an operation code that determines
//! the number, meaning, and operation of all subsequent parameters.
//! This interface is not for use by applications.  It exists solely
//! for verifying the correct operation of the SQLite library.  Depending
//! on how the SQLite library is compiled, this interface might not exist.
//! The details of the operation codes, their meanings, the parameters
//! they take, and what they do are all subject to change without notice.
//! Unlike most of the SQLite API, this function is not guaranteed to
//! operate consistently from one release to the next.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_test_control(op: c_int, arg: ...) -> c_int {
    todo!()
}

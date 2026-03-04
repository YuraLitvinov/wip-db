//!
//! The sqlite3_cancel_auto_extension(X) interface unregisters the
//! initialization routine X that was registered using a prior call to
//! sqlite3_auto_extension(X).  The sqlite3_cancel_auto_extension(X)
//! routine returns 1 if initialization routine X was successfully
//! unregistered and it returns 0 if X was not on the list of initialization
//! routines.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_cancel_auto_extension(X)
//! - sqlite3_auto_extension(X)
//! - sqlite3_cancel_auto_extension(X)
//!
use std::os::raw::*;

pub type EntrypointCallback = Option<unsafe extern "C" fn()>;

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_cancel_auto_extension(_x_entry_point: EntrypointCallback) -> c_int {
    // Return 0: not found (auto-extensions not implemented)
    0
}

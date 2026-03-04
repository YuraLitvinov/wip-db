//!
//! This interface disables all automatic extensions previously
//! registered using sqlite3_auto_extension().
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_auto_extension()
//!
use std::os::raw::*;

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_reset_auto_extension() {
    // No-op: automatic extensions are not implemented
}

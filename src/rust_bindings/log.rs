//!
//! The sqlite3_log() interface writes a message into the error log
//! established by the SQLITE_CONFIG_LOG option to sqlite3_config().
//! If logging is enabled, the zFormat string and subsequent arguments are
//! used with sqlite3_snprintf() to generate the final output string.
//! The sqlite3_log() interface is intended for use by extensions such as
//! virtual tables, collating functions, and SQL functions.  While there is
//! nothing to prevent an application from calling sqlite3_log(), doing so
//! is considered bad form.
//! The zFormat string must not be NULL.
//! To avoid deadlocks and other threading problems, the sqlite3_log() routine
//! will not use dynamically allocated memory.  The log message is stored in
//! a fixed-length buffer on the stack.  If the log message is longer than
//! a few hundred characters, it will be truncated to the length of the
//! buffer.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_log()
//! - SQLITE_CONFIG_LOG
//! - sqlite3_config()
//! - sqlite3_snprintf()
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_log(iErrCode: c_int, zFormat: *const c_char, arg: ...) {
    todo!()
}

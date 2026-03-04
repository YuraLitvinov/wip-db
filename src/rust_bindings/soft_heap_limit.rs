//!
//! This is a deprecated version of the sqlite3_soft_heap_limit64()
//! interface.  This routine is provided for historical compatibility
//! only.  All new applications should use the
//! sqlite3_soft_heap_limit64() interface rather than this one.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_soft_heap_limit64()
//! - sqlite3_soft_heap_limit64()
//!
use std::os::raw::*;

unsafe extern "C" {
    fn sqlite3_soft_heap_limit64(n: i64);
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_soft_heap_limit(n: c_int) {
    let mut limit = n;
    if limit < 0 {
        limit = 0;
    }
    unsafe {
        sqlite3_soft_heap_limit64(limit as i64);
    }
}

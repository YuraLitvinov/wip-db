//!
//! The sqlite3_strglob(P,X) interface returns zero if and only if
//! string X matches the GLOB pattern P.
//! The definition of GLOB pattern matching used in
//! sqlite3_strglob(P,X) is the same as for the "X GLOB P" operator in the
//! SQL dialect understood by SQLite.  The sqlite3_strglob(P,X) function
//! is case sensitive.
//! Note that this routine returns zero on a match and non-zero if the strings
//! do not match, the same as sqlite3_stricmp() and sqlite3_strnicmp().
//! See also: sqlite3_strlike().
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_strglob(P,X)
//! - sqlite3_strglob(P,X)
//! - sqlite3_strglob(P,X)
//! - sqlite3_stricmp()
//! - sqlite3_strnicmp()
//! - sqlite3_strlike()
//!
use std::os::raw::*;

const SQLITE_MATCH: c_int = 0;
const SQLITE_NOMATCH: c_int = 1;
const SQLITE_NOWILDCARDMATCH: c_int = 2;

fn pattern_compare_glob(pattern: &[u8], string: &[u8]) -> c_int {
    let mut p_idx = 0;
    let mut s_idx = 0;

    loop {
        if p_idx >= pattern.len() {
            return if s_idx >= string.len() {
                SQLITE_MATCH
            } else {
                SQLITE_NOMATCH
            };
        }

        let p_char = pattern[p_idx];

        match p_char {
            b'*' => {
                // Skip multiple * characters
                while p_idx < pattern.len() && pattern[p_idx] == b'*' {
                    p_idx += 1;
                }

                if p_idx >= pattern.len() {
                    return SQLITE_MATCH;
                }

                // Try to match the rest of the pattern at each position in the string
                loop {
                    let result = pattern_compare_glob(&pattern[p_idx..], &string[s_idx..]);
                    if result == SQLITE_MATCH {
                        return SQLITE_MATCH;
                    }
                    if s_idx >= string.len() {
                        break;
                    }
                    s_idx += 1;
                }
                return SQLITE_NOMATCH;
            }
            b'?' => {
                // Match any single character
                if s_idx >= string.len() {
                    return SQLITE_NOMATCH;
                }
                p_idx += 1;
                s_idx += 1;
            }
            b'[' => {
                // Character class matching
                if s_idx >= string.len() {
                    return SQLITE_NOMATCH;
                }
                let s_char = string[s_idx];
                p_idx += 1;

                let mut invert = false;
                if p_idx < pattern.len() && pattern[p_idx] == b'^' {
                    invert = true;
                    p_idx += 1;
                }

                let mut matched = false;
                while p_idx < pattern.len() && pattern[p_idx] != b']' {
                    if p_idx + 2 < pattern.len()
                        && pattern[p_idx + 1] == b'-'
                        && pattern[p_idx + 2] != b']'
                    {
                        // Range like [a-z]
                        if s_char >= pattern[p_idx] && s_char <= pattern[p_idx + 2] {
                            matched = true;
                        }
                        p_idx += 3;
                    } else {
                        // Single character
                        if s_char == pattern[p_idx] {
                            matched = true;
                        }
                        p_idx += 1;
                    }
                }

                if p_idx >= pattern.len() {
                    return SQLITE_NOMATCH;
                }

                // Skip the closing ]
                p_idx += 1;

                if (matched && !invert) || (!matched && invert) {
                    s_idx += 1;
                } else {
                    return SQLITE_NOMATCH;
                }
            }
            _ => {
                // Regular character
                if s_idx >= string.len() || string[s_idx] != p_char {
                    return SQLITE_NOMATCH;
                }
                p_idx += 1;
                s_idx += 1;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_strglob(z_glob: *const c_char, z_str: *const c_char) -> c_int {
    if z_str.is_null() {
        if z_glob.is_null() {
            SQLITE_MATCH
        } else {
            SQLITE_NOMATCH
        }
    } else if z_glob.is_null() {
        SQLITE_NOMATCH
    } else {
        unsafe {
            let glob_cstr = std::ffi::CStr::from_ptr(z_glob);
            let str_cstr = std::ffi::CStr::from_ptr(z_str);
            let glob_bytes = glob_cstr.to_bytes();
            let str_bytes = str_cstr.to_bytes();

            pattern_compare_glob(glob_bytes, str_bytes)
        }
    }
}

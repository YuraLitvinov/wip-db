//!
//! The sqlite3_strlike(P,X,E) interface returns zero if and only if
//! string X matches the LIKE pattern P with escape character E.
//! The definition of LIKE pattern matching used in
//! sqlite3_strlike(P,X,E) is the same as for the "X LIKE P ESCAPE E"
//! operator in the SQL dialect understood by SQLite.  For "X LIKE P" without
//! the ESCAPE clause, set the E parameter of sqlite3_strlike(P,X,E) to 0.
//! As with the LIKE operator, the sqlite3_strlike(P,X,E) function is case
//! insensitive - equivalent upper and lower case ASCII characters match
//! one another.
//! The sqlite3_strlike(P,X,E) function matches Unicode characters, though
//! only ASCII characters are case folded.
//! Note that this routine returns zero on a match and non-zero if the strings
//! do not match, the same as sqlite3_stricmp() and sqlite3_strnicmp().
//! See also: sqlite3_strglob().
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - sqlite3_strlike(P,X,E)
//! - sqlite3_strlike(P,X,E)
//! - sqlite3_strlike(P,X,E)
//! - sqlite3_strlike(P,X,E)
//! - sqlite3_strlike(P,X,E)
//! - sqlite3_stricmp()
//! - sqlite3_strnicmp()
//! - sqlite3_strglob()
//!
use std::os::raw::*;

const SQLITE_MATCH: c_int = 0;
const SQLITE_NOMATCH: c_int = 1;

fn to_lower(c: u8) -> u8 {
    if c >= b'A' && c <= b'Z' {
        c - b'A' + b'a'
    } else {
        c
    }
}

fn pattern_compare_like(
    pattern: &[u8],
    string: &[u8],
    escape: Option<u8>,
    case_insensitive: bool,
) -> c_int {
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

        // Handle escape character
        if escape.is_some() && p_char == escape.unwrap() && p_idx + 1 < pattern.len() {
            p_idx += 1;
            let next_char = pattern[p_idx];
            if s_idx >= string.len() {
                return SQLITE_NOMATCH;
            }
            let s_char = string[s_idx];
            let matches = if case_insensitive {
                to_lower(s_char) == to_lower(next_char)
            } else {
                s_char == next_char
            };

            if !matches {
                return SQLITE_NOMATCH;
            }
            p_idx += 1;
            s_idx += 1;
            continue;
        }

        match p_char {
            b'%' => {
                // Match any sequence of characters
                p_idx += 1;

                // Skip multiple % characters
                while p_idx < pattern.len() && pattern[p_idx] == b'%' {
                    p_idx += 1;
                }

                if p_idx >= pattern.len() {
                    return SQLITE_MATCH;
                }

                // Try to match the rest of the pattern at each position in the string
                loop {
                    let result = pattern_compare_like(&pattern[p_idx..], &string[s_idx..], escape, case_insensitive);
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
            b'_' => {
                // Match any single character
                if s_idx >= string.len() {
                    return SQLITE_NOMATCH;
                }
                p_idx += 1;
                s_idx += 1;
            }
            _ => {
                // Regular character - case insensitive comparison
                if s_idx >= string.len() {
                    return SQLITE_NOMATCH;
                }

                let s_char = string[s_idx];
                let matches = if case_insensitive {
                    to_lower(s_char) == to_lower(p_char)
                } else {
                    s_char == p_char
                };

                if !matches {
                    return SQLITE_NOMATCH;
                }
                p_idx += 1;
                s_idx += 1;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_strlike(
    z_pattern: *const c_char,
    z_str: *const c_char,
    c_esc: c_uint,
) -> c_int {
    if z_str.is_null() {
        if z_pattern.is_null() {
            SQLITE_MATCH
        } else {
            SQLITE_NOMATCH
        }
    } else if z_pattern.is_null() {
        SQLITE_NOMATCH
    } else {
        unsafe {
            let pattern_cstr = std::ffi::CStr::from_ptr(z_pattern);
            let str_cstr = std::ffi::CStr::from_ptr(z_str);
            let pattern_bytes = pattern_cstr.to_bytes();
            let str_bytes = str_cstr.to_bytes();

            let escape = if c_esc > 0 {
                Some(c_esc as u8)
            } else {
                None
            };

            pattern_compare_like(pattern_bytes, str_bytes, escape, true)
        }
    }
}

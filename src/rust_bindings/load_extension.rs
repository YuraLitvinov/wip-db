//!
//! This interface loads an SQLite extension library from the named file.
//! The sqlite3_load_extension() interface attempts to load an
//! SQLite extension library contained in the file zFile.  If
//! the file cannot be loaded directly, attempts are made to load
//! with various operating-system specific extensions added.
//! So for example, if "samplelib" cannot be loaded, then names like
//! "samplelib.so" or "samplelib.dylib" or "samplelib.dll" might
//! be tried also.
//! The entry point is zProc.
//! zProc may be 0, in which case SQLite will try to come up with an
//! entry point name on its own.  It first tries "sqlite3_extension_init".
//! If that does not work, it constructs a name "sqlite3_X_init" where
//! X consists of the lower-case equivalent of all ASCII alphabetic
//! characters in the filename from the last "/" to the first following
//! "." and omitting any initial "lib".
//! The sqlite3_load_extension() interface returns
//! SQLITE_OK on success and SQLITE_ERROR if something goes wrong.
//! If an error occurs and pzErrMsg is not 0, then the
//! sqlite3_load_extension() interface shall attempt to
//! fill *pzErrMsg with error message text stored in memory
//! obtained from sqlite3_malloc(). The calling function
//! should free this memory by calling sqlite3_free().
//! Extension loading must be enabled using
//! sqlite3_enable_load_extension() or
//! sqlite3_db_config(db,SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION,1,NULL)
//! prior to calling this API,
//! otherwise an error will be returned.
//! Security warning: It is recommended that the
//! SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION method be used to enable only this
//! interface.  The use of the sqlite3_enable_load_extension() interface
//! should be avoided.  This will keep the SQL function load_extension()
//! disabled and prevent SQL injections from giving attackers
//! access to extension loading capabilities.
//! See also the load_extension() SQL function.
//! See also lists of
//! Objects,
//! Constants, and
//! Functions.
//! Related constants and functions:
//! - SQLITE_OK
//! - SQLITE_ERROR
//! - sqlite3_load_extension()
//! - sqlite3_malloc()
//! - sqlite3_free()
//! - sqlite3_enable_load_extension()
//! - sqlite3_db_config
//! - SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION
//! - SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION
//! - sqlite3_enable_load_extension()
//!
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn sqlite3_load_extension(db: *mut Sqlite3, zFile: *const c_char, zProc: *const c_char, pzErrMsg: *mut *mut c_char) -> c_int {
    todo!()
}

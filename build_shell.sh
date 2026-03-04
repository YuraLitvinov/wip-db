#!/usr/bin/env bash
set -e

# Build sqlite3 shell by linking shell.c against libsqlite3.so

PROJ=$(pwd)
SRC="$PROJ/sqlite-src-3510200"

if [ ! -f "$PROJ/lib/libsqlite3.so" ]; then
    echo "ERROR: libsqlite3.so not found at $PROJ/lib/libsqlite3.so"
    echo "Run ./create_libsqlite3_so.sh first"
    exit 1
fi


cc -o "$SRC/sqlite3" \
  "$SRC/shell.c" \
  -I"$(readlink -f "$SRC")" \
  -L"$PROJ/lib" -lsqlite3 \
  -Wl,-rpath,"$PROJ/lib" \
  -lm -lpthread -ldl \
  -DSQLITE_SHELL_HAVE_RECOVER=1 \
  -DSQLITE_ENABLE_DBPAGE_VTAB \
  -DSQLITE_ENABLE_DBSTAT_VTAB \


chmod +x "$SRC/sqlite3"

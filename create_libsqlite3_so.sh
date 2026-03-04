#!/usr/bin/env bash
set -euo pipefail

PROJ=$(pwd)
SRC="$PROJ/sqlite-src-3510200"
TEMP_DIR=$(mktemp -d /tmp/sqlite_so_XXXXXX)
trap "rm -rf '$TEMP_DIR'" EXIT

cargo build -q --release


# Compile fresh sqlite3.o with correct defines
SQLITE3_OBJ="$PROJ/target/sqlite3_so.o"
cc -fPIC -O3 -g \
  $(sed 's/\r//' "$PROJ/defines.txt" | xargs) \
  -I"$SRC" -I"$SRC/src" \
  -c "$SRC/sqlite3.c" \
  -o "$SQLITE3_OBJ"

# Rename symbols in the compiled object
while IFS= read -r sym; do
    objcopy --redefine-sym "${sym}=__c_${sym}" "$SQLITE3_OBJ"
done < "$PROJ/sqlite3_symbols.txt"

# Verify tungsten_register_mutex exists in Rust staticlib before linking
# echo "→ checking Rust staticlib exports:"
# nm "$PROJ/target/release/libtungsten_db.a" 2>/dev/null \
#   | grep "tungsten_register_mutex" \
#   || { echo "ERROR: tungsten_register_mutex not found in staticlib"; exit 1; }

# Link everything together
mkdir -p "$PROJ/lib"

cc -shared -fPIC -O3 -g \
  "$SQLITE3_OBJ" \
  -Wl,--whole-archive \
  "$PROJ/target/release/libtungsten_db.a" \
  -Wl,--no-whole-archive \
  -Wl,--export-dynamic \
  -Wl,--version-script="$PROJ/sqlite3.exports" \
  -Wl,--allow-multiple-definition \
  -Wl,-rpath,'$ORIGIN' \
  -lm -lpthread -ldl -licuuc -licui18n -licudata \
  -o "$PROJ/lib/libsqlite3.so"
  #-Wl,-init,tungsten_register_mutex \

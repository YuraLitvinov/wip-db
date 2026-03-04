#!/bin/bash
set -e

TESTFIXTURE="./sqlite-src-3510200/rustfixture"
TESTDIR="./sqlite-src-3510200/test"
JOBS="${1:-16}"  # parallelism, default 4, override with: ./run_fast_tests.sh 8

mapfile -t TESTS < <(find "$TESTDIR" -name "*.test" -type f | sort)

TMPBASE=$(mktemp -d)
trap 'rm -rf "$TMPBASE"' EXIT

PASSED=0
FAILED=0
FAILED_NAMES=()

run_test() {
    local filepath="$1"
    local t
    t=$(basename "$filepath" .test)
    local workdir="$TMPBASE/$t"
    mkdir -p "$workdir"
    if (cd "$workdir" && "$OLDPWD/$TESTFIXTURE" "$OLDPWD/$filepath") \
        > "$TMPBASE/${t}.log" 2>&1; then
        echo "pass:$t"
    else
        echo "fail:$t"
    fi
}

export -f run_test
export TESTFIXTURE TESTDIR TMPBASE OLDPWD="$PWD"

echo "Running ${#TESTS[@]} tests with $JOBS parallel jobs..."
echo ""

# Use GNU parallel if available, otherwise xargs -P
if command -v parallel &>/dev/null; then
    results=$(printf '%s\n' "${TESTS[@]}" | parallel -j "$JOBS" run_test)
else
    results=$(printf '%s\n' "${TESTS[@]}" | xargs -P "$JOBS" -I{} bash -c 'run_test "$@"' _ {})
fi

while IFS= read -r line; do
    status="${line%%:*}"
    name="${line#*:}"
    if [ "$status" = "pass" ]; then
        echo "  ✓ $name"
        ((PASSED += 1))
    else
        echo "  ✗ $name"
        ((FAILED += 1))
        FAILED_NAMES+=("$name")
        tail -n 5 "$TMPBASE/${name}.log" | sed 's/^/      /'
    fi
done <<< "$results"

echo ""
echo "Results: $PASSED passed, $FAILED failed"

if [ ${#FAILED_NAMES[@]} -gt 0 ]; then
    echo "Failed: ${FAILED_NAMES[*]}"
    exit 1
fi

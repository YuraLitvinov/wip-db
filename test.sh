#!/bin/bash
set -e

# test.sh - Unified testing workflow: build + test + log
#
# Usage:
#   ./test.sh <model> <function_name> [function_name2] ... [--issue "description"]
#
# Models: haiku, sonnet, opus
#
# This script:
#   1. Rebuilds library with create_libsqlite3_so.sh
#   2. Optionally rebuilds testfixture if sqlite3_symbols.txt changed
#   3. Finds and runs relevant tests for all functions
#   4. Logs results to JSON via test_logger.py

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MODEL="${1:-}"
shift || true

FUNCTION_NAMES=()
ISSUE_DESC=""

# Parse remaining arguments
while [ $# -gt 0 ]; do
    case "$1" in
        --issue)
            ISSUE_DESC="$2"
            shift 2
            ;;
        *)
            FUNCTION_NAMES+=("$1")
            shift
            ;;
    esac
done

cd "$SCRIPT_DIR"

# ============================================================================
# VALIDATION
# ============================================================================

if [ -z "$MODEL" ] || [ ${#FUNCTION_NAMES[@]} -eq 0 ]; then
    cat <<EOF
Usage: ./test.sh <model> <function_name> [function_name2] ... [--issue "description"]

Models: haiku, sonnet, opus

Examples:
  ./test.sh haiku sqlite3_changes
  ./test.sh sonnet sqlite3_randomness sqlite3_errmsg
  ./test.sh haiku sqlite3_changes --issue "Segfault after change"

EOF
    exit 1
fi

echo "=========================================="
echo "Rust-SQLite Testing Workflow"
echo "=========================================="
echo "Model: $MODEL"
echo "Functions: ${FUNCTION_NAMES[*]}"
if [ -n "$ISSUE_DESC" ]; then
    echo "Issue: $ISSUE_DESC"
fi
echo ""

# ============================================================================
# STEP 1: REBUILD LIBRARY
# ============================================================================

echo "[1/3] Rebuilding library with create_libsqlite3_so.sh..."
if [ ! -f "./create_libsqlite3_so.sh" ]; then
    echo "ERROR: create_libsqlite3_so.sh not found"
    exit 1
fi
bash ./create_libsqlite3_so.sh "${FUNCTION_NAMES[@]}"

# ============================================================================
# STEP 2: CONDITIONAL TESTFIXTURE REBUILD
# ============================================================================

echo ""
echo "[2/3] Checking for sqlite3_symbols.txt changes..."

# Check if sqlite3_symbols.txt was modified in git diff
if git diff HEAD sqlite3_symbols.txt 2>/dev/null | grep -q "^[+-]"; then
    echo "  → Changes detected, rebuilding testfixture..."
    if [ ! -f "./testfixture_swap_optimized.sh" ]; then
        echo "  ERROR: testfixture_swap_optimized.sh not found"
        exit 1
    fi
    bash ./testfixture_swap_optimized.sh "${FUNCTION_NAMES[@]}"
else
    echo "  → No changes detected (skipping testfixture rebuild)"
fi

# ============================================================================
# STEP 3: FIND AND RUN TESTS
# ============================================================================

echo ""
echo "[3/3] Finding and running tests"

TEST_DIR="./sqlite-src-3510200/test"
if [ ! -d "$TEST_DIR" ]; then
    echo "ERROR: Test directory not found at $TEST_DIR"
    exit 1
fi

TESTFIXTURE="./sqlite-src-3510200/rustfixture"
if [ ! -f "$TESTFIXTURE" ]; then
    echo "ERROR: testfixture not found at $TESTFIXTURE"
    exit 1
fi

TOTAL_PASSED=0
TOTAL_FAILED=0
PASSED_TEST_FILES=""
FAILED_TEST_FILES=""
FAILED_TESTS_BY_FN=()
TEST_STATUS="skipped"

# Process each function name
for FN_NAME in "${FUNCTION_NAMES[@]}"; do
    echo ""
    echo "  Testing: $FN_NAME"

    # Find test files containing this function name
    FOUND_TESTS=$(find "$TEST_DIR" -name "*.test" -type f -exec grep -l "$FN_NAME" {} \; | sort)

    if [ -z "$FOUND_TESTS" ]; then
        echo "    → No test files found"
        continue
    fi

    PASSED=0
    FAILED=0
    FUNCTION_FAILED_TESTS=""

    echo "    Found test files:"
    while IFS= read -r test_file; do
        test_name=$(basename "$test_file")
        echo "      • $test_name"
    done <<< "$FOUND_TESTS"

    echo "    Running tests:"
    while IFS= read -r test_file; do
        test_name=$(basename "$test_file")
        echo -n "      → $test_name ... "

        if $TESTFIXTURE "$test_file" > /tmp/test_output.log 2>&1; then
            echo "✓ PASS"
            ((PASSED += 1))
            ((TOTAL_PASSED += 1))
            TEST_STATUS="pass"
            if [ -n "$PASSED_TEST_FILES" ]; then
                PASSED_TEST_FILES="$PASSED_TEST_FILES,$test_name"
            else
                PASSED_TEST_FILES="$test_name"
            fi
        else
            echo "✗ FAIL"
            ((FAILED += 1))
            ((TOTAL_FAILED += 1))
            FUNCTION_FAILED_TESTS="$FUNCTION_FAILED_TESTS\n      - $test_name"
            if [ -n "$FAILED_TEST_FILES" ]; then
                FAILED_TEST_FILES="$FAILED_TEST_FILES,$test_name"
            else
                FAILED_TEST_FILES="$test_name"
            fi
            # Show last few lines of failure
            tail -n 5 /tmp/test_output.log | sed 's/^/        /'
        fi
    done <<< "$FOUND_TESTS"

    echo "    Results: $PASSED passed, $FAILED failed"
    if [ $FAILED -gt 0 ]; then
        echo -e "    Failed tests:$FUNCTION_FAILED_TESTS"
        TEST_STATUS="fail"
        FAILED_TESTS_BY_FN+=("$FN_NAME: $FAILED failures")
    fi
done

echo ""
echo "  Overall Results: $TOTAL_PASSED passed, $TOTAL_FAILED failed"
if [ $TOTAL_FAILED -gt 0 ]; then
    TEST_STATUS="fail"
fi

# ============================================================================
# WRITE FUNCTION NAMES TO sqlite3_symbols.txt (only on success)
# ============================================================================

if [ "$TEST_STATUS" = "pass" ]; then
    echo "Writing function names to sqlite3_symbols.txt..."
    printf "%s\n" "${FUNCTION_NAMES[@]}" > sqlite3_symbols.txt
    echo "  → Written ${#FUNCTION_NAMES[@]} symbol(s)"
else
    echo "Skipping sqlite3_symbols.txt update (tests did not pass)"
fi

# ============================================================================
# STEP 4: LOG RESULTS
# ============================================================================

echo ""
echo "Logging results to JSON..."

FUNCTIONS_STR=$(IFS=,; echo "${FUNCTION_NAMES[*]}")

# Log failed tests
LOG_ARGS=(
    --model "$MODEL"
    --function "$FUNCTIONS_STR"
    --status "$TEST_STATUS"
)

if [ -n "$FAILED_TEST_FILES" ]; then
    LOG_ARGS+=(--tests "$FAILED_TEST_FILES")
fi

if [ -n "$ISSUE_DESC" ]; then
    LOG_ARGS+=(--issue "$ISSUE_DESC")
elif [ "$TEST_STATUS" = "fail" ]; then
    ISSUE_MSG="$TOTAL_FAILED test(s) failed"
    if [ ${#FAILED_TESTS_BY_FN[@]} -gt 0 ]; then
        ISSUE_MSG="$ISSUE_MSG (${FAILED_TESTS_BY_FN[0]})"
    fi
    LOG_ARGS+=(--issue "$ISSUE_MSG")
fi

echo $LOG_ARGS
python3 testing/test_logger.py "${LOG_ARGS[@]}"

# Log passed tests
if [ -n "$PASSED_TEST_FILES" ]; then
    python3 testing/test_logger.py --model "$MODEL" --function "$FUNCTIONS_STR" --status "pass" --tests "$PASSED_TEST_FILES"
fi

# ============================================================================
# SUMMARY
# ============================================================================

echo ""
echo "=========================================="
echo "Test Log"
echo "=========================================="
python3 testing/test_logger.py --view
#!/usr/bin/env python3
"""
test_logger.py - Log testing results to JSON schema

Usage:
    python3 test_logger.py --model haiku --function sqlite3_changes --tests changes.test,changes2.test --status pass
    python3 test_logger.py --model sonnet --function sqlite3_randomness --status fail --issue "Segfault on NULL pointer"
    python3 test_logger.py --view

Note: --model is required when logging results (not required for --view)
"""

import json
import argparse
import os
import time
from datetime import datetime
from pathlib import Path

# Log file in project root (one level up from this script)
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(SCRIPT_DIR)
LOG_FILE = os.path.join(PROJECT_ROOT, f".test_log.json")

def load_log():
    """Load existing log or create new one."""
    if os.path.exists(LOG_FILE):
        with open(LOG_FILE, 'r+') as f:
            return json.load(f)
    return {
        "history": [],
        "current": {
            "model": None,
            "function": None,
            "tests": [],
            "status": None,
            "current_issue": None,
            "timestamp": None
        }
    }

def save_log(log_data):
    """Save log to JSON file."""
    with open(LOG_FILE, 'w') as f:
        json.dump(log_data, f, indent=2)
    print(f"✓ Logged to {LOG_FILE}")

def log_test(model, function, tests, status, issue=None):
    """Log a test run."""
    log_data = load_log()

    # Convert tests string to list if needed
    if isinstance(tests, str):
        tests = [t.strip() for t in tests.split(',') if t.strip()]

    entry = {
        "timestamp": datetime.now().isoformat(),
        "model": model,
        "function": function,
        "tests": tests,
        "status": status,
        "current_issue": issue
    }

    # Update current state
    log_data["current"] = entry

    # Add to history
    log_data["history"].append(entry)

    save_log(log_data)
    print_entry(entry)

def print_entry(entry):
    """Pretty print a log entry."""
    print("\n" + "="*60)
    print(f"Model:  {entry['model']}")
    print(f"Function: {entry['function']}")
    print(f"Status: {entry['status']}")
    print(f"Tests: {', '.join(entry['tests']) if entry['tests'] else 'None'}")
    if entry['current_issue']:
        print(f"Issue: {entry['current_issue']}")
    print(f"Time: {entry['timestamp']}")
    print("="*60)

def view_log():
    """Display the current log."""
    log_data = load_log()

    print("\n" + "="*60)
    print("CURRENT STATE")
    print("="*60)
    current = log_data["current"]
    if current["model"]:
        print_entry(current)
    else:
        print("No test runs logged yet.")

    if log_data["history"]:
        print("\n" + "="*60)
        print(f"HISTORY ({len(log_data['history'])} entries)")
        print("="*60)
        for i, entry in enumerate(log_data["history"][-10:], 1):  # Last 10
            ts = entry["timestamp"][:19]  # YYYY-MM-DD HH:MM:SS
            model = entry["model"]
            fn = entry["function"] or "unknown"
            status = entry["status"]
            tests = len(entry["tests"])
            print(f"{i:2d}. [{ts}] {model:8s} | {fn:25s} | {status:8s} | {tests} tests")

def main():
    parser = argparse.ArgumentParser(
        description="Log rust-sqlite test runs to JSON",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 test_logger.py --model haiku --function sqlite3_changes --tests changes.test --status pass
  python3 test_logger.py --model sonnet --function sqlite3_randomness --status fail --issue "Segfault on NULL"
  python3 test_logger.py --view
        """
    )

    parser.add_argument("--model", help="Model used (haiku, sonnet, opus) - REQUIRED for logging")
    parser.add_argument("--function", help="Function being tested/implemented")
    parser.add_argument("--tests", help="Comma-separated test files (e.g., 'changes.test,changes2.test')")
    parser.add_argument("--status", choices=["pass", "fail", "partial", "skipped"],
                       help="Overall test status")
    parser.add_argument("--issue", help="Current issue/troubleshooting notes")
    parser.add_argument("--view", action="store_true", help="Display current log")

    args = parser.parse_args()

    if args.view:
        view_log()
    elif args.function and args.status:
        if not args.model:
            parser.error("--model is required when logging test results")
        log_test(args.model, args.function, args.tests or "", args.status, args.issue)
    else:
        parser.print_help()

if __name__ == "__main__":
    main()

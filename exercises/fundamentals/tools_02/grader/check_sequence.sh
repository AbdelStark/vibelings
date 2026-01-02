#!/bin/bash
# Check tool call sequence invariants
# Input: JSON array of tool calls via stdin

set -e

# Read input
INPUT=$(cat)

# Check if we have any tool calls
CALL_COUNT=$(echo "$INPUT" | jq 'length')
if [ "$CALL_COUNT" -eq 0 ]; then
    echo "ERROR: No tool calls provided"
    exit 1
fi

# Extract tool names in order
TOOL_SEQUENCE=$(echo "$INPUT" | jq -r '.[].name')

# Check 1: Must read orders.py before writing to it
READ_ORDERS_IDX=-1
WRITE_ORDERS_IDX=-1
IDX=0
while IFS= read -r tool; do
    case "$tool" in
        read_file)
            # Check if this reads orders.py
            PATH_ARG=$(echo "$INPUT" | jq -r ".[$IDX].arguments.path // empty")
            if echo "$PATH_ARG" | grep -q "orders.py"; then
                if [ "$READ_ORDERS_IDX" -eq -1 ]; then
                    READ_ORDERS_IDX=$IDX
                fi
            fi
            ;;
        write_file)
            # Check if this writes orders.py
            PATH_ARG=$(echo "$INPUT" | jq -r ".[$IDX].arguments.path // empty")
            if echo "$PATH_ARG" | grep -q "orders.py"; then
                WRITE_ORDERS_IDX=$IDX
            fi
            ;;
    esac
    IDX=$((IDX + 1))
done <<< "$TOOL_SEQUENCE"

# Validate read before write
if [ "$WRITE_ORDERS_IDX" -ne -1 ]; then
    if [ "$READ_ORDERS_IDX" -eq -1 ]; then
        echo "ERROR: write_file called on orders.py without reading it first"
        exit 1
    fi
    if [ "$READ_ORDERS_IDX" -ge "$WRITE_ORDERS_IDX" ]; then
        echo "ERROR: read_file must come before write_file for orders.py"
        exit 1
    fi
fi

# Check 2: run_tests should come after any write_file
LAST_WRITE_IDX=-1
LAST_TEST_IDX=-1
IDX=0
while IFS= read -r tool; do
    case "$tool" in
        write_file)
            LAST_WRITE_IDX=$IDX
            ;;
        run_tests)
            LAST_TEST_IDX=$IDX
            ;;
    esac
    IDX=$((IDX + 1))
done <<< "$TOOL_SEQUENCE"

if [ "$LAST_WRITE_IDX" -ne -1 ] && [ "$LAST_TEST_IDX" -ne -1 ]; then
    if [ "$LAST_WRITE_IDX" -ge "$LAST_TEST_IDX" ]; then
        echo "ERROR: run_tests should come after write_file to verify changes"
        exit 1
    fi
fi

# Check 3: Must have at least read_file for orders.py
if [ "$READ_ORDERS_IDX" -eq -1 ]; then
    echo "ERROR: Must read orders.py to understand the bug"
    exit 1
fi

# Check 4: Should have write_file to fix the bug
if [ "$WRITE_ORDERS_IDX" -eq -1 ]; then
    echo "WARNING: No write_file call - how will you fix the bug?"
    # Not a failure, but let's require it for this exercise
    echo "ERROR: Must write to orders.py to fix the bug"
    exit 1
fi

echo "OK: Tool sequence is valid"
echo "  - Read orders.py at step $((READ_ORDERS_IDX + 1))"
echo "  - Write orders.py at step $((WRITE_ORDERS_IDX + 1))"
if [ "$LAST_TEST_IDX" -ne -1 ]; then
    echo "  - Run tests at step $((LAST_TEST_IDX + 1))"
fi
exit 0

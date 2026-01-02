#!/bin/bash
# Check that error handling patterns are properly implemented
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

# Track error handling coverage
HAS_RETRY=false
HAS_FALLBACK=false
ALL_HAVE_ERROR_HANDLING=true

# Check each tool call for error handling
IDX=0
while [ $IDX -lt "$CALL_COUNT" ]; do
    TOOL_NAME=$(echo "$INPUT" | jq -r ".[$IDX].name // empty")
    ON_ERROR=$(echo "$INPUT" | jq ".[$IDX].on_error // empty")

    if [ -z "$ON_ERROR" ] || [ "$ON_ERROR" = "null" ] || [ "$ON_ERROR" = "" ]; then
        echo "WARNING: Tool call '$TOOL_NAME' at index $IDX has no error handling"
        ALL_HAVE_ERROR_HANDLING=false
    else
        # Check for retry configuration
        RETRY=$(echo "$ON_ERROR" | jq '.retry // empty')
        if [ -n "$RETRY" ] && [ "$RETRY" != "null" ] && [ "$RETRY" != "" ]; then
            HAS_RETRY=true

            # Validate retry configuration
            MAX_ATTEMPTS=$(echo "$RETRY" | jq '.max_attempts // 0')
            if [ "$MAX_ATTEMPTS" -lt 1 ] || [ "$MAX_ATTEMPTS" -gt 10 ]; then
                echo "ERROR: Tool '$TOOL_NAME' has invalid max_attempts (should be 1-10)"
                exit 1
            fi

            BACKOFF=$(echo "$RETRY" | jq -r '.backoff // empty')
            if [ -n "$BACKOFF" ]; then
                case "$BACKOFF" in
                    exponential|linear|constant)
                        ;;
                    *)
                        echo "ERROR: Tool '$TOOL_NAME' has invalid backoff strategy: $BACKOFF"
                        exit 1
                        ;;
                esac
            fi
        fi

        # Check for fallback configuration
        FALLBACK=$(echo "$ON_ERROR" | jq '.fallback // empty')
        if [ -n "$FALLBACK" ] && [ "$FALLBACK" != "null" ] && [ "$FALLBACK" != "" ]; then
            HAS_FALLBACK=true
        fi

        # Check for fail_immediately
        FAIL_IMMEDIATELY=$(echo "$ON_ERROR" | jq '.fail_immediately // false')
        if [ "$FAIL_IMMEDIATELY" = "true" ]; then
            # This is acceptable for unrecoverable errors
            :
        fi
    fi

    IDX=$((IDX + 1))
done

# Validate overall error handling strategy
if [ "$ALL_HAVE_ERROR_HANDLING" = false ]; then
    echo "ERROR: Not all tool calls have error handling defined"
    exit 1
fi

if [ "$HAS_RETRY" = false ]; then
    echo "ERROR: No retry strategy defined for any tool call"
    echo "HINT: Transient errors like SERVICE_UNAVAILABLE should be retried"
    exit 1
fi

if [ "$HAS_FALLBACK" = false ]; then
    echo "WARNING: No fallback strategy defined"
    echo "HINT: Consider what happens if a tool completely fails"
    # Not a failure, but strongly recommended
fi

echo "OK: Error handling patterns are properly implemented"
echo "  - Retry strategies: present"
if [ "$HAS_FALLBACK" = true ]; then
    echo "  - Fallback strategies: present"
fi
echo "  - All tool calls have error handling"
exit 0

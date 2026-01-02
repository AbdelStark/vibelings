#!/bin/bash
# Check that proper observability patterns are implemented
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

# Track observability coverage
ALL_HAVE_TRACE=true
ALL_HAVE_TIMING=true
ALL_HAVE_COST=true
TRACE_ID=""
SPAN_IDS=""

# Check each tool call for observability metadata
IDX=0
while [ $IDX -lt "$CALL_COUNT" ]; do
    TOOL_NAME=$(echo "$INPUT" | jq -r ".[$IDX].name // empty")

    # Check for trace metadata
    TRACE=$(echo "$INPUT" | jq ".[$IDX].trace // empty")
    if [ -z "$TRACE" ] || [ "$TRACE" = "null" ] || [ "$TRACE" = "" ]; then
        echo "ERROR: Tool call '$TOOL_NAME' at index $IDX missing trace metadata"
        ALL_HAVE_TRACE=false
    else
        # Validate trace structure
        CURRENT_TRACE_ID=$(echo "$TRACE" | jq -r '.trace_id // empty')
        SPAN_ID=$(echo "$TRACE" | jq -r '.span_id // empty')
        OPERATION=$(echo "$TRACE" | jq -r '.operation // empty')

        if [ -z "$CURRENT_TRACE_ID" ]; then
            echo "ERROR: Tool '$TOOL_NAME' missing trace_id"
            ALL_HAVE_TRACE=false
        elif [ -z "$TRACE_ID" ]; then
            TRACE_ID="$CURRENT_TRACE_ID"
        elif [ "$CURRENT_TRACE_ID" != "$TRACE_ID" ]; then
            echo "ERROR: Tool '$TOOL_NAME' has different trace_id ($CURRENT_TRACE_ID vs $TRACE_ID)"
            echo "HINT: All operations in a workflow should share the same trace_id"
            ALL_HAVE_TRACE=false
        fi

        if [ -z "$SPAN_ID" ]; then
            echo "ERROR: Tool '$TOOL_NAME' missing span_id"
            ALL_HAVE_TRACE=false
        fi

        if [ -z "$OPERATION" ]; then
            echo "WARNING: Tool '$TOOL_NAME' missing operation name"
        fi
    fi

    # Check for timing metadata
    TIMING=$(echo "$INPUT" | jq ".[$IDX].timing // empty")
    if [ -z "$TIMING" ] || [ "$TIMING" = "null" ] || [ "$TIMING" = "" ]; then
        echo "ERROR: Tool call '$TOOL_NAME' at index $IDX missing timing metadata"
        ALL_HAVE_TIMING=false
    else
        EST_DURATION=$(echo "$TIMING" | jq '.estimated_duration_ms // 0')
        TIMEOUT=$(echo "$TIMING" | jq '.timeout_ms // 0')

        if [ "$EST_DURATION" -le 0 ]; then
            echo "ERROR: Tool '$TOOL_NAME' has invalid estimated_duration_ms"
            ALL_HAVE_TIMING=false
        fi

        if [ "$TIMEOUT" -le 0 ]; then
            echo "ERROR: Tool '$TOOL_NAME' has invalid timeout_ms"
            ALL_HAVE_TIMING=false
        fi

        if [ "$EST_DURATION" -gt "$TIMEOUT" ]; then
            echo "WARNING: Tool '$TOOL_NAME' estimated_duration exceeds timeout"
        fi
    fi

    # Check for cost metadata
    COST=$(echo "$INPUT" | jq ".[$IDX].cost // empty")
    if [ -z "$COST" ] || [ "$COST" = "null" ] || [ "$COST" = "" ]; then
        echo "ERROR: Tool call '$TOOL_NAME' at index $IDX missing cost metadata"
        ALL_HAVE_COST=false
    else
        EST_COST=$(echo "$COST" | jq '.estimated_usd // 0')
        FACTORS=$(echo "$COST" | jq '.cost_factors // []')

        if echo "$EST_COST" | grep -qE '^[0-9.]+$'; then
            : # Valid number
        else
            echo "ERROR: Tool '$TOOL_NAME' has invalid estimated_usd"
            ALL_HAVE_COST=false
        fi

        FACTOR_COUNT=$(echo "$FACTORS" | jq 'length')
        if [ "$FACTOR_COUNT" -eq 0 ]; then
            echo "WARNING: Tool '$TOOL_NAME' has no cost_factors documented"
        fi
    fi

    IDX=$((IDX + 1))
done

# Final validation
if [ "$ALL_HAVE_TRACE" = false ]; then
    echo "ERROR: Not all tool calls have proper trace metadata"
    exit 1
fi

if [ "$ALL_HAVE_TIMING" = false ]; then
    echo "ERROR: Not all tool calls have proper timing metadata"
    exit 1
fi

if [ "$ALL_HAVE_COST" = false ]; then
    echo "ERROR: Not all tool calls have proper cost metadata"
    exit 1
fi

echo "OK: Observability patterns properly implemented"
echo "  - All operations share trace_id: $TRACE_ID"
echo "  - All operations have timing estimates"
echo "  - All operations have cost estimates"
exit 0

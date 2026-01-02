# Handling Tool Failures

## Goal

Demonstrate proper error handling patterns when tools fail, including
retry strategies and graceful degradation with fallback approaches.

## Why This Matters

In production agentic systems, tools fail. APIs return errors. Services time out.
Rate limits kick in. A reliable agent must:

1. **Anticipate failures** - Not assume every call succeeds
2. **Implement retries** - With exponential backoff for transient errors
3. **Have fallbacks** - Alternative approaches when primary fails
4. **Degrade gracefully** - Provide partial results rather than complete failure

This exercise teaches the error handling discipline that separates toy demos
from production-ready agents.

## The Task

You are building a data aggregation agent. Your task is to fetch user profile
data from multiple sources and combine it.

**Task**: "Get the complete user profile for user_id 12345, including their
preferences and recent activity."

Some of these tools may fail. Your output must demonstrate proper error handling.

## Available Tools

### `get_user_profile`
Get basic user profile data.

**Parameters**:
- `user_id` (string, required): The user ID

**Possible Errors**:
- `USER_NOT_FOUND` - User doesn't exist
- `SERVICE_UNAVAILABLE` - Temporary outage

### `get_user_preferences`
Get user preferences and settings.

**Parameters**:
- `user_id` (string, required): The user ID

**Possible Errors**:
- `PREFERENCES_NOT_SET` - User has no preferences
- `RATE_LIMITED` - Too many requests

### `get_recent_activity`
Get user's recent activity feed.

**Parameters**:
- `user_id` (string, required): The user ID
- `limit` (integer, optional): Max items to return (default: 10)

**Possible Errors**:
- `TIMEOUT` - Request timed out
- `RATE_LIMITED` - Too many requests

## Expected Output Format

Your output must show how you would handle errors:

```json
{
  "tool_calls": [
    {
      "name": "tool_name",
      "arguments": {...},
      "on_error": {
        "retry": {
          "max_attempts": 3,
          "backoff": "exponential"
        },
        "fallback": "description of fallback behavior"
      }
    }
  ],
  "error_handling_strategy": "description of overall approach"
}
```

## Error Handling Patterns

### Pattern 1: Retry with Backoff
For transient errors (SERVICE_UNAVAILABLE, TIMEOUT):
```json
{
  "on_error": {
    "retry": {
      "max_attempts": 3,
      "backoff": "exponential",
      "retryable_errors": ["SERVICE_UNAVAILABLE", "TIMEOUT"]
    }
  }
}
```

### Pattern 2: Graceful Fallback
For non-retryable errors or when retries exhaust:
```json
{
  "on_error": {
    "fallback": "Return partial result without preferences",
    "continue": true
  }
}
```

### Pattern 3: Fail Fast
For unrecoverable errors (USER_NOT_FOUND):
```json
{
  "on_error": {
    "fail_immediately": true,
    "reason": "Cannot continue without valid user"
  }
}
```

## Grading

Your output is validated for:

1. **Tool schema compliance** - All tool calls have valid arguments
2. **Error handling presence** - Each tool call addresses potential failures
3. **Appropriate strategies** - Right pattern for each error type
4. **Overall coherence** - Strategy matches the task requirements

## Key Lessons

1. **Transient vs permanent errors**: Retry transient ones, fail fast on permanent
2. **Exponential backoff**: Don't hammer a failing service
3. **Partial success**: Better than complete failure
4. **Circuit breakers**: Know when to stop trying
5. **Error context**: Preserve information about what failed and why

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about which errors are retryable
- Consider what happens if a tool fails completely
- Always have a plan for partial results

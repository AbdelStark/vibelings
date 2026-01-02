# Handling Tool Failures

## Goal

Demonstrate proper error handling patterns when tools fail, including
retry strategies and graceful degradation with fallback approaches.

## Why This Matters

In production agentic systems, tools fail. APIs return errors. Services time out.
Rate limits kick in. A reliable agent must:

1. **Anticipate failures** — Not assume every call succeeds
2. **Implement retries** — With exponential backoff for transient errors
3. **Have fallbacks** — Alternative approaches when primary fails
4. **Degrade gracefully** — Provide partial results rather than complete failure

This exercise teaches the error handling discipline that separates toy demos
from production-ready agents.

## The Concept: Error Classification

Not all errors are equal. The first step in handling errors is classifying them:

```
                            ┌─────────────────┐
                            │   Error Occurs  │
                            └────────┬────────┘
                                     │
                       ┌─────────────┴─────────────┐
                       ▼                           ▼
              ┌────────────────┐         ┌────────────────┐
              │  Is it our     │   No    │  External      │
              │  fault?        │────────►│  system error  │
              └───────┬────────┘         └───────┬────────┘
                      │ Yes                      │
                      ▼                          ▼
              ┌────────────────┐    ┌────────────────────────────┐
              │  Fix code/     │    │  Will retry help?          │
              │  config        │    └──────────┬─────────────────┘
              └────────────────┘               │
                                    ┌──────────┴──────────┐
                                    ▼                     ▼
                             ┌───────────┐         ┌───────────┐
                             │ TRANSIENT │         │ PERMANENT │
                             │ Retry     │         │ Fail fast │
                             └───────────┘         └───────────┘
```

| Error Type | Examples | Strategy |
|------------|----------|----------|
| **Transient** | TIMEOUT, SERVICE_UNAVAILABLE, RATE_LIMITED | Retry with backoff |
| **Permanent** | USER_NOT_FOUND, INVALID_INPUT | Fail fast or fallback |
| **Expected** | PREFERENCES_NOT_SET | Handle as valid empty state |

**The key insight**: Transient errors are *temporary* — the same request might succeed later. Permanent errors are *deterministic* — retrying won't help.

### Exponential Backoff

When retrying, don't hammer the service:

```
Time ──────────────────────────────────────────────────────────────►

    ┃ Attempt 1        ┃ Attempt 2        ┃ Attempt 3        ┃ Attempt 4
    ┃                  ┃                  ┃                  ┃
    ▼                  ▼                  ▼                  ▼
────●──────────────────●──────────────────●──────────────────●────────
    │     wait 1s      │     wait 2s      │     wait 4s      │
    │◄────────────────►│◄────────────────►│◄────────────────►│
                           ▲                   ▲
                     Doubles each time   Exponential growth
```

**Bad pattern (linear/immediate)**:
```
────●──●──●──●──●──●──●──●──●──●────► Hammers the service!
```

**Good pattern (exponential + jitter)**:
```
────●────────●──────────────●────────────────────────●──────────────►
              + random delay prevents thundering herd
```

This gives the service time to recover. Linear or immediate retries often make things worse.

### Graceful Degradation

When tools fail, you have choices:
1. **Complete failure** — Stop everything (sometimes correct)
2. **Partial success** — Return what you could get (often better)
3. **Fallback value** — Use a default or cached value

The right choice depends on the task. Missing preferences? Use defaults. Missing the user entirely? That's fatal.

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
- `USER_NOT_FOUND` — User doesn't exist (permanent)
- `SERVICE_UNAVAILABLE` — Temporary outage (transient)

### `get_user_preferences`
Get user preferences and settings.

**Parameters**:
- `user_id` (string, required): The user ID

**Possible Errors**:
- `PREFERENCES_NOT_SET` — User has no preferences (expected state)
- `RATE_LIMITED` — Too many requests (transient)

### `get_recent_activity`
Get user's recent activity feed.

**Parameters**:
- `user_id` (string, required): The user ID
- `limit` (integer, optional): Max items to return (default: 10)

**Possible Errors**:
- `TIMEOUT` — Request timed out (transient)
- `RATE_LIMITED` — Too many requests (transient)

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

## Common Mistakes

**1. Retrying permanent errors**
```json
{"retry": {"retryable_errors": ["USER_NOT_FOUND"]}}  // Wrong
```
USER_NOT_FOUND won't resolve with retries. You're just wasting time and quota.

**2. Immediate retries without backoff**
```json
{"retry": {"max_attempts": 10, "backoff": "none"}}  // Wrong
```
This hammers a struggling service and makes outages worse.

**3. No fallback for optional data**
If preferences fail, use empty/default. Don't fail the entire request.

**4. Failing on expected states**
PREFERENCES_NOT_SET isn't an error—it's a valid state. Handle it as empty data, not a failure.

**5. Inconsistent error handling**
Every tool needs error handling, not just the "risky" ones. All tools can fail.

## Grading

Your output is validated for:

1. **Tool schema compliance** — All tool calls have valid arguments
2. **Error handling presence** — Each tool call addresses potential failures
3. **Appropriate strategies** — Right pattern for each error type
4. **Overall coherence** — Strategy matches the task requirements

## Key Lessons

1. **Transient vs permanent errors**: Retry transient ones, fail fast on permanent
2. **Exponential backoff**: Don't hammer a failing service
3. **Partial success**: Better than complete failure
4. **Circuit breakers**: Know when to stop trying
5. **Error context**: Preserve information about what failed and why

**The mental model**: Think of error handling as a decision tree. Each error type branches to a different strategy. The tree should be explicit, not implicit.

## Connections

- **Prerequisite**: [tools_01](../tools_01/) introduces basic tool calling
- **Related**: [guardrails_01](../guardrails_01/) handles input validation errors
- **Advanced**: [production_eval_01](../../production/production_eval_01/) tests error handling systematically
- **Production**: Error handling patterns appear in [mcp/client_01](../../mcp/client_01/) and [workflows](../../workflows/)

## Further Reading

- [Release It!](https://pragprog.com/titles/mnee2/release-it-second-edition/) — Chapter on stability patterns (circuit breakers, timeouts, bulkheads)
- [Exponential backoff and jitter](https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/) — AWS best practices
- [Resilience4j](https://resilience4j.readme.io/) — Fault tolerance library patterns (applicable concepts)

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about which errors are retryable (transient) vs not (permanent)
- Consider what happens if a tool fails completely—can you proceed?
- Always have a plan for partial results
- PREFERENCES_NOT_SET is an expected state, not an error

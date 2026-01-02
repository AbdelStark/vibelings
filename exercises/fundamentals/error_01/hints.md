# Hint 1: Categorize Errors

First, categorize each error type:

**Retryable (transient)**:
- SERVICE_UNAVAILABLE
- TIMEOUT
- RATE_LIMITED (with backoff)

**Non-retryable (permanent)**:
- USER_NOT_FOUND
- PREFERENCES_NOT_SET (expected case, not error)

---

# Hint 2: Error Handling Structure

Each tool call needs an `on_error` object. Example:

```json
{
  "name": "get_user_profile",
  "arguments": {"user_id": "12345"},
  "on_error": {
    "retry": {...},
    "fallback": "..."
  }
}
```

---

# Hint 3: Retry Configuration

For retryable errors, use exponential backoff:

```json
"retry": {
  "max_attempts": 3,
  "backoff": "exponential",
  "retryable_errors": ["SERVICE_UNAVAILABLE", "TIMEOUT"]
}
```

---

# Hint 4: Fallback Strategies

Think about what to do when all retries fail:

- `get_user_profile`: Must succeed (fail_immediately if USER_NOT_FOUND)
- `get_user_preferences`: Can return empty/default preferences
- `get_recent_activity`: Can return empty activity list

---

# Hint 5: Complete Solution

```json
{
  "tool_calls": [
    {
      "name": "get_user_profile",
      "arguments": {"user_id": "12345"},
      "on_error": {
        "retry": {
          "max_attempts": 3,
          "backoff": "exponential",
          "retryable_errors": ["SERVICE_UNAVAILABLE"]
        },
        "fail_immediately": true,
        "fail_on": ["USER_NOT_FOUND"],
        "reason": "Cannot continue without valid user"
      }
    },
    {
      "name": "get_user_preferences",
      "arguments": {"user_id": "12345"},
      "on_error": {
        "retry": {
          "max_attempts": 2,
          "backoff": "exponential",
          "retryable_errors": ["RATE_LIMITED"]
        },
        "fallback": "Return empty preferences object if unavailable"
      }
    },
    {
      "name": "get_recent_activity",
      "arguments": {"user_id": "12345", "limit": 10},
      "on_error": {
        "retry": {
          "max_attempts": 3,
          "backoff": "exponential",
          "retryable_errors": ["TIMEOUT", "RATE_LIMITED"]
        },
        "fallback": "Return empty activity list if unavailable"
      }
    }
  ],
  "error_handling_strategy": "Fail fast on missing user, retry transient errors with exponential backoff, degrade gracefully for optional data (preferences, activity)"
}
```

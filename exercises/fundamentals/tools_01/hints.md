# Hint 1: Output Format

Your output must be valid JSON with this structure:

```json
{
  "tool_calls": [...]
}
```

Make sure there's no text before or after the JSON.

---

# Hint 2: Tool Selection

The query asks about current weather, not a forecast.
Which tool is designed for current weather conditions?

---

# Hint 3: Required Parameters

Look at the `get_weather` tool's parameters:
- `location` is required
- `units` is optional

What location is the user asking about?

---

# Hint 4: Complete Solution

For "What's the weather like in San Francisco?", you need:

```json
{
  "tool_calls": [
    {
      "name": "get_weather",
      "arguments": {
        "location": "San Francisco"
      }
    }
  ]
}
```

Note: `units` can be omitted since it has a default value, or you can explicitly set it to "fahrenheit".

# Basic Tool Calling

## Goal

Generate a JSON output that demonstrates correct tool calling patterns by specifying
the exact tool calls needed to complete a task.

## Why This Matters

Tool calling is the foundation of agentic systems. An LLM doesn't have real-world
capabilities on its own - it needs tools to read files, query databases, call APIs,
and interact with systems. The reliability of an agentic system depends on:

1. **Correct tool selection** - choosing the right tool for the job
2. **Valid arguments** - passing properly structured data to the tool
3. **Schema compliance** - ensuring arguments match the tool's contract

This exercise teaches you to think about tools as contracts with strict schemas,
not as fuzzy natural language interfaces.

## The Task

You are building a weather assistant. Given a user query about the weather,
output the correct tool call(s) to fulfill the request.

**User Query**: "What's the weather like in San Francisco?"

## Available Tools

### `get_weather`
Retrieves current weather for a location.

**Parameters**:
- `location` (string, required): City name, optionally with state/country
- `units` (string, optional): "celsius" or "fahrenheit" (default: "fahrenheit")

### `get_forecast`
Retrieves a multi-day weather forecast.

**Parameters**:
- `location` (string, required): City name
- `days` (integer, required): Number of days (1-7)
- `units` (string, optional): "celsius" or "fahrenheit"

## Expected Output Format

Your output must be a JSON object with a `tool_calls` array:

```json
{
  "tool_calls": [
    {
      "name": "tool_name",
      "arguments": {
        "param1": "value1"
      }
    }
  ]
}
```

## Example Valid Output

For the query "What's the current weather in Tokyo?":

```json
{
  "tool_calls": [
    {
      "name": "get_weather",
      "arguments": {
        "location": "Tokyo",
        "units": "celsius"
      }
    }
  ]
}
```

## Grading

Your output is validated against:

1. **Structure** - Must have a `tool_calls` array
2. **Tool validity** - Each tool must be from the available tools list
3. **Schema compliance** - Arguments must match the tool's parameter schema
4. **Completeness** - All required parameters must be present

## Key Lesson

**Tools are contracts, not suggestions.** When an LLM calls a tool, it must:
- Use the exact tool name defined in the schema
- Provide all required parameters
- Use correct types for each parameter
- Stay within defined constraints (enums, ranges, patterns)

This deterministic validation is what makes agentic systems reliable.
Random hallucinated tool names or malformed arguments cause runtime failures.

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Check `grader/tools_schema.json` for exact parameter definitions
- Remember: only call tools that are actually needed for the query

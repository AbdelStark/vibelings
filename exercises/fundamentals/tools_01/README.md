# Basic Tool Calling

## Goal

Generate a JSON output that demonstrates correct tool calling patterns by specifying
the exact tool calls needed to complete a task.

## Why This Matters

Tool calling is the foundation of agentic systems. An LLM doesn't have real-world
capabilities on its own — it needs tools to read files, query databases, call APIs,
and interact with systems. The reliability of an agentic system depends on:

1. **Correct tool selection** — choosing the right tool for the job
2. **Valid arguments** — passing properly structured data to the tool
3. **Schema compliance** — ensuring arguments match the tool's contract

Without these, you get: hallucinated tool names that don't exist, wrong parameter types that crash execution, missing required fields that cause silent failures. Tool calling discipline is what separates working agents from demos that fail in production.

## The Concept: Tools as Typed Functions

Think of tools as functions with strict type signatures:

```python
# Not this (loose)
def get_weather(location, **kwargs):
    ...

# This (strict)
def get_weather(
    location: str,           # Required
    units: Literal["celsius", "fahrenheit"] = "fahrenheit"  # Optional with default
) -> WeatherData:
    ...
```

The tool schema is the contract. It defines:
- **What tools exist** (you can't call tools that aren't defined)
- **What arguments each accepts** (types, constraints, required vs optional)
- **What happens on violation** (validation error, not silent failure)

This is why tool calling isn't "prompt engineering" — it's API design. The same principles that make REST APIs reliable apply here: explicit contracts, schema validation, predictable errors.

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

## Common Mistakes

**1. Hallucinated tool names**
```json
{"name": "check_weather"}     // Wrong: tool doesn't exist
{"name": "getWeather"}        // Wrong: camelCase vs snake_case
{"name": "get_weather"}       // Correct
```
LLMs sometimes invent plausible-sounding tool names. Only use tools that are explicitly defined.

**2. Missing required parameters**
```json
{"name": "get_forecast", "arguments": {"location": "NYC"}}  // Wrong: missing "days"
```
Optional parameters can be omitted; required parameters cannot.

**3. Wrong parameter types**
```json
{"days": "7"}    // Wrong: string
{"days": 7}      // Correct: integer
```

**4. Invalid enum values**
```json
{"units": "Celsius"}     // Wrong: capitalization
{"units": "kelvin"}      // Wrong: not in enum
{"units": "celsius"}     // Correct
```

**5. Calling unnecessary tools**
The user asked about current weather, not a forecast. Don't call `get_forecast` when `get_weather` suffices.

## Grading

Your output is validated against:

1. **Structure** — Must have a `tool_calls` array
2. **Tool validity** — Each tool must be from the available tools list
3. **Schema compliance** — Arguments must match the tool's parameter schema
4. **Completeness** — All required parameters must be present

## Key Lesson

**Tools are contracts, not suggestions.** When an LLM calls a tool, it must:
- Use the exact tool name defined in the schema
- Provide all required parameters
- Use correct types for each parameter
- Stay within defined constraints (enums, ranges, patterns)

This deterministic validation is what makes agentic systems reliable. A tool schema is a machine-readable specification that can be validated automatically. No ambiguity, no interpretation, no "close enough."

**The practical implication**: When you design tools, make the schema as strict as practical. Require fields that are always needed. Use enums instead of free-form strings. Set min/max on numeric ranges. The stricter the schema, the more errors you catch before execution.

## Connections

- **Prerequisite**: [json_01](../json_01/) introduces schema validation
- **Next**: [tools_02](../tools_02/) covers multi-tool orchestration
- **Related**: [mcp/server_01](../../mcp/server_01/) defines tools in MCP format
- **Advanced**: [error_01](../error_01/) handles tool call failures

## Further Reading

- [Anthropic: Tool use](https://docs.anthropic.com/en/docs/build-with-claude/tool-use) — Official Claude tool calling guide
- [OpenAI: Function calling](https://platform.openai.com/docs/guides/function-calling) — OpenAI's implementation
- [JSON Schema for tool parameters](https://json-schema.org/understanding-json-schema/) — The underlying validation language

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Check `grader/tools_schema.json` for exact parameter definitions
- Remember: only call tools that are actually needed for the query
- "Current weather" means `get_weather`, not `get_forecast`

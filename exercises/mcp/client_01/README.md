# MCP Tool Call Request

## Goal

Construct a valid MCP tool call request using the JSON-RPC 2.0 format. This is
how clients invoke tools exposed by MCP servers.

## Why This Matters

After learning to define MCP tools (server_01), the next step is understanding how
clients call those tools. MCP uses JSON-RPC 2.0 as its transport protocol, which means:

1. **Structured requests** - Every request has a defined format with id, method, and params
2. **Correlatable responses** - Request IDs enable matching responses to requests
3. **Standard error handling** - Errors follow a predictable format

Understanding the request format is essential for building MCP clients that correctly
invoke server capabilities.

## The Task

Given the `calculate_area` tool from the previous exercise, construct a valid
MCP request to calculate the area of a circle with radius 5.

## MCP Request Format (JSON-RPC 2.0)

An MCP tool call uses the `tools/call` method:

```json
{
  "jsonrpc": "2.0",
  "id": "request-id",
  "method": "tools/call",
  "params": {
    "name": "tool_name",
    "arguments": {
      "param1": "value1"
    }
  }
}
```

Key fields:
- `jsonrpc`: Must be exactly "2.0"
- `id`: A unique identifier for this request (string or number)
- `method`: Must be "tools/call" for tool invocations
- `params.name`: The tool to call
- `params.arguments`: Arguments to pass to the tool

## Expected Output Format

Create a JSON-RPC request to calculate the area of a circle with radius 5:

```json
{
  "jsonrpc": "2.0",
  "id": "calc-001",
  "method": "tools/call",
  "params": {
    "name": "calculate_area",
    "arguments": {
      "shape": "circle",
      "radius": 5
    }
  }
}
```

## Grading

Your output is validated against:

1. **JSON-RPC version** - Must be exactly "2.0"
2. **Request ID** - Must be present (string or integer)
3. **Method** - Must be exactly "tools/call"
4. **Tool name** - Must be "calculate_area" in params
5. **Arguments** - Must include shape="circle" and radius (a positive number)

## Key Lesson

**JSON-RPC provides structure for tool invocation.** The protocol ensures:
- Requests can be uniquely identified for correlation
- Methods are well-defined (tools/call, tools/list, etc.)
- Arguments are structured and validatable

This standardization is what makes MCP interoperable - any MCP client can call
any MCP server's tools using the same request format.

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- The id can be any string or number - pick something meaningful
- Remember: shape must be "circle" for this request
- radius must be a number, not a string

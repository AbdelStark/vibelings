# MCP Tool Call Request

## Goal

Construct a valid MCP tool call request using the JSON-RPC 2.0 format. This is
how clients invoke tools exposed by MCP servers.

## Why This Matters

After learning to define MCP tools (server_01), the next step is understanding how
clients call those tools. MCP uses JSON-RPC 2.0 as its transport protocol, which means:

1. **Structured requests** — Every request has a defined format with id, method, and params
2. **Correlatable responses** — Request IDs enable matching responses to requests
3. **Standard error handling** — Errors follow a predictable format

Understanding the request format is essential for building MCP clients that correctly
invoke server capabilities.

## The Concept: JSON-RPC 2.0

JSON-RPC is a lightweight remote procedure call protocol. It's like HTTP for function calls:

```
Client                              Server
  │                                   │
  │── Request (id: "abc") ──────────►│
  │   method: "tools/call"            │
  │   params: {...}                   │
  │                                   │
  │◄── Response (id: "abc") ─────────│
  │   result: {...}                   │
  │                                   │
```

The `id` field is crucial: it lets you match responses to requests, especially when multiple requests are in-flight. Without it, you wouldn't know which response belongs to which request.

### MCP Methods

MCP defines several standard methods:

| Method | Purpose |
|--------|---------|
| `tools/list` | Discover available tools |
| `tools/call` | Invoke a tool |
| `resources/list` | Discover available resources |
| `resources/read` | Fetch resource contents |
| `prompts/list` | Discover prompt templates |

This exercise focuses on `tools/call`.

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

## Common Mistakes

**1. Wrong JSON-RPC version**
```json
{"jsonrpc": "1.0"}  // Wrong
{"jsonrpc": 2.0}    // Wrong: must be string
{"jsonrpc": "2.0"}  // Correct
```

**2. Missing request ID**
```json
{"method": "tools/call", "params": {...}}  // Wrong: no id
```
The `id` is required for request/response correlation.

**3. Wrong method for tool calls**
```json
{"method": "call"}           // Wrong
{"method": "tool/call"}      // Wrong: missing 's'
{"method": "tools/call"}     // Correct
```

**4. Arguments as string instead of number**
```json
{"radius": "5"}   // Wrong: string
{"radius": 5}     // Correct: number
```

## Grading

Your output is validated against:

1. **JSON-RPC version** — Must be exactly "2.0"
2. **Request ID** — Must be present (string or integer)
3. **Method** — Must be exactly "tools/call"
4. **Tool name** — Must be "calculate_area" in params
5. **Arguments** — Must include shape="circle" and radius (a positive number)

## Key Lesson

**JSON-RPC provides structure for tool invocation.** The protocol ensures:
- Requests can be uniquely identified for correlation
- Methods are well-defined (tools/call, tools/list, etc.)
- Arguments are structured and validatable

This standardization is what makes MCP interoperable — any MCP client can call
any MCP server's tools using the same request format. No custom HTTP endpoints, no provider-specific SDKs.

## Connections

- **Prerequisite**: [server_01](../server_01/) defines the tool being called
- **Next**: [resource_01](../resource_01/) covers MCP resources
- **Related**: [fundamentals/error_01](../../fundamentals/error_01/) — JSON-RPC has standard error handling

## Further Reading

- [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification) — The underlying protocol
- [MCP Specification: Transport](https://spec.modelcontextprotocol.io/) — How MCP uses JSON-RPC
- [MCP Python SDK](https://github.com/modelcontextprotocol/python-sdk) — Python implementation

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- The id can be any string or number — pick something meaningful
- Remember: shape must be "circle" for this request
- radius must be a number, not a string

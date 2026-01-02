# MCP Tool Definition

## Goal

Define a tool according to the Model Context Protocol (MCP) specification. This is the
foundation for building MCP servers that expose capabilities to AI agents.

## Why This Matters

The Model Context Protocol (MCP) is an open standard for connecting AI models to external
tools and data sources. Unlike proprietary tool calling formats, MCP provides:

1. **Standardized tool definitions** - A common schema that works across providers
2. **JSON-RPC transport** - Reliable request/response semantics
3. **Discoverability** - Tools can be listed and described programmatically
4. **Security boundaries** - Clear capability declarations

Understanding MCP tool definitions is essential for building interoperable agentic systems.

## The Task

Define an MCP tool that calculates the area of geometric shapes. Your tool definition
must follow the MCP specification exactly.

**Tool Name**: `calculate_area`

**Purpose**: Calculate the area of a circle, rectangle, or triangle.

## MCP Tool Schema

An MCP tool definition has this structure:

```json
{
  "name": "tool_name",
  "description": "What the tool does",
  "inputSchema": {
    "type": "object",
    "properties": {
      "param1": { "type": "string", "description": "..." }
    },
    "required": ["param1"]
  }
}
```

Note the key differences from OpenAI-style tool definitions:
- Uses `inputSchema` (not `parameters`)
- Follows JSON Schema draft-07
- No `additionalProperties` by default in MCP

## Expected Output Format

Your output must be a valid MCP tool definition as a JSON object:

```json
{
  "name": "calculate_area",
  "description": "Calculate the area of a geometric shape",
  "inputSchema": {
    "type": "object",
    "properties": {
      "shape": {
        "type": "string",
        "enum": ["circle", "rectangle", "triangle"],
        "description": "The type of shape"
      },
      ...additional parameters based on shape...
    },
    "required": ["shape"]
  }
}
```

## Required Parameters

Your tool must accept these parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `shape` | string | Yes | Shape type: "circle", "rectangle", or "triangle" |
| `radius` | number | For circle | The radius of the circle |
| `width` | number | For rectangle | Width of the rectangle |
| `height` | number | For rectangle/triangle | Height of the shape |
| `base` | number | For triangle | Base of the triangle |

## Grading

Your output is validated against:

1. **Structure** - Must have `name`, `description`, and `inputSchema`
2. **Name** - Must be exactly "calculate_area"
3. **Input Schema** - Must define `shape` enum with correct values
4. **Required fields** - Must declare `shape` as required
5. **Property types** - All numeric parameters must be type "number"

## Key Lesson

**MCP tools are self-describing contracts.** When you define an MCP tool:
- The `inputSchema` tells clients exactly what inputs are accepted
- The `description` explains what the tool does in natural language
- Required fields are explicitly declared
- Types are enforced by the protocol

This makes MCP tools discoverable and validatable - a client can inspect
available tools and ensure it's calling them correctly before execution.

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Check the MCP specification at https://spec.modelcontextprotocol.io/
- Remember: `inputSchema` not `parameters`
- All numeric values should be type "number", not "integer"

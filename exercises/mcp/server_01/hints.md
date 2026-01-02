# Hints for MCP Tool Definition

## Hint 1: Structure
The basic structure of an MCP tool is:
```json
{
  "name": "...",
  "description": "...",
  "inputSchema": { ... }
}
```

## Hint 2: inputSchema vs parameters
MCP uses `inputSchema`, not `parameters`. This is a key difference from OpenAI-style tools.

## Hint 3: Shape enum
The shape parameter should be defined as:
```json
"shape": {
  "type": "string",
  "enum": ["circle", "rectangle", "triangle"],
  "description": "The type of geometric shape"
}
```

## Hint 4: Required array
The `required` field in inputSchema is an array of parameter names:
```json
"required": ["shape"]
```

## Hint 5: Complete example
A complete MCP tool definition looks like:
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
        "description": "The type of shape to calculate"
      },
      "radius": {
        "type": "number",
        "description": "Radius for circle area calculation"
      },
      "width": {
        "type": "number",
        "description": "Width for rectangle area calculation"
      },
      "height": {
        "type": "number",
        "description": "Height for rectangle or triangle"
      },
      "base": {
        "type": "number",
        "description": "Base for triangle area calculation"
      }
    },
    "required": ["shape"]
  }
}
```

# Hints for MCP Tool Call Request

## Hint 1: JSON-RPC Structure
Every JSON-RPC 2.0 request has four key fields:
- jsonrpc (version)
- id (request identifier)
- method (what to call)
- params (call arguments)

## Hint 2: Version String
The jsonrpc field must be exactly "2.0" (as a string, not a number):
```json
"jsonrpc": "2.0"
```

## Hint 3: Method Name
For calling MCP tools, the method is always "tools/call":
```json
"method": "tools/call"
```

## Hint 4: Params Structure
The params object contains the tool name and its arguments:
```json
"params": {
  "name": "calculate_area",
  "arguments": { ... }
}
```

## Hint 5: Complete Example
A complete MCP tool call request:
```json
{
  "jsonrpc": "2.0",
  "id": "request-1",
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

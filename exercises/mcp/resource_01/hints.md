# Hints for MCP Resource Definition

---

## Hint 1: Structure

An MCP resource has exactly four required fields:
- `uri` - The resource identifier
- `name` - Human-readable label
- `description` - What the resource contains
- `mimeType` - Content type for parsing

---

## Hint 2: URI Format

The URI must be exactly `user://profile/current`. MCP allows custom URI schemes
beyond http/https. The format is `scheme://path`.

---

## Hint 3: MIME Type

For JSON data, use `application/json` as the mimeType. This tells clients
how to parse the resource content when they fetch it.

---

## Hint 4: Complete Example

Here's a complete valid solution:

```json
{
  "uri": "user://profile/current",
  "name": "Current User Profile",
  "description": "The authenticated user's profile information including name, email, and preferences",
  "mimeType": "application/json"
}
```

Make sure your description is at least 20 characters long and your name is at least 5 characters.

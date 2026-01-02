# MCP Resource Definition

## Goal

Define a resource according to the Model Context Protocol (MCP) specification. Resources
expose data to AI agents, complementing tools which expose actions.

## Why This Matters

While MCP tools let agents *do* things, MCP resources let agents *read* things. Resources
represent data sources that can be:

1. **Listed** - Clients can discover available resources
2. **Read** - Clients can fetch resource contents
3. **Subscribed** - Clients can receive updates (advanced)

Common resource examples include:
- File contents
- Database records
- API responses
- Configuration data
- Documentation

Understanding resources is essential for building complete MCP servers.

## The Task

Define an MCP resource that represents a user profile. Your resource definition
must follow the MCP specification exactly.

**Resource URI**: `user://profile/current`

**Purpose**: Expose the current user's profile information.

## MCP Resource Schema

An MCP resource definition has this structure:

```json
{
  "uri": "resource://path/to/resource",
  "name": "Human-readable name",
  "description": "What this resource contains",
  "mimeType": "application/json"
}
```

Key points about MCP resources:
- URIs use custom schemes (not just http/https)
- `mimeType` tells clients how to parse the content
- Resources are read-only by default
- The actual content is fetched separately via `resources/read`

## Expected Output Format

Your output must be a valid MCP resource definition as a JSON object:

```json
{
  "uri": "user://profile/current",
  "name": "Current User Profile",
  "description": "The authenticated user's profile information including name, email, and preferences",
  "mimeType": "application/json"
}
```

## Required Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uri` | string | Yes | Resource URI (must be "user://profile/current") |
| `name` | string | Yes | Human-readable name (minimum 5 characters) |
| `description` | string | Yes | Description of contents (minimum 20 characters) |
| `mimeType` | string | Yes | Content type (must be "application/json") |

## Grading

Your output is validated against:

1. **Structure** - Must have `uri`, `name`, `description`, and `mimeType`
2. **URI** - Must be exactly "user://profile/current"
3. **MIME Type** - Must be "application/json"
4. **Name** - Must be at least 5 characters
5. **Description** - Must be at least 20 characters

## Key Lesson

**Resources are data contracts.** They tell clients:
- What data is available (via `uri` and `name`)
- What to expect (via `mimeType`)
- Why it matters (via `description`)

The separation of resource *definition* from resource *content* is intentional:
1. List resources first (cheap, cacheable)
2. Fetch content on demand (may be expensive)
3. Subscribe to changes (optional, for real-time needs)

This pattern enables efficient discovery without loading all data upfront.

## Resources vs Tools

| Aspect | Resources | Tools |
|--------|-----------|-------|
| Purpose | Read data | Perform actions |
| Idempotent | Yes | Not necessarily |
| Side effects | None | May have |
| Caching | Yes | Typically no |
| Example | "Get user profile" | "Update user profile" |

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Check the MCP specification at https://spec.modelcontextprotocol.io/
- Remember: URIs can use custom schemes like `user://`
- All four fields are required

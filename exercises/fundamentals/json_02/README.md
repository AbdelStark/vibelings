# Nested JSON Structures

## Goal

Generate a valid JSON object representing a `Team` with nested members and project information.

## Why This Matters

Real-world APIs rarely deal with flat objects. Agentic systems must handle nested structures correctly — arrays of objects, optional nested fields, and complex validation rules.

Consider an LLM generating a multi-step workflow plan, an org chart, or a nested configuration file. Each layer introduces more opportunities for structural errors. Nested schema validation catches these at every level, not just the surface.

## The Concept: Compositional Schemas

JSON Schema supports **composition** — schemas within schemas:

```
Team (object)
├── name (string)
├── department (string)
├── members (array)
│   └── [items] Member (object)
│       ├── name (string)
│       ├── role (enum: lead|senior|junior)
│       └── skills (array of strings)
└── currentProject (object, optional)
    ├── name (string)
    ├── deadline (date string)
    └── status (enum)
```

This hierarchy mirrors how we think about data: teams contain members, members have skills, teams work on projects. The schema encodes this structure formally.

**The power**: validation is recursive. If `members[2].skills[0]` is the wrong type, the schema catches it. You don't need custom validation code for each nesting level.

## Requirements

Generate a JSON object representing a team with:

- `name` (string, required): Team name
- `department` (string, required): Department name
- `members` (array, required): Array of member objects, each with:
  - `name` (string, required): Member's name
  - `role` (string, required): One of "lead", "senior", "junior"
  - `skills` (array of strings, required): List of skills
- `currentProject` (object, optional): Current project with:
  - `name` (string, required): Project name
  - `deadline` (string, required): ISO 8601 date format (YYYY-MM-DD)
  - `status` (string, required): One of "planning", "active", "review", "complete"

## Example Valid Output

```json
{
  "name": "Platform Team",
  "department": "Engineering",
  "members": [
    {
      "name": "Sarah Chen",
      "role": "lead",
      "skills": ["architecture", "rust", "kubernetes"]
    },
    {
      "name": "Marcus Johnson",
      "role": "senior",
      "skills": ["backend", "python", "aws"]
    }
  ],
  "currentProject": {
    "name": "API Gateway v2",
    "deadline": "2025-06-30",
    "status": "active"
  }
}
```

## Common Mistakes

**1. Missing required fields in nested objects**
```json
{
  "members": [{"name": "Alice"}]  // Wrong: missing role and skills
}
```
Each nesting level has its own required fields. A member isn't valid without `role` and `skills`.

**2. Wrong enum values**
```json
{"role": "Lead"}     // Wrong: capitalization matters
{"role": "lead"}     // Correct
{"status": "done"}   // Wrong: not in enum
{"status": "complete"}  // Correct
```

**3. Date format errors**
```json
{"deadline": "June 30, 2025"}   // Wrong: not ISO 8601
{"deadline": "2025-6-30"}       // Wrong: needs zero-padding
{"deadline": "2025-06-30"}      // Correct
```

**4. Skills as single string instead of array**
```json
{"skills": "python, rust"}      // Wrong: string
{"skills": ["python", "rust"]}  // Correct: array
```

## Grading

Validated against the schema. Checks:
1. All required fields present at each nesting level
2. Enum values match allowed options exactly (case-sensitive)
3. Arrays contain objects with correct structure
4. Date format follows ISO 8601 (YYYY-MM-DD)
5. Nested arrays (like `skills`) contain correct types

## Key Lesson

**Schema depth matters**: Nested validation catches errors at every level. A single typo in a deeply nested field can break the entire contract.

Think of it like type checking in a programming language. In a dynamically typed language, `team.members[0].skills.includes("rust")` might fail at runtime if `skills` is a string instead of an array. With schema validation, this is caught before execution.

The mental model: every `{` opens a new validation context. Every `[` opens an array where each item must be valid. Errors propagate up—if any nested element fails, the whole structure fails.

## Connections

- **Prerequisite**: [json_01](../json_01/) introduces flat schemas
- **Next**: [json_03](../json_03/) focuses on array constraints
- **Related**: [mcp/server_01](../../mcp/server_01/) uses nested schemas for tool definitions
- **Production**: Nested structures appear everywhere in workflow and config generation

## Further Reading

- [JSON Schema: Objects](https://json-schema.org/understanding-json-schema/reference/object) — Nested object validation
- [JSON Schema: Combining schemas](https://json-schema.org/understanding-json-schema/reference/combining) — allOf, anyOf, oneOf
- [ISO 8601 Date Format](https://en.wikipedia.org/wiki/ISO_8601) — The date format standard

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Check that EVERY member has name, role, AND skills
- Remember: skills is an array, not a comma-separated string
- The currentProject is optional—but if present, all its fields are required

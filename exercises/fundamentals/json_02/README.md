# Nested JSON Structures

## Goal

Generate a valid JSON object representing a `Team` with nested members and project information.

## Why This Matters

Real-world APIs rarely deal with flat objects. Agentic systems must handle nested structures correctly — arrays of objects, optional nested fields, and complex validation rules.

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
  - `deadline` (string, required): ISO 8601 date format
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

## Grading

Validated against the schema. Checks:
1. All required fields present at each nesting level
2. Enum values match allowed options
3. Arrays contain objects with correct structure
4. Date format is valid

## Key Lesson

**Schema depth matters**: Nested validation catches errors at every level. A single typo in a deeply nested field can break the entire contract.

# JSON Output Contracts

## Goal

Generate a valid JSON object that matches the required schema for a `Person` record.

## Why This Matters

Structured output is fundamental to agentic systems. When an LLM generates data that other systems consume, the output **must** conform to a contract (schema). This isn't about "prompt engineering tricks" — it's about engineering reliable interfaces.

## Requirements

Generate a JSON object representing a person with the following fields:

- `name` (string, required): Full name of the person
- `age` (integer, required): Age in years (must be >= 0 and <= 150)
- `email` (string, required): Valid email format
- `occupation` (string, optional): Job title or profession

## Example Valid Output

```json
{
  "name": "Alice Johnson",
  "age": 32,
  "email": "alice.johnson@example.com",
  "occupation": "Software Engineer"
}
```

## Grading

Your output will be validated against the JSON Schema in `grader/schema.json`. The exercise passes if:

1. Output is valid JSON
2. All required fields are present
3. All field types are correct
4. Age is within valid range
5. Email matches the expected pattern

## Key Lesson

**Contracts over vibes**: Instead of hoping the LLM "says the right thing," we define a precise schema and validate against it. This is deterministic, reproducible, and debuggable.

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Check the schema file to understand exact requirements
- Focus on getting the structure right before worrying about content

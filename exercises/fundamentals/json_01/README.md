# JSON Output Contracts

## Goal

Generate a valid JSON object that matches the required schema for a `Person` record.

## Why This Matters

Structured output is fundamental to agentic systems. When an LLM generates data that other systems consume, the output **must** conform to a contract (schema). This isn't about "prompt engineering tricks" — it's about engineering reliable interfaces.

Consider what happens without contracts: the LLM outputs `"age": "thirty-two"` instead of `"age": 32`. Your downstream code crashes. Or it outputs extra fields that break your parser. Or it omits the email entirely. With a schema, these failures are caught immediately and automatically—not discovered in production at 3 AM.

## The Concept: Schema as Contract

A **JSON Schema** is a formal specification of what valid data looks like. It declares:
- Required fields (must be present)
- Types (string, integer, array, etc.)
- Constraints (min/max values, patterns, enums)
- Structure (nested objects, arrays of specific types)

The schema acts as a **contract between producer and consumer**. The LLM (producer) commits to outputting data that matches the schema. Your code (consumer) can rely on that structure being present.

This is the same principle behind:
- **API specifications** (OpenAPI/Swagger)
- **Database schemas** (SQL DDL, Prisma schemas)
- **Type systems** (TypeScript interfaces, Rust structs)
- **Protocol buffers** (gRPC message definitions)

The agentic systems version is: define the contract in JSON Schema, validate LLM output against it.

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

## Common Mistakes

**1. Age as string instead of integer**
```json
{"age": "32"}      // Wrong: string
{"age": 32}        // Correct: integer
```
LLMs often quote numbers. JSON Schema distinguishes between `"32"` (a string) and `32` (an integer).

**2. Missing required fields**
```json
{"name": "Alice", "age": 32}  // Wrong: missing email
```
Optional fields can be omitted; required fields cannot.

**3. Extra fields when schema forbids them**
```json
{"name": "Alice", "age": 32, "email": "a@b.com", "phone": "555-1234"}
```
If the schema specifies `"additionalProperties": false`, unexpected fields cause validation failure.

**4. Invalid email format**
```json
{"email": "alice at example dot com"}  // Wrong: not email format
{"email": "alice@example.com"}          // Correct
```
Email patterns have specific syntax requirements.

## Grading

Your output will be validated against the JSON Schema in `grader/schema.json`. The exercise passes if:

1. Output is valid JSON (parseable)
2. All required fields are present (`name`, `age`, `email`)
3. All field types are correct (string, integer, string)
4. Age is within valid range (0–150)
5. Email matches the expected pattern

## Key Lesson

**Contracts over vibes**: Instead of hoping the LLM "says the right thing," define a precise schema and validate against it. This is deterministic, reproducible, and debuggable.

The mental shift: don't ask "did the LLM produce reasonable output?" Instead ask: "does this output satisfy the contract?" The first question is subjective and unverifiable. The second is objective and automatable.

This pattern scales. Once you think in contracts, you can compose them: tool inputs have schemas, tool outputs have schemas, workflows have schemas. Each interface becomes a checkpoint where you can verify correctness.

## Connections

- **Next**: [json_02](../json_02/) adds nested objects to schemas
- **Related**: [tools_01](../tools_01/) applies contracts to tool definitions
- **Advanced**: [context_01](../../context/context_01/) uses structured output for system prompts

## Further Reading

- [JSON Schema specification](https://json-schema.org/understanding-json-schema/) — The language for defining contracts
- [Anthropic: Structured output](https://docs.anthropic.com/en/docs/build-with-claude/structured-output) — Using JSON mode with Claude
- [OpenAI: Structured outputs](https://platform.openai.com/docs/guides/structured-outputs) — JSON Schema enforcement in API calls

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Check the schema file to understand exact requirements
- Focus on getting the structure right before worrying about content

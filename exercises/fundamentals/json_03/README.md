# Array Contracts

## Goal

Generate a valid JSON object representing an event schedule with multiple sessions,
demonstrating proper handling of arrays and array item validation.

## Why This Matters

Production agentic systems frequently work with collections: lists of search results,
arrays of tool calls, batches of records to process. Each item in an array must
conform to its own schema, and the array itself may have constraints (min/max items,
uniqueness).

Consider an LLM generating a batch of database operations, or a list of files to modify, or a sequence of API calls. If one item is malformed, you might:
- Fail the entire batch (frustrating but safe)
- Process partial results (risky—which items succeeded?)
- Silently skip bad items (dangerous—data loss)

Schema validation surfaces these problems immediately, before execution.

## The Concept: Array Schema Constraints

JSON Schema provides two levels of array validation:

**Item-level**: What each element must look like
```json
{
  "items": {
    "type": "object",
    "properties": { ... },
    "required": [ ... ]
  }
}
```

**Array-level**: Constraints on the collection itself
```json
{
  "type": "array",
  "minItems": 2,
  "maxItems": 10,
  "uniqueItems": true
}
```

This dual validation catches both:
- Individual items that are malformed
- Collections that are too small, too large, or have duplicates

**The analogy**: Think of a database table. Each row must match the table schema (item validation). The table might have constraints like "at least one row" or "primary key must be unique" (array validation).

## Requirements

Generate a JSON object representing a conference schedule with the following structure:

### Top-level fields

- `event_name` (string, required): Name of the event
- `date` (string, required): Event date in ISO format (YYYY-MM-DD)
- `sessions` (array, required): List of sessions (minimum 2, maximum 10)

### Session object fields

Each session must have:

- `id` (string, required): Unique session identifier (e.g., "S001")
- `title` (string, required): Session title
- `speaker` (string, required): Speaker name
- `time_slot` (string, required): Time in 24h format (HH:MM)
- `duration_minutes` (integer, required): Duration (15-120 minutes)
- `track` (string, required): One of "technical", "workshop", "keynote"

## Example Valid Output

```json
{
  "event_name": "AI Engineering Summit 2025",
  "date": "2025-06-15",
  "sessions": [
    {
      "id": "S001",
      "title": "Building Reliable Agentic Systems",
      "speaker": "Dr. Sarah Chen",
      "time_slot": "09:00",
      "duration_minutes": 60,
      "track": "keynote"
    },
    {
      "id": "S002",
      "title": "MCP Server Implementation Workshop",
      "speaker": "Alex Rivera",
      "time_slot": "10:30",
      "duration_minutes": 90,
      "track": "workshop"
    },
    {
      "id": "S003",
      "title": "Deterministic Grading for LLM Outputs",
      "speaker": "Jordan Kim",
      "time_slot": "14:00",
      "duration_minutes": 45,
      "track": "technical"
    }
  ]
}
```

## Common Mistakes

**1. Too few or too many items**
```json
{"sessions": [{"...": "..."}]}           // Wrong: only 1 session (min 2)
{"sessions": [...11 sessions...]}        // Wrong: 11 sessions (max 10)
```

**2. Duration outside valid range**
```json
{"duration_minutes": 10}     // Wrong: below minimum (15)
{"duration_minutes": 180}    // Wrong: above maximum (120)
{"duration_minutes": "60"}   // Wrong: string instead of integer
```

**3. Invalid time format**
```json
{"time_slot": "9:00"}       // Wrong: needs leading zero
{"time_slot": "09:00 AM"}   // Wrong: 24h format, no AM/PM
{"time_slot": "09:00"}      // Correct
```

**4. Empty arrays or missing fields**
```json
{"sessions": []}                         // Wrong: empty (min 2)
{"sessions": [{"id": "S001"}]}          // Wrong: session missing fields
```

## Grading

Your output will be validated against the JSON Schema in `grader/schema.json`. The exercise passes if:

1. Output is valid JSON
2. All required fields are present
3. Array has at least 2 sessions (and no more than 10)
4. Each session has all required fields with correct types
5. Duration is within valid range (15-120)
6. Track is one of the allowed enum values
7. Date and time formats are correct

## Key Lesson

**Arrays require item-level validation.** It's not enough to check that you have a list —
each element must be valid. In production systems, a single malformed item in an array
can break downstream processing.

Think of arrays as contracts with two parts:
1. **The container contract**: "I promise to have 2-10 items"
2. **The item contract**: "I promise each item has these fields with these types"

Both contracts must be satisfied. An empty array violates the container contract. A malformed item violates the item contract. Either failure means the whole validation fails.

This strictness is a feature, not a bug. It surfaces problems early, before they cascade through your system.

## Connections

- **Prerequisite**: [json_02](../json_02/) introduces nested objects
- **Related**: [tools_02](../tools_02/) uses arrays of tool calls
- **Production**: [production_eval_01](../../production/production_eval_01/) validates arrays of test cases

## Further Reading

- [JSON Schema: Arrays](https://json-schema.org/understanding-json-schema/reference/array) — Full array validation reference
- [Batch processing patterns](https://docs.anthropic.com/en/docs/build-with-claude/batch-processing) — Handling multiple items efficiently
- [Time formats](https://en.wikipedia.org/wiki/ISO_8601#Times) — ISO 8601 time representation

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Check that each session has ALL required fields
- Verify the track field uses exact enum values ("technical", "workshop", or "keynote")
- Ensure duration is between 15 and 120 minutes
- Make sure you have at least 2 sessions

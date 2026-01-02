# Array Contracts

## Goal

Generate a valid JSON object representing an event schedule with multiple sessions,
demonstrating proper handling of arrays and array item validation.

## Why This Matters

Production agentic systems frequently work with collections: lists of search results,
arrays of tool calls, batches of records to process. Each item in an array must
conform to its own schema, and the array itself may have constraints (min/max items,
uniqueness).

This exercise teaches:

1. **Array item validation** - Each item must match the schema
2. **Array-level constraints** - Minimum/maximum number of items
3. **Object arrays** - Complex objects within arrays
4. **Practical data modeling** - Real-world schedule representation

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

When working with collections:
- Define clear schemas for array items
- Set sensible bounds on array size
- Use enums for constrained string fields
- Validate each item independently

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Check that each session has ALL required fields
- Verify the track field uses exact enum values ("technical", "workshop", or "keynote")
- Ensure duration is between 15 and 120 minutes
- Make sure you have at least 2 sessions

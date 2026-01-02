# System Prompt Structure

## Goal

Design a well-structured system prompt that uses clear sections and XML tags to organize instructions for an AI assistant.

## Why This Matters

Context engineering starts with how you structure your system prompt. A well-organized prompt:
- Reduces ambiguity and improves instruction following
- Makes it easier to update and maintain prompts
- Helps the model distinguish between different types of instructions
- Provides clear boundaries between configuration, behavior, and constraints

This is the foundation of the "Goldilocks zone" — specific enough to guide behavior, flexible enough to handle edge cases.

## Requirements

Create a system prompt configuration (as JSON) for a customer support assistant with the following sections:

1. **Role** (`role`): Define who the assistant is and its primary purpose
2. **Capabilities** (`capabilities`): List what the assistant can do (array of strings)
3. **Constraints** (`constraints`): Define boundaries and limitations (array of strings)
4. **Response Format** (`response_format`): Specify how responses should be structured
5. **Examples** (`examples`): Provide at least one example interaction

The output must be valid JSON matching the required schema.

## Example Structure

```json
{
  "role": "You are a helpful...",
  "capabilities": ["Can do X", "Can do Y"],
  "constraints": ["Must not do A", "Must not do B"],
  "response_format": {
    "style": "concise",
    "max_length": 500,
    "include_sources": false
  },
  "examples": [
    {
      "user": "Example question",
      "assistant": "Example response"
    }
  ]
}
```

## Grading

Your output will be validated against a JSON Schema. The exercise passes if:

1. All required sections are present
2. Arrays contain at least the minimum required items
3. The response format includes required formatting fields
4. At least one example interaction is provided

## Key Lesson

**Structure creates clarity**: Just as code benefits from clear organization, prompts benefit from explicit sections. XML tags like `<role>`, `<constraints>`, and `<examples>` (or JSON structure as we use here) help models parse complex instructions reliably.

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about what a customer support agent needs to know
- Consider what constraints prevent harmful or off-topic responses

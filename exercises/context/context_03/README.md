# Just-in-Time Context Retrieval

## Goal

Design a context retrieval strategy that loads information progressively based on task requirements, rather than pre-loading everything upfront.

## Why This Matters

Pre-loading all potentially relevant context wastes tokens and degrades performance. Just-in-time (JIT) context retrieval:

- Keeps initial context lean and focused
- Loads detailed information only when the task requires it
- Mirrors how humans work — we don't memorize entire codebases, we look things up when needed
- Enables handling tasks that would otherwise exceed context limits

This pattern is essential for agents that work on complex, multi-step tasks.

## Requirements

You're designing a JIT context system for a technical support agent. Create a retrieval strategy that specifies:

1. **Initial context** (`initial_context`): What's always loaded at conversation start
2. **Retrieval triggers** (`triggers`): Conditions that trigger context loading
3. **Context sources** (`sources`): Available sources with their metadata
4. **Loading priority** (`loading_order`): Order in which sources are queried

For each trigger, specify:
- The condition pattern that activates it
- Which source(s) to query
- Maximum tokens to retrieve
- Whether to cache the result

## Example Scenario

User asks: "How do I reset my password?"
- Initial context has basic product info
- Trigger: "password" or "reset" keywords detected
- Action: Load authentication documentation (max 500 tokens)
- Cache: Yes (likely to be referenced again)

## Grading

Your output will be validated against a JSON Schema. The exercise passes if:

1. Initial context specifies at least 2 items always loaded
2. At least 3 triggers are defined with valid patterns
3. At least 3 sources are defined with metadata
4. Loading order is specified as an array of source names

## Key Lesson

**Progressive disclosure beats pre-loading**: Don't stuff everything into the initial prompt. Design your agent to recognize when it needs more context and fetch it dynamically. This keeps base performance high and enables handling of arbitrarily complex queries.

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about what a support agent ALWAYS needs vs. what's situational
- Consider how user queries signal what context is needed

# Context Budget Management

## Goal

Design a context budget allocation plan that prioritizes different types of context within a fixed token limit.

## Why This Matters

LLM context windows are finite resources. As conversations grow, models experience "context rot" — degraded performance as attention is spread thin across more tokens. Effective context engineering requires:

- Understanding the relative importance of different context types
- Allocating tokens strategically based on task requirements
- Making explicit trade-offs when context is constrained

The goal: **Find the smallest set of high-signal tokens that maximize the likelihood of your desired outcome.**

## Requirements

You're building an agent that assists with code review. Design a context budget allocation that distributes 8000 tokens across these context types:

1. **System prompt** (`system_prompt`): Core instructions and behavior
2. **Code under review** (`code_context`): The actual code being reviewed
3. **Documentation** (`documentation`): Relevant docs, style guides, best practices
4. **Conversation history** (`conversation_history`): Previous messages in the review
5. **Tool definitions** (`tool_definitions`): Available tools the agent can use

Your allocation must include:
- Token budget for each category (must sum to exactly 8000)
- Priority level (1-5, where 1 is highest priority)
- Compression strategy for when context exceeds budget
- Justification for the allocation

## Grading

Your output will be validated against a JSON Schema. The exercise passes if:

1. All five context types are allocated
2. Token budgets sum to exactly 8000
3. Priority levels are valid (1-5)
4. Each category has a compression strategy
5. Justifications are provided

## Key Lesson

**Context is a zero-sum game**: Every token spent on low-value content is a token unavailable for high-value content. Explicit budgets force you to make trade-offs consciously rather than letting context bloat happen accidentally.

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about which context is most critical for accurate code reviews
- Consider what can be summarized vs. what must be verbatim

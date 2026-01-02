# Context Budget Management

## Goal

Design a context budget allocation plan that prioritizes different types of context within a fixed token limit.

## Why This Matters

LLM context windows are finite resources. As conversations grow, models experience "context rot" — degraded performance as attention is spread thin across more tokens. Effective context engineering requires:

- Understanding the relative importance of different context types
- Allocating tokens strategically based on task requirements
- Making explicit trade-offs when context is constrained

The goal: **Find the smallest set of high-signal tokens that maximize the likelihood of your desired outcome.**

## The Concept: Context as a Budget

Think of context like RAM in a computer. You have a fixed amount. You must decide:
- What to load (and what to leave out)
- What to keep resident (and what to page out)
- How to compress data that's too large

```
Total Budget: 8000 tokens
┌────────────────────────────────────────────┐
│ System Prompt: 500 tokens (essential)      │
│ ████████                                   │
├────────────────────────────────────────────┤
│ Code Context: 4000 tokens (primary input)  │
│ ████████████████████████████████████████   │
├────────────────────────────────────────────┤
│ Documentation: 1500 tokens (reference)     │
│ ███████████████                            │
├────────────────────────────────────────────┤
│ Conversation: 1500 tokens (history)        │
│ ███████████████                            │
├────────────────────────────────────────────┤
│ Tool Definitions: 500 tokens (fixed)       │
│ ████████                                   │
└────────────────────────────────────────────┘
```

### Priority-Based Allocation

When context exceeds budget, you must compress. Priority determines what gets cut first:

| Priority | Compression Strategy |
|----------|---------------------|
| 1 (Highest) | Never compress — this is essential |
| 2 | Compress only if absolutely necessary |
| 3 | Compress when approaching limits |
| 4 | Aggressively compress or summarize |
| 5 (Lowest) | Drop entirely if needed |

### Compression Strategies

Different context types compress differently:

- **System prompt**: Rarely compress — core behavior
- **Code context**: Show relevant sections, not entire files
- **Documentation**: Summarize, retrieve on-demand
- **Conversation**: Compact older turns, keep recent
- **Tool definitions**: Fixed size, can't compress

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

## Common Mistakes

**1. Equal allocation**
```json
{"system_prompt": 1600, "code_context": 1600, ...}  // Wrong: not all context is equal
```
A code review agent needs code more than conversation history. Allocate based on task importance.

**2. No compression strategy**
When context overflows, what gets cut? Without a strategy, you get random truncation.

**3. Budgets that don't sum correctly**
```json
{"total": 7500}  // Wrong: doesn't match the 8000 limit
```

**4. Ignoring fixed costs**
Tool definitions are often fixed size. Budget for them first, then allocate the remainder.

## Grading

Your output will be validated against a JSON Schema. The exercise passes if:

1. All five context types are allocated
2. Token budgets sum to exactly 8000
3. Priority levels are valid (1-5)
4. Each category has a compression strategy
5. Justifications are provided

## Key Lesson

**Context is a zero-sum game**: Every token spent on low-value content is a token unavailable for high-value content. Explicit budgets force you to make trade-offs consciously rather than letting context bloat happen accidentally.

The mindset shift: stop thinking "fit everything in" and start thinking "what's the minimal context that achieves the goal?" Lean context often performs better than bloated context.

## Connections

- **Prerequisite**: [context_01](../context_01/) covers system prompt structure
- **Next**: [context_03](../context_03/) introduces just-in-time retrieval
- **Related**: [fundamentals/observability_01](../../fundamentals/observability_01/) covers cost awareness

## Further Reading

- [Anthropic: Context engineering](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents) — Managing the finite resource
- [Context length vs attention](https://arxiv.org/abs/2307.03172) — Research on "lost in the middle" phenomenon
- [Token counting tools](https://github.com/openai/tiktoken) — Practical token estimation

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about which context is most critical for accurate code reviews
- Consider what can be summarized vs. what must be verbatim
- The code being reviewed is probably the highest priority

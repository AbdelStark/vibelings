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

## The Concept: Progressive Context Loading

Instead of loading everything upfront, JIT retrieval works like a search engine:

```
Initial State (lean):
┌──────────────────────────────────────┐
│ System prompt: 500 tokens            │
│ Basic product info: 200 tokens       │
│ Available: 7300 tokens               │
└──────────────────────────────────────┘

User asks: "How do I reset my password?"
           │
           ▼
Trigger detected: "password" + "reset"
           │
           ▼
Load: Authentication documentation
           │
           ▼
┌──────────────────────────────────────┐
│ System prompt: 500 tokens            │
│ Basic product info: 200 tokens       │
│ Auth docs: 400 tokens (just loaded)  │
│ Available: 6900 tokens               │
└──────────────────────────────────────┘
```

### Key Components

**1. Initial Context**: The minimal "always-on" information
- Role and behavior instructions
- Product/service overview
- How to request more context

**2. Triggers**: Patterns that signal "load more context"
- Keywords: "password", "billing", "refund"
- Intents: question about X, request for Y
- Context: after tool call Z, when error W occurs

**3. Sources**: Where to get the additional context
- Documentation embeddings
- Knowledge base articles
- Previous conversation summaries
- Database lookups

**4. Caching**: Whether to keep loaded context
- Frequently referenced → cache
- One-time lookup → discard after use

### The Trade-off

| Approach | Pros | Cons |
|----------|------|------|
| Pre-load everything | Simple, no retrieval latency | Context bloat, worse performance |
| JIT retrieval | Lean context, scales to any size | Retrieval latency, trigger complexity |

JIT is almost always better for production agents working on complex tasks.

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

## Common Mistakes

**1. Too much initial context**
```json
{"initial_context": ["all_product_docs", "all_faq", "all_history"]}  // Wrong: defeats the purpose
```
Initial context should be minimal — just enough to handle common cases and know when to fetch more.

**2. Triggers that are too broad**
```json
{"pattern": "help"}  // Wrong: too common, will over-trigger
{"pattern": "help AND password AND (reset OR forgot)"}  // Better: specific
```

**3. No caching strategy**
Repeated queries for the same context waste time and tokens. Cache things that will be referenced again.

**4. No priority order**
When multiple sources match, which do you query first? Define an order.

## Grading

Your output will be validated against a JSON Schema. The exercise passes if:

1. Initial context specifies at least 2 items always loaded
2. At least 3 triggers are defined with valid patterns
3. At least 3 sources are defined with metadata
4. Loading order is specified as an array of source names

## Key Lesson

**Progressive disclosure beats pre-loading**: Don't stuff everything into the initial prompt. Design your agent to recognize when it needs more context and fetch it dynamically. This keeps base performance high and enables handling of arbitrarily complex queries.

The mental model: your agent is a research assistant with a filing cabinet. They don't read every file before answering a question — they look up what's relevant when the question arrives.

## Connections

- **Prerequisite**: [context_02](../context_02/) covers context budgeting
- **Next**: [context_04](../context_04/) introduces context compaction
- **Related**: [fundamentals/tools_01](../../fundamentals/tools_01/) — retrieval can use tool calls

## Further Reading

- [RAG (Retrieval Augmented Generation)](https://www.pinecone.io/learn/retrieval-augmented-generation/) — The underlying technique
- [Anthropic: Retrieval patterns](https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering/rag-and-retrieval) — Official guidance
- [Vector databases](https://www.pinecone.io/learn/vector-database/) — Infrastructure for similarity search

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about what a support agent ALWAYS needs vs. what's situational
- Consider how user queries signal what context is needed
- "Password" and "billing" queries need very different context

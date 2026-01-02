# Context Compaction

## Goal

Given a long conversation history, produce a compacted summary that preserves critical information while dramatically reducing token count.

## Why This Matters

Long-running agent sessions accumulate context that eventually exceeds limits. Naive truncation loses important decisions and context. Effective compaction:

- Preserves key decisions, facts, and commitments
- Discards redundant tool outputs and verbose explanations
- Maintains enough context for the agent to continue coherently
- Reduces tokens by 50-80% while retaining 90%+ of useful information

This is essential for agents that handle extended tasks or multi-session workflows.

## The Concept: Lossy vs. Lossless Compression

Context compaction is **lossy compression** — you intentionally discard information to fit within limits. The skill is choosing *what* to discard.

```
High-value (keep):              Low-value (discard):
• User made decision X          • Agent's verbose explanation
• Fact Y was established        • Tool output formatting
• Task is now in state Z        • Acknowledgments ("Got it!")
• Open questions remain         • Repeated information
```

### What to Preserve

| Type | Example | Why it matters |
|------|---------|----------------|
| **Decisions** | "User chose Kubernetes" | Affects all future actions |
| **Facts** | "Budget is $500/month" | Constrains options |
| **State** | "Currently configuring auth" | Required to continue |
| **Open items** | "TLS cert source TBD" | Must be resolved |
| **Commitments** | "Will use Helm charts" | Agent is bound by this |

### What to Discard

| Type | Example | Why it's safe to discard |
|------|---------|-------------------------|
| **Exploration** | "Here are 5 options..." | Decision supersedes this |
| **Tool outputs** | Raw JSON responses | Extract facts, discard format |
| **Acknowledgments** | "Thanks for clarifying!" | No information content |
| **Redundancy** | Same fact stated 3 times | Keep once |

### Compression Ratio

A good compaction achieves:
- 50-80% token reduction
- 90%+ information retention (for task continuation)
- Traceability (turn numbers for audit)

## Requirements

You're given a conversation transcript (in the prompt). Produce a compacted summary that includes:

1. **Key decisions** (`decisions`): Important choices made during the conversation
2. **Facts established** (`facts`): Information that was confirmed or discovered
3. **Current state** (`current_state`): Where the task currently stands
4. **Open items** (`open_items`): Unresolved questions or pending actions
5. **Compression metadata** (`metadata`): Statistics about the compaction

For each decision and fact, include:
- A concise statement of what was decided/established
- Why it matters for continuing the task
- The turn number where it occurred (for traceability)

## Example

Before compaction (2000 tokens):
```
Turn 1: User asks about deployment options
Turn 2: Agent lists 5 options with pros/cons
Turn 3: User asks about Kubernetes specifically
Turn 4: Agent provides detailed K8s explanation
Turn 5: User decides on K8s with Helm
Turn 6: Agent confirms and asks about cluster size
...
```

After compaction (400 tokens):
```json
{
  "decisions": [
    {"statement": "Deploy using Kubernetes with Helm charts", "turn": 5}
  ],
  "facts": [
    {"statement": "Target environment is AWS EKS", "turn": 7}
  ],
  "current_state": "Configuring Helm values for production cluster",
  "open_items": ["Cluster size not yet determined", "TLS certificate source pending"]
}
```

## Common Mistakes

**1. Preserving exploration instead of decisions**
```json
{"decisions": ["Discussed 5 deployment options"]}  // Wrong: this is exploration
{"decisions": ["Chose Kubernetes with Helm"]}      // Correct: the actual decision
```

**2. Missing turn numbers**
Turn numbers enable audit trails. If something seems wrong later, you can reference the original context.

**3. Losing open items**
The next agent turn needs to know what's unresolved. Missing open items causes confusion.

**4. No metadata**
How do you know the compaction is good? Metadata shows the compression ratio.

## Grading

Your output will be validated against a JSON Schema. The exercise passes if:

1. At least 2 decisions are identified with turn numbers
2. At least 2 facts are documented
3. Current state is clearly described
4. Open items lists remaining questions
5. Metadata includes original and compacted token estimates

## Key Lesson

**Compress without losing signal**: The goal isn't to make context smaller — it's to make context *denser*. Every token in a compacted summary should carry maximum information value. Tool outputs, verbose explanations, and repetitive acknowledgments are compression targets.

Think of it as extracting the "state" of the conversation: what was decided, what's known, where we are, what's left to do. Everything else is derivable or irrelevant.

## Connections

- **Prerequisite**: [context_03](../context_03/) covers JIT retrieval
- **Next**: [context_05](../context_05/) covers tool design for efficiency
- **Related**: [fundamentals/observability_01](../../fundamentals/observability_01/) — compaction should preserve trace info

## Further Reading

- [Conversation summarization](https://arxiv.org/abs/2109.07943) — Research on extractive summarization
- [Anthropic: Long conversations](https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering/long-context-window-tips) — Managing extended sessions
- [State machines](https://en.wikipedia.org/wiki/Finite-state_machine) — Mental model for "current state"

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Focus on what would be lost if you couldn't see the original conversation
- Decisions that affect future actions are highest priority to preserve
- Ask: "What does the next agent turn need to know?"

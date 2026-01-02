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

## Grading

Your output will be validated against a JSON Schema. The exercise passes if:

1. At least 2 decisions are identified with turn numbers
2. At least 2 facts are documented
3. Current state is clearly described
4. Open items lists remaining questions
5. Metadata includes original and compacted token estimates

## Key Lesson

**Compress without losing signal**: The goal isn't to make context smaller — it's to make context *denser*. Every token in a compacted summary should carry maximum information value. Tool outputs, verbose explanations, and repetitive acknowledgments are compression targets.

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Focus on what would be lost if you couldn't see the original conversation
- Decisions that affect future actions are highest priority to preserve

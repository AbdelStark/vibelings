# Human-in-the-Loop Patterns

## Goal

Design a workflow that incorporates human approval gates, confidence thresholds,
and fallback handlers for cases where automation cannot proceed safely.

## Why This Matters

Not all decisions can be automated. Reliable agentic systems must:
- Recognize when confidence is too low
- Request human intervention appropriately
- Provide sufficient context for human decisions
- Handle timeout and rejection gracefully

Human-in-the-loop (HITL) is not a failure mode - it's a design pattern for safety.

## The Concept: Confidence-Based Routing

The key insight is that model confidence should drive the automation/human split:

```
                   Classification Result
                          │
                          ▼
              ┌───────────────────────┐
              │   Confidence Check    │
              └───────────────────────┘
                    │         │
         ≥ 0.95     │         │    < 0.80
        ┌───────────┘         └───────────┐
        ▼                                 ▼
   ┌──────────┐                    ┌──────────────┐
   │ Auto-Act │                    │ Human Queue  │
   └──────────┘                    └──────────────┘
                                         │
                              ┌──────────┼──────────┐
                              ▼          ▼          ▼
                          Approved   Rejected   Timeout
                              │          │          │
                              ▼          ▼          ▼
                          Publish    Remove      Hold
```

**The principle**: Humans are expensive and slow. Use them where their judgment matters most — in the ambiguous cases where automation is unreliable.

### The Three Timeout Strategies

| Strategy | When to use | Trade-off |
|----------|-------------|-----------|
| Escalate | High-stakes decisions | Adds latency but maintains oversight |
| Default-safe | Low-risk content | Faster but might over-filter |
| Default-permissive | Time-sensitive, low-risk | Faster but might miss issues |

Choose based on the cost of false positives vs false negatives in your domain.

## The Task

Design a content moderation workflow that:

1. Receives user-generated content
2. Runs automated classification
3. Routes to human review when confidence is below threshold
4. Handles approval, rejection, and timeout outcomes

## Workflow Structure

Your output must define a workflow with approval gates:

```json
{
  "workflow": {
    "name": "Content Moderation",
    "steps": [...],
    "approval_gates": [...],
    "timeout_handling": {...}
  }
}
```

## Required Steps

### Step 1: receive_content
Ingests the user content with metadata.

### Step 2: classify_content
Runs automated classification with confidence score.

### Step 3: route_decision
Routes based on confidence threshold.

### Step 4: apply_action
Applies the final moderation decision.

## Approval Gates

Define at least one approval gate with:

```json
{
  "id": "human_review",
  "trigger_condition": "confidence < 0.8",
  "request_to": ["moderator_queue"],
  "context_fields": ["content_id", "content_text", "classification", "confidence"],
  "timeout_seconds": 3600,
  "outcomes": {
    "approved": { "next_step": "apply_action", "action": "publish" },
    "rejected": { "next_step": "apply_action", "action": "remove" },
    "timeout": { "next_step": "apply_action", "action": "hold" }
  }
}
```

## Key Design Elements

### Confidence Thresholds
Define when automation is sufficient vs when humans are needed:
- High confidence (>=0.95): Auto-approve or auto-reject
- Medium confidence (0.8-0.95): May need spot-check
- Low confidence (<0.8): Requires human review

### Context for Humans
Provide enough information for quick, accurate decisions:
- The content itself
- Classification result and confidence
- Similar past decisions (if available)
- Time constraints

### Timeout Handling
Define what happens when humans don't respond:
- Escalate to different queue
- Apply default safe action
- Notify stakeholders

## Grading

Your output is validated against:

1. **Workflow structure** - Has name, steps, approval_gates, timeout_handling
2. **Step structure** - Each step has required fields
3. **Approval gate** - At least one gate with proper outcome handling
4. **Timeout handling** - Defines escalation and default actions

## Common Mistakes

**1. Binary confidence thresholds**
```json
{"threshold": 0.5}  // Wrong: too coarse, everything is either auto or human
```
Use multiple thresholds to create zones: auto-approve, auto-reject, human-review.

**2. Insufficient context for human reviewers**
```json
{"context_fields": ["content_id"]}  // Wrong: reviewer has to look up everything
{"context_fields": ["content_id", "content_text", "classification", "confidence", "similar_cases"]}  // Better
```
Every click costs reviewer time. Put information at their fingertips.

**3. No timeout handling**
What happens when the human queue backs up? Without timeout handling, items sit forever and users wait indefinitely.

**4. Treating all human decisions equally**
Some decisions are routine, others are edge cases that should influence your model. Track and categorize human decisions for feedback loops.

**5. No way to override automation**
Even high-confidence automation should have a path to human review for appeals or audits.

## Key Lesson

**Human-in-the-loop is a reliability pattern, not a crutch.**

Well-designed HITL workflows:
- Minimize human involvement (only when needed)
- Maximize human effectiveness (good context, clear choices)
- Handle human unavailability (timeouts, defaults)
- Learn from human decisions (improve automation over time)

The goal is not to remove humans, but to use their time wisely on cases
that genuinely require human judgment.

## Connections

- **Prerequisite**: [workflow_tool_wiring_01](../workflow_tool_wiring_01/) — pipeline structure
- **Related**: [production/production_eval_01](../../production/production_eval_01/) — use human decisions to build eval sets
- **Related**: [fundamentals/guardrails_01](../../fundamentals/guardrails_01/) — confidence thresholds are a form of guardrail

## Further Reading

- [Anthropic: Human feedback](https://www.anthropic.com/research/rlhf) — How human feedback improves models
- [Apple: Human Interface Guidelines for ML](https://developer.apple.com/design/human-interface-guidelines/machine-learning) — Designing for human-AI interaction
- [Google PAIR: Human-AI Interaction](https://pair.withgoogle.com/) — Research on human-AI collaboration

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about what a human reviewer needs to make a quick decision
- Consider: what's the safest default action on timeout?

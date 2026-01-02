# Evaluation Harness Design

## Goal

Design an evaluation harness that measures agent reliability through deterministic
test cases, multi-run assessment, and regression detection.

## Why This Matters

Production agentic systems require continuous evaluation to:
- Catch regressions before they reach users
- Measure reliability across model updates
- Validate behavior against known-good examples
- Track drift over time

Evals are not one-time tests — they're an ongoing reliability practice.

## The Concept: Evals as Production Infrastructure

LLMs are non-deterministic. The same prompt can produce different outputs. This makes testing fundamentally different from traditional software:

```
Traditional Testing:          LLM Testing:
f(x) = y                      f(x) ≈ y (sometimes)
Pass/Fail (binary)            Pass rate (statistical)
One run sufficient            Multiple runs required
```

**The key insight**: You're not testing "correctness" — you're measuring *reliability*. An 80% pass rate might be acceptable for some tasks, catastrophic for others.

### The Eval Feedback Loop

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│   Code Change ───► Run Evals ───► Results           │
│        ▲                              │             │
│        │                              ▼             │
│        └─── Iterate ◄──── Regression? ──► Deploy   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

Evals run on every change. Regressions block deployment. This is CI/CD for LLM systems.

## The Task

Design an evaluation harness for a customer support agent that handles
ticket classification and response generation.

Your harness must define:
1. Test cases with deterministic expected outcomes
2. Evaluation metrics with acceptance thresholds
3. Multi-run reliability requirements
4. Regression detection configuration

## Harness Structure

Your output must follow this structure:

```json
{
  "eval_harness": {
    "name": "Harness Name",
    "agent_under_test": {...},
    "test_cases": [...],
    "metrics": [...],
    "reliability": {...},
    "regression_detection": {...}
  }
}
```

## Required Components

### 1. Agent Under Test
Describe the agent being evaluated:
```json
{
  "agent_under_test": {
    "name": "customer_support_agent",
    "capabilities": ["ticket_classification", "response_generation"],
    "model": "claude-sonnet-4-20250514"
  }
}
```

### 2. Test Cases (minimum 3)
Each test case needs:
- `id`: Unique identifier
- `input`: The test input
- `expected`: Expected behavior/output
- `type`: One of "deterministic", "semantic", "behavioral"

### 3. Metrics (minimum 2)
Each metric needs:
- `name`: Metric identifier
- `type`: "accuracy", "latency", "cost", or "reliability"
- `threshold`: Acceptance threshold
- `aggregation`: How to aggregate across runs

### 4. Reliability Configuration
```json
{
  "reliability": {
    "runs_per_case": 5,
    "pass_threshold": 0.8,
    "confidence_interval": 0.95
  }
}
```

### 5. Regression Detection
```json
{
  "regression_detection": {
    "baseline": "previous_release",
    "tolerance": 0.05,
    "alert_on_degradation": true
  }
}
```

## Test Case Types

| Type | Use Case | Evaluation Method |
|------|----------|-------------------|
| deterministic | Exact match expected | String equality |
| semantic | Meaning preserved | Embedding similarity |
| behavioral | Correct action taken | Schema validation |

## Common Mistakes

**1. Only testing happy paths**
Real users send typos, irrelevant questions, and edge cases. Test those too.

**2. Insufficient runs for statistical confidence**
```json
{"runs_per_case": 1}  // Wrong: single run proves nothing
{"runs_per_case": 5}  // Better: can measure reliability
```

**3. No baseline for regression detection**
Without a baseline, you can't detect degradation. Always compare to known-good state.

**4. Binary metrics for statistical behaviors**
```json
{"threshold": 1.0}  // Wrong: demands perfection from non-deterministic system
{"threshold": 0.8}  // Better: accepts reasonable reliability
```

## Grading

Your output is validated against:

1. **Structure** — Must have all required sections
2. **Test cases** — Minimum 3 with required fields
3. **Metrics** — Minimum 2 with thresholds
4. **Reliability** — Proper multi-run configuration
5. **Regression** — Detection configuration present

## Key Lesson

**Evals are production infrastructure, not afterthoughts.**

Good eval harnesses:
- Run automatically on every change
- Have deterministic success criteria
- Catch regressions before deployment
- Provide actionable failure information
- Track reliability over time

Without evals, you're flying blind. Model updates, prompt changes, and even API updates can cause regressions. Evals catch them.

## Connections

- **Prerequisite**: All fundamentals exercises — evals tie everything together
- **Related**: [production_budget_01](../production_budget_01/) — evals should include cost metrics
- **Production**: Evals integrate with CI/CD pipelines

## Further Reading

- [Anthropic: Evaluating AI systems](https://www.anthropic.com/research/evaluating-ai-systems) — Framework for AI evaluation
- [OpenAI Evals](https://github.com/openai/evals) — Open-source evaluation framework
- [Braintrust](https://www.braintrust.dev/) — Production eval infrastructure

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about what could go wrong with a support agent
- Consider: what would a "silent failure" look like?
- Include both classification accuracy AND response quality metrics

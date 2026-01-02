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

Evals are not one-time tests - they're an ongoing reliability practice.

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

## Grading

Your output is validated against:

1. **Structure** - Must have all required sections
2. **Test cases** - Minimum 3 with required fields
3. **Metrics** - Minimum 2 with thresholds
4. **Reliability** - Proper multi-run configuration
5. **Regression** - Detection configuration present

## Key Lesson

**Evals are production infrastructure, not afterthoughts.**

Good eval harnesses:
- Run automatically on every change
- Have deterministic success criteria
- Catch regressions before deployment
- Provide actionable failure information
- Track reliability over time

Without evals, you're flying blind.

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about what could go wrong with a support agent
- Consider: what would a "silent failure" look like?

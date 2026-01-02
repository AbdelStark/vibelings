# Hint 1: Basic Structure

Your output needs this structure:

```json
{
  "eval_harness": {
    "name": "Customer Support Agent Eval",
    "agent_under_test": {...},
    "test_cases": [...],
    "metrics": [...],
    "reliability": {...},
    "regression_detection": {...}
  }
}
```

---

# Hint 2: Agent Under Test

Describe what you're testing:

```json
{
  "agent_under_test": {
    "name": "customer_support_agent",
    "capabilities": ["ticket_classification", "response_generation"],
    "model": "claude-sonnet-4-20250514"
  }
}
```

---

# Hint 3: Test Cases

You need at least 3 test cases with different types:

```json
{
  "id": "billing_inquiry",
  "input": "How do I update my payment method?",
  "expected": {"category": "billing", "priority": "low"},
  "type": "deterministic"
}
```

Types: "deterministic", "semantic", "behavioral"

---

# Hint 4: Metrics

You need at least 2 metrics:

```json
[
  {
    "name": "classification_accuracy",
    "type": "accuracy",
    "threshold": 0.95,
    "aggregation": "mean"
  },
  {
    "name": "response_latency",
    "type": "latency",
    "threshold": 2000,
    "aggregation": "p95"
  }
]
```

---

# Hint 5: Complete Solution

```json
{
  "eval_harness": {
    "name": "Customer Support Agent Eval",
    "agent_under_test": {
      "name": "customer_support_agent",
      "capabilities": ["ticket_classification", "response_generation"],
      "model": "claude-sonnet-4-20250514"
    },
    "test_cases": [
      {
        "id": "billing_inquiry",
        "input": "How do I update my payment method?",
        "expected": {"category": "billing", "priority": "low"},
        "type": "deterministic"
      },
      {
        "id": "urgent_outage",
        "input": "The entire system is down, we can't process any orders!",
        "expected": {"category": "technical", "priority": "critical"},
        "type": "deterministic"
      },
      {
        "id": "polite_response",
        "input": "I'm frustrated with your service",
        "expected": "empathetic acknowledgment with solution offer",
        "type": "semantic"
      }
    ],
    "metrics": [
      {
        "name": "classification_accuracy",
        "type": "accuracy",
        "threshold": 0.95,
        "aggregation": "mean"
      },
      {
        "name": "response_latency_ms",
        "type": "latency",
        "threshold": 2000,
        "aggregation": "p95"
      }
    ],
    "reliability": {
      "runs_per_case": 5,
      "pass_threshold": 0.8,
      "confidence_interval": 0.95
    },
    "regression_detection": {
      "baseline": "v1.2.0",
      "tolerance": 0.05,
      "alert_on_degradation": true
    }
  }
}
```

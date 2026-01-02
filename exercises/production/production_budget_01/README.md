# Cost and Latency Budgets

## Goal

Design a budget configuration that enforces cost limits and latency SLOs
for agent operations, with graceful degradation when limits are approached.

## Why This Matters

Production agents can become expensive and slow without proper controls:
- A single runaway agent can burn through your monthly API budget
- Unbounded retries can cause latency to spiral
- Users expect predictable response times
- Cost overruns are hard to detect until the bill arrives

Budget enforcement is operational hygiene, not optional optimization.

## The Task

Design a budget configuration for a customer-facing agent that must:
- Stay within per-request and daily cost limits
- Meet latency SLOs (P95 < 3 seconds)
- Degrade gracefully when approaching limits
- Alert when budgets are at risk

## Budget Configuration Structure

```json
{
  "budget_config": {
    "name": "Budget Profile Name",
    "cost_limits": {...},
    "latency_slos": {...},
    "token_limits": {...},
    "degradation_policy": {...},
    "monitoring": {...}
  }
}
```

## Required Components

### 1. Cost Limits
```json
{
  "cost_limits": {
    "per_request_max_usd": 0.10,
    "daily_limit_usd": 100.00,
    "monthly_limit_usd": 2000.00,
    "alert_thresholds": [0.5, 0.8, 0.95]
  }
}
```

### 2. Latency SLOs
```json
{
  "latency_slos": {
    "p50_ms": 1000,
    "p95_ms": 3000,
    "p99_ms": 5000,
    "timeout_ms": 10000
  }
}
```

### 3. Token Limits
```json
{
  "token_limits": {
    "max_input_tokens": 4096,
    "max_output_tokens": 2048,
    "max_total_tokens": 8192
  }
}
```

### 4. Degradation Policy
What to do when approaching limits:
```json
{
  "degradation_policy": {
    "on_cost_warning": [...],
    "on_latency_warning": [...],
    "on_limit_reached": "reject"
  }
}
```

### 5. Monitoring
```json
{
  "monitoring": {
    "metrics_to_track": [...],
    "alert_channels": [...],
    "dashboard_enabled": true
  }
}
```

## Degradation Strategies

| Trigger | Strategy | Effect |
|---------|----------|--------|
| 80% daily budget | use_smaller_model | Switch to cheaper model |
| 95% daily budget | reduce_context | Trim context window |
| P95 latency warning | skip_optional_tools | Disable non-critical tools |
| Budget exhausted | reject_new_requests | Return graceful error |

## Grading

Your output is validated against:

1. **Cost limits** - Must have per-request, daily, and monthly limits
2. **Latency SLOs** - Must have P50, P95, P99, and timeout
3. **Token limits** - Must have input, output, and total limits
4. **Degradation** - Must define responses to warnings
5. **Monitoring** - Must have metrics and alerts configured

## Key Lesson

**Budgets are guardrails, not targets.**

Effective budget management:
- Sets limits before problems occur
- Degrades gracefully instead of failing hard
- Alerts early so you can respond
- Tracks spend for capacity planning
- Enforces limits automatically (humans forget)

The goal is predictable operations, not penny-pinching.

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think: "What happens at 3am when no one is watching?"
- Consider: graceful degradation > hard failures

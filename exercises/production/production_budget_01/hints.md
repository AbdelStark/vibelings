# Hint 1: Basic Structure

Your output needs this structure:

```json
{
  "budget_config": {
    "name": "Production Budget Profile",
    "cost_limits": {...},
    "latency_slos": {...},
    "token_limits": {...},
    "degradation_policy": {...},
    "monitoring": {...}
  }
}
```

---

# Hint 2: Cost Limits

Set reasonable limits at multiple levels:

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

---

# Hint 3: Latency SLOs

Define percentile targets:

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

The timeout should be higher than p99.

---

# Hint 4: Degradation Policy

Define actions for each warning level:

```json
{
  "degradation_policy": {
    "on_cost_warning": ["use_smaller_model", "reduce_context"],
    "on_latency_warning": ["skip_optional_tools", "reduce_retries"],
    "on_limit_reached": "reject"
  }
}
```

on_limit_reached must be: "reject", "queue", "degrade", or "fallback"

---

# Hint 5: Complete Solution

```json
{
  "budget_config": {
    "name": "Customer Agent Budget v1",
    "cost_limits": {
      "per_request_max_usd": 0.10,
      "daily_limit_usd": 100.00,
      "monthly_limit_usd": 2000.00,
      "alert_thresholds": [0.5, 0.8, 0.95]
    },
    "latency_slos": {
      "p50_ms": 1000,
      "p95_ms": 3000,
      "p99_ms": 5000,
      "timeout_ms": 10000
    },
    "token_limits": {
      "max_input_tokens": 4096,
      "max_output_tokens": 2048,
      "max_total_tokens": 8192
    },
    "degradation_policy": {
      "on_cost_warning": ["use_smaller_model", "reduce_context_window"],
      "on_latency_warning": ["skip_optional_tools", "disable_retries"],
      "on_limit_reached": "reject"
    },
    "monitoring": {
      "metrics_to_track": [
        "cost_per_request",
        "daily_spend",
        "p95_latency",
        "error_rate",
        "token_usage"
      ],
      "alert_channels": ["slack", "pagerduty"],
      "dashboard_enabled": true
    }
  }
}
```

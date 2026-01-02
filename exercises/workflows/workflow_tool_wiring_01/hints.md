# Hint 1: Pipeline Structure

Your output must follow this structure:

```json
{
  "pipeline": {
    "name": "Order Processing Pipeline",
    "steps": [...],
    "error_handling": {...}
  }
}
```

---

# Hint 2: Step Structure

Each step needs four required fields:

```json
{
  "id": "step_id",
  "tool": "tool_name",
  "input_mapping": { ... },
  "output_schema": { ... }
}
```

The five required step IDs are:
- fetch_order
- transform_data
- enrich_customer
- validate_order
- format_output

---

# Hint 3: Data References

Use `{{step_id.field}}` syntax to reference previous step outputs:

```json
{
  "id": "enrich_customer",
  "tool": "crm_lookup",
  "input_mapping": {
    "customer_id": "{{transform_data.customer_id}}"
  }
}
```

---

# Hint 4: Error Handling

The error_handling section needs three parts:

```json
{
  "error_handling": {
    "on_step_failure": "retry",
    "retry_policy": {
      "max_retries": 3,
      "backoff": "exponential"
    },
    "fallback": {
      "action": "queue_for_manual_review",
      "notify": ["ops-team@example.com"]
    }
  }
}
```

---

# Hint 5: Complete Solution

```json
{
  "pipeline": {
    "name": "Order Processing Pipeline",
    "steps": [
      {
        "id": "fetch_order",
        "tool": "http_request",
        "input_mapping": {
          "url": "{{trigger.order_url}}",
          "method": "GET"
        },
        "output_schema": {
          "order_id": "string",
          "customer_id": "string",
          "items": "array",
          "total": "number"
        }
      },
      {
        "id": "transform_data",
        "tool": "data_transform",
        "input_mapping": {
          "source": "{{fetch_order}}"
        },
        "output_schema": {
          "order_id": "string",
          "customer_id": "string",
          "line_items": "array",
          "amount_cents": "integer"
        }
      },
      {
        "id": "enrich_customer",
        "tool": "crm_lookup",
        "input_mapping": {
          "customer_id": "{{transform_data.customer_id}}"
        },
        "output_schema": {
          "customer_name": "string",
          "customer_email": "string",
          "tier": "string"
        }
      },
      {
        "id": "validate_order",
        "tool": "validator",
        "input_mapping": {
          "order": "{{transform_data}}",
          "customer": "{{enrich_customer}}"
        },
        "output_schema": {
          "valid": "boolean",
          "errors": "array"
        },
        "conditions": [
          {"field": "amount_cents", "operator": "gt", "value": 0},
          {"field": "line_items", "operator": "not_empty"}
        ]
      },
      {
        "id": "format_output",
        "tool": "formatter",
        "input_mapping": {
          "order": "{{transform_data}}",
          "customer": "{{enrich_customer}}",
          "validation": "{{validate_order}}"
        },
        "output_schema": {
          "processed_order": "object",
          "timestamp": "string"
        }
      }
    ],
    "error_handling": {
      "on_step_failure": "retry",
      "retry_policy": {
        "max_retries": 3,
        "backoff": "exponential"
      },
      "fallback": {
        "action": "queue_for_manual_review",
        "notify": ["ops@example.com"]
      }
    }
  }
}
```

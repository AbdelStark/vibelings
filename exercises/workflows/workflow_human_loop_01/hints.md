# Hint 1: Workflow Structure

Your output needs this structure:

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

---

# Hint 2: Steps

You need 4 steps with these IDs:
- receive_content
- classify_content
- route_decision
- apply_action

Each step needs at least `id` and `action` fields.

---

# Hint 3: Approval Gate Structure

An approval gate needs all these fields:

```json
{
  "id": "human_review",
  "trigger_condition": "confidence < 0.8",
  "request_to": ["queue_name"],
  "context_fields": ["field1", "field2"],
  "timeout_seconds": 3600,
  "outcomes": {
    "approved": { "next_step": "...", "action": "..." },
    "rejected": { "next_step": "...", "action": "..." },
    "timeout": { "next_step": "...", "action": "..." }
  }
}
```

---

# Hint 4: Timeout Handling

```json
{
  "timeout_handling": {
    "default_action": "hold_for_review",
    "escalation": {
      "enabled": true,
      "escalate_to": ["senior_moderator_queue"],
      "notify": ["team-lead@example.com"]
    }
  }
}
```

---

# Hint 5: Complete Solution

```json
{
  "workflow": {
    "name": "Content Moderation",
    "steps": [
      {
        "id": "receive_content",
        "action": "ingest",
        "input": ["content_id", "content_text", "user_id", "timestamp"]
      },
      {
        "id": "classify_content",
        "action": "ml_classify",
        "model": "content_safety_v2",
        "output": ["classification", "confidence", "categories"]
      },
      {
        "id": "route_decision",
        "action": "conditional_route",
        "conditions": [
          {"if": "confidence >= 0.95 AND classification == 'safe'", "goto": "apply_action", "with": {"decision": "auto_approve"}},
          {"if": "confidence >= 0.95 AND classification == 'unsafe'", "goto": "apply_action", "with": {"decision": "auto_reject"}},
          {"if": "confidence < 0.95", "goto": "human_review"}
        ]
      },
      {
        "id": "apply_action",
        "action": "execute_decision",
        "actions": {
          "publish": "make_content_visible",
          "remove": "delete_content",
          "hold": "queue_for_later_review"
        }
      }
    ],
    "approval_gates": [
      {
        "id": "human_review",
        "trigger_condition": "confidence < 0.95",
        "request_to": ["moderator_queue"],
        "context_fields": ["content_id", "content_text", "classification", "confidence", "categories"],
        "timeout_seconds": 3600,
        "outcomes": {
          "approved": {"next_step": "apply_action", "action": "publish"},
          "rejected": {"next_step": "apply_action", "action": "remove"},
          "timeout": {"next_step": "apply_action", "action": "hold"}
        }
      }
    ],
    "timeout_handling": {
      "default_action": "hold_for_review",
      "escalation": {
        "enabled": true,
        "escalate_to": ["senior_moderator_queue"],
        "notify": ["moderation-leads@example.com"]
      }
    }
  }
}
```

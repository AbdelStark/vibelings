# Hint 1: Basic Structure

Your workflow JSON needs three top-level keys:

```json
{
  "name": "...",
  "nodes": [...],
  "connections": {...}
}
```

---

# Hint 2: Node Structure

Each node needs these fields:

```json
{
  "id": "unique-id-here",
  "name": "Display Name",
  "type": "n8n-nodes-base.nodeType",
  "position": [0, 0],
  "parameters": {}
}
```

The three node types you need are:
- `n8n-nodes-base.webhook`
- `n8n-nodes-base.if`
- `n8n-nodes-base.postgres`

---

# Hint 3: Connections Structure

Connections use source node names as keys:

```json
{
  "connections": {
    "Source Node Name": {
      "main": [
        [
          {
            "node": "Target Node Name",
            "type": "main",
            "index": 0
          }
        ]
      ]
    }
  }
}
```

You need two connections: Webhook -> Validate and Validate -> Store.

---

# Hint 4: Complete Solution

```json
{
  "name": "User Signup Processing",
  "nodes": [
    {
      "id": "webhook-1",
      "name": "Signup Webhook",
      "type": "n8n-nodes-base.webhook",
      "position": [0, 0],
      "parameters": {
        "path": "signup"
      }
    },
    {
      "id": "if-1",
      "name": "Validate Email",
      "type": "n8n-nodes-base.if",
      "position": [200, 0],
      "parameters": {
        "conditions": {
          "string": [
            {
              "value1": "={{$json.email}}",
              "operation": "contains",
              "value2": "@"
            }
          ]
        }
      }
    },
    {
      "id": "postgres-1",
      "name": "Store User",
      "type": "n8n-nodes-base.postgres",
      "position": [400, 0],
      "parameters": {
        "operation": "insert"
      }
    }
  ],
  "connections": {
    "Signup Webhook": {
      "main": [
        [
          {
            "node": "Validate Email",
            "type": "main",
            "index": 0
          }
        ]
      ]
    },
    "Validate Email": {
      "main": [
        [
          {
            "node": "Store User",
            "type": "main",
            "index": 0
          }
        ]
      ]
    }
  }
}
```

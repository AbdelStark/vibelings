# Workflow JSON Schema

## Goal

Generate a valid n8n-style workflow JSON that defines nodes and their connections
for a simple data processing pipeline.

## Why This Matters

Workflow orchestration tools like n8n use JSON to represent workflows as graphs:
- **Nodes** represent individual operations (HTTP requests, data transforms, etc.)
- **Connections** define the data flow between nodes
- **Parameters** configure each node's behavior

Understanding workflow JSON structure is essential for:
1. **Programmatic workflow creation** - generating workflows from code
2. **Workflow import/export** - moving workflows between environments
3. **Workflow validation** - ensuring workflows are syntactically correct
4. **Debugging** - understanding why a workflow behaves unexpectedly

## The Task

Create a workflow that processes new user signups:

1. **Trigger**: Webhook receives signup data
2. **Validate**: Check if email format is valid
3. **Store**: Save to database

## Workflow Structure

An n8n workflow JSON has this core structure:

```json
{
  "name": "Workflow Name",
  "nodes": [
    {
      "id": "unique-node-id",
      "name": "Node Display Name",
      "type": "n8n-nodes-base.nodeType",
      "position": [x, y],
      "parameters": { ... }
    }
  ],
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

## Required Nodes

Your workflow must include these three nodes:

### 1. Webhook Node (Trigger)
- Type: `n8n-nodes-base.webhook`
- Name: `Signup Webhook`
- Parameters: `path` set to `signup`

### 2. Validation Node
- Type: `n8n-nodes-base.if`
- Name: `Validate Email`
- Parameters: `conditions` with email validation

### 3. Database Node
- Type: `n8n-nodes-base.postgres`
- Name: `Store User`
- Parameters: `operation` set to `insert`

## Expected Connections

```
Signup Webhook -> Validate Email -> Store User
```

Each connection flows from `main` output to `main` input.

## Grading

Your output is validated against:

1. **Workflow metadata** - Must have `name` field
2. **Node structure** - Each node must have required fields (id, name, type, position)
3. **Node types** - Must use the specified node types
4. **Connections** - Must properly connect all three nodes in sequence

## Key Lesson

**Workflows are graphs with contracts.** Each node has:
- A type that determines its capabilities
- Parameters that configure its behavior
- Inputs and outputs that define data flow

Understanding this structure lets you:
- Generate workflows programmatically
- Validate workflows before deployment
- Debug connection issues
- Migrate workflows between systems

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Check `grader/schema.json` for exact structure requirements
- Node positions are just for visual layout - use any valid [x, y] coordinates

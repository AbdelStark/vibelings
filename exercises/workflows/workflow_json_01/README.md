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

## The Concept: Workflows as Directed Graphs

A workflow is a directed acyclic graph (DAG) where:
- Nodes are operations
- Edges are data flow paths
- Execution follows the edges

```
┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
│  Webhook        │─────►│  Validate       │─────►│  Store          │
│  (trigger)      │      │  (transform)    │      │  (action)       │
└─────────────────┘      └─────────────────┘      └─────────────────┘
       │                        │                        │
       ▼                        ▼                        ▼
    receives              checks data               writes data
    HTTP POST              format                   to database
```

**The key insight**: The same workflow can be represented multiple ways (visual UI, JSON, code), but the underlying structure is always a graph. JSON is just one serialization format.

### Why JSON for Workflows?

| Property | Benefit |
|----------|---------|
| Human-readable | Can be reviewed in code review |
| Machine-parsable | Can be validated, transformed, generated |
| Portable | Works across systems and languages |
| Versionable | Can be stored in git with meaningful diffs |

This is why every major workflow tool (n8n, Airflow, GitHub Actions) uses JSON/YAML — it balances human and machine needs.

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

## Common Mistakes

**1. Forgetting connection direction**
```json
{"Validate Email": {"main": [[{"node": "Signup Webhook"}]]}}  // Wrong: backwards
{"Signup Webhook": {"main": [[{"node": "Validate Email"}]]}}  // Correct: source → target
```
Connections go FROM source TO target. The key is the source node, the value specifies where data flows.

**2. Mismatched node names**
```json
{"nodes": [{"name": "Signup Webhook"}], "connections": {"SignupWebhook": {...}}}  // Wrong: names don't match
```
Node names in connections must exactly match node names in the nodes array. Case and spaces matter.

**3. Missing position coordinates**
Every node needs a `position: [x, y]` for the visual editor. Even if you're generating workflows programmatically, include positions or the UI will break.

**4. Wrong connection structure**
```json
{"Source": {"main": [{"node": "Target"}]}}      // Wrong: missing array level
{"Source": {"main": [[{"node": "Target"}]]}}    // Correct: array of arrays
```
The nested arrays allow for multiple outputs (branches) from a single node.

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

## Connections

- **Prerequisite**: [fundamentals/json_01](../../fundamentals/json_01/) — JSON schema basics
- **Next**: [workflow_tool_wiring_01](../workflow_tool_wiring_01/) — data flow between steps
- **Related**: [fundamentals/tools_01](../../fundamentals/tools_01/) — nodes are similar to tools

## Further Reading

- [n8n Documentation](https://docs.n8n.io/) — Workflow automation platform
- [Airflow Concepts](https://airflow.apache.org/docs/apache-airflow/stable/core-concepts/dags.html) — DAGs in workflow orchestration
- [GitHub Actions Syntax](https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions) — Another workflow JSON/YAML format

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Check `grader/schema.json` for exact structure requirements
- Node positions are just for visual layout - use any valid [x, y] coordinates

# Tracing and Cost Awareness

## Goal

Demonstrate proper observability patterns for agentic workflows, including
structured traces, logging, and cost estimation for each operation.

## Why This Matters

Production agentic systems must be observable. Without observability you cannot:

1. **Debug failures** — What happened? What was the sequence of calls?
2. **Optimize performance** — Where is time being spent?
3. **Control costs** — How much does each request cost?
4. **Audit actions** — What did the agent do and why?

LLMs are non-deterministic. The same prompt might produce different outputs, different tool call sequences, different costs. Without observability, you're flying blind.

This exercise teaches the observability discipline that makes agents maintainable and cost-effective.

## The Concept: Distributed Tracing for Agents

Agentic workflows are like microservice architectures: multiple components (LLM, tools, external services) working together. We use the same observability patterns:

```
                    trace_id: abc-123
┌──────────────────────────────────────────────────┐
│                                                  │
│  span_id: span-1          span_id: span-2        │
│  parent: null             parent: span-1         │
│  operation: read_doc      operation: summarize   │
│  duration: 100ms          duration: 2000ms       │
│  cost: $0.001             cost: $0.01            │
│                                                  │
│                           span_id: span-3        │
│                           parent: span-2         │
│                           operation: send_msg    │
│                           duration: 50ms         │
│                           cost: $0.0001          │
│                                                  │
└──────────────────────────────────────────────────┘
```

**Trace**: The entire request, end-to-end
**Span**: A single operation within the trace
**Propagation**: Child spans reference their parent

With this structure, you can:
- Follow a request through all its operations
- See exactly where time was spent
- Know exactly what each request cost
- Reconstruct the sequence of events for debugging

## The Task

You are building an agent that processes document requests. For each tool call,
you must include observability metadata: trace IDs, timing estimates, and cost
projections.

**Task**: "Summarize the Q3 report and send it to the team lead."

## Available Tools

### `read_document`
Read a document from storage.

**Parameters**:
- `document_id` (string, required): Document identifier
- `format` (string, optional): "text" or "markdown"

**Cost**: ~0.001 USD per KB read

### `summarize_text`
Generate a summary using LLM.

**Parameters**:
- `text` (string, required): Text to summarize
- `max_length` (integer, optional): Maximum summary length in words

**Cost**: ~0.01 USD per 1K tokens processed

### `send_message`
Send a message to a recipient.

**Parameters**:
- `to` (string, required): Recipient identifier
- `subject` (string, required): Message subject
- `body` (string, required): Message body

**Cost**: ~0.0001 USD per message

## Expected Output Format

Each tool call must include observability metadata:

```json
{
  "tool_calls": [
    {
      "name": "tool_name",
      "arguments": {...},
      "trace": {
        "trace_id": "unique-trace-id",
        "span_id": "unique-span-id",
        "parent_span_id": "parent-id-or-null",
        "operation": "human-readable operation name"
      },
      "timing": {
        "estimated_duration_ms": 500,
        "timeout_ms": 5000
      },
      "cost": {
        "estimated_usd": 0.01,
        "cost_factors": ["tokens: 1000", "storage: 10KB"]
      }
    }
  ],
  "workflow_metadata": {
    "total_estimated_cost_usd": 0.025,
    "total_estimated_duration_ms": 2000,
    "trace_id": "workflow-level-trace-id"
  }
}
```

## Observability Patterns

### Pattern 1: Trace Propagation
```json
"trace": {
  "trace_id": "abc-123",
  "span_id": "span-1",
  "parent_span_id": null,
  "operation": "read_q3_report"
}
```

Each subsequent call uses the same trace_id but new span_id, with parent_span_id
pointing to the previous operation.

### Pattern 2: Cost Estimation
```json
"cost": {
  "estimated_usd": 0.01,
  "cost_factors": ["tokens: 1000 @ $0.01/1K"]
}
```

### Pattern 3: Timing Budgets
```json
"timing": {
  "estimated_duration_ms": 500,
  "timeout_ms": 5000
}
```

## Common Mistakes

**1. Different trace_id for each span**
```json
{"trace_id": "abc-123", "span_id": "span-1"}
{"trace_id": "def-456", "span_id": "span-2"}  // Wrong: different trace
```
All spans in a workflow share the same trace_id. That's what makes them traceable as a unit.

**2. Missing parent relationships**
```json
{"span_id": "span-2", "parent_span_id": null}  // Wrong: should reference span-1
```
Span hierarchy shows causality. Span-2 happened because span-1 completed.

**3. No cost factors**
Just `"estimated_usd": 0.01` isn't enough. Include what factors drive the cost—helps with optimization.

**4. Missing totals**
Individual span costs are useful, but the workflow total is what shows up on the bill.

**5. Unrealistic timeout/duration ratios**
```json
{"estimated_duration_ms": 500, "timeout_ms": 501}  // Wrong: too tight
```
Timeouts should be 5-10x expected duration to handle variance.

## Grading

Your output is validated for:

1. **Tool schema compliance** — All arguments valid
2. **Trace consistency** — All operations share trace_id, proper span hierarchy
3. **Cost awareness** — Each call has cost estimate, total calculated
4. **Timing budgets** — Each call has estimated duration and timeout
5. **Workflow metadata** — Totals are present and consistent

## Key Lessons

1. **Trace everything** — Every operation needs a trace context
2. **Estimate costs upfront** — Know what you'll spend before spending it
3. **Set timeouts** — Every operation needs a time budget
4. **Structured logs** — Use structured data, not free text
5. **Span hierarchy** — Track parent-child relationships between operations

**The practical value**: When something breaks at 3 AM, you'll have a trace showing exactly what happened. When costs spike, you'll see which operations are responsible. When users complain about latency, you'll know where time is being spent.

## Real-World Application

In production systems:
- Trace IDs are used to correlate logs across distributed systems
- Cost estimates are used for budgeting and alerts
- Timing data feeds into SLO dashboards
- Structured logs enable automated analysis

## Connections

- **Prerequisite**: [tools_02](../tools_02/) introduces multi-step workflows
- **Related**: [production/production_budget_01](../../production/production_budget_01/) focuses on cost management
- **Production**: Observability enables [production/production_eval_01](../../production/production_eval_01/)

## Further Reading

- [OpenTelemetry](https://opentelemetry.io/) — The standard for distributed tracing
- [Anthropic: Usage tracking](https://docs.anthropic.com/en/docs/build-with-claude/usage) — Monitoring Claude API usage
- [Honeycomb observability](https://www.honeycomb.io/observability-101) — Introduction to observability principles
- [Google SRE book: Monitoring](https://sre.google/sre-book/monitoring-distributed-systems/) — Monitoring distributed systems

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about how you'd debug a failure in this workflow
- Consider what someone looking at traces would need to see
- Remember: every operation is a span in the trace
- All spans share the same trace_id, but have unique span_ids

# Hint 1: Trace Structure

All operations in a workflow share the same `trace_id`.
Each operation gets a unique `span_id`.
The first operation has `parent_span_id: null`.

```json
"trace": {
  "trace_id": "workflow-abc-123",
  "span_id": "span-001",
  "parent_span_id": null,
  "operation": "read_q3_report"
}
```

---

# Hint 2: Span Hierarchy

For sequential operations:
- First operation: parent_span_id = null
- Second operation: parent_span_id = first span_id
- Third operation: parent_span_id = second span_id

This shows the execution chain.

---

# Hint 3: Timing Estimates

Use realistic estimates based on operation type:
- read_document: ~200ms, timeout 5000ms
- summarize_text: ~2000ms (LLM call), timeout 30000ms
- send_message: ~100ms, timeout 5000ms

Always set timeout > estimated_duration.

---

# Hint 4: Cost Factors

Document what drives the cost:
```json
"cost": {
  "estimated_usd": 0.01,
  "cost_factors": ["~1K tokens at $0.01/1K tokens"]
}
```

---

# Hint 5: Complete Solution

```json
{
  "tool_calls": [
    {
      "name": "read_document",
      "arguments": {
        "document_id": "q3-report-2024",
        "format": "text"
      },
      "trace": {
        "trace_id": "wf-doc-summary-001",
        "span_id": "span-read-001",
        "parent_span_id": null,
        "operation": "read_q3_report"
      },
      "timing": {
        "estimated_duration_ms": 200,
        "timeout_ms": 5000
      },
      "cost": {
        "estimated_usd": 0.005,
        "cost_factors": ["~5KB document at $0.001/KB"]
      }
    },
    {
      "name": "summarize_text",
      "arguments": {
        "text": "{{document_content}}",
        "max_length": 200
      },
      "trace": {
        "trace_id": "wf-doc-summary-001",
        "span_id": "span-summarize-002",
        "parent_span_id": "span-read-001",
        "operation": "summarize_report"
      },
      "timing": {
        "estimated_duration_ms": 2000,
        "timeout_ms": 30000
      },
      "cost": {
        "estimated_usd": 0.02,
        "cost_factors": ["~2K tokens in + out at $0.01/1K"]
      }
    },
    {
      "name": "send_message",
      "arguments": {
        "to": "team-lead",
        "subject": "Q3 Report Summary",
        "body": "{{summary_content}}"
      },
      "trace": {
        "trace_id": "wf-doc-summary-001",
        "span_id": "span-send-003",
        "parent_span_id": "span-summarize-002",
        "operation": "send_summary_to_lead"
      },
      "timing": {
        "estimated_duration_ms": 100,
        "timeout_ms": 5000
      },
      "cost": {
        "estimated_usd": 0.0001,
        "cost_factors": ["1 message at $0.0001/message"]
      }
    }
  ],
  "workflow_metadata": {
    "total_estimated_cost_usd": 0.0251,
    "total_estimated_duration_ms": 2300,
    "trace_id": "wf-doc-summary-001"
  }
}
```

Note: All operations share trace_id "wf-doc-summary-001" and have proper parent-child span relationships.

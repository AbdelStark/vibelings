# Tool Wiring Patterns

## Goal

Design a tool pipeline that demonstrates proper data transformation and error handling
between workflow steps.

## Why This Matters

Real-world workflows rarely consist of isolated tools. Instead, they form pipelines where:
- Output from one tool becomes input to another
- Data must be transformed between steps
- Errors at any step must be handled gracefully
- Each step's contract must be satisfied

Understanding tool wiring patterns is essential for building reliable agentic systems.

## The Task

Design a pipeline for processing and enriching customer orders:

1. **Fetch Order** - Retrieve order from API
2. **Transform Data** - Convert to internal format
3. **Enrich** - Add customer details from CRM
4. **Validate** - Check business rules
5. **Output** - Format final result

## Pipeline Specification

Your output must be a JSON object describing the pipeline:

```json
{
  "pipeline": {
    "name": "Pipeline Name",
    "steps": [...],
    "error_handling": {...}
  }
}
```

## Required Steps

### Step 1: fetch_order
```json
{
  "id": "fetch_order",
  "tool": "http_request",
  "input_mapping": {
    "url": "{{trigger.order_url}}",
    "method": "GET"
  },
  "output_schema": {
    "order_id": "string",
    "items": "array",
    "total": "number"
  }
}
```

### Step 2: transform_data
Converts external order format to internal format.

### Step 3: enrich_customer
Fetches customer data from CRM using order's customer_id.

### Step 4: validate_order
Checks that total > 0 and items is non-empty.

### Step 5: format_output
Combines all data into final output structure.

## Data Flow Rules

1. Each step receives the previous step's output
2. Input mappings use `{{step_id.field}}` syntax for references
3. Each step must declare its output schema
4. Validation steps should include `conditions` array

## Error Handling

Your pipeline must include an `error_handling` section with:
- `on_step_failure`: What to do when a step fails
- `retry_policy`: When to retry failed steps
- `fallback`: Final fallback action

## Grading

Your output is validated against:

1. **Pipeline structure** - Must have name, steps, and error_handling
2. **Step structure** - Each step needs id, tool, input_mapping, output_schema
3. **Data references** - Input mappings must use valid step references
4. **Error handling** - Must include required error handling fields

## Key Lesson

**Wiring tools is about contracts at boundaries.** Each step:
- Declares what it expects (input_mapping)
- Declares what it produces (output_schema)
- References previous steps explicitly

This explicit data flow makes pipelines:
- Debuggable - you can trace data through each step
- Testable - each step can be tested in isolation
- Reliable - schema mismatches are caught at design time

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about what data flows between each step
- Error handling is about graceful degradation, not just retries

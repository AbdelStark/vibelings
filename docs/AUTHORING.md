# Exercise Authoring Guide

> How to create exercises for vibelings

This guide explains how to create new exercises for vibelings. Exercises are the core learning unit - each one teaches a specific concept about building reliable agentic systems.

## Quick Start

1. Create exercise directory: `exercises/<track>/<exercise_id>/`
2. Add `manifest.toml` with metadata
3. Write `README.md` with learning objectives
4. Create grader in `grader/` directory
5. Add `starter/prompt.txt` for learner input
6. Add `hints.md` for progressive hints
7. Test with `cargo run -- run <track>/<exercise_id>`

## Directory Structure

```
exercises/
└── <track>/                    # Track name (fundamentals, mcp, workflows, production)
    └── <exercise_id>/          # Exercise ID (e.g., json_01, server_01)
        ├── manifest.toml       # Exercise configuration (required)
        ├── README.md           # Exercise description (required)
        ├── hints.md            # Progressive hints (recommended)
        ├── starter/            # Starter files for learner
        │   └── prompt.txt      # Default prompt file
        ├── grader/             # Grading files
        │   ├── schema.json     # JSON Schema for validation
        │   └── *.sh            # Invariant check scripts
        └── fixtures/           # Deterministic mock data (optional)
            └── *.json
```

## Tracks

| Track | Directory | Focus |
|-------|-----------|-------|
| Agentic Fundamentals | `fundamentals` | Core primitives: JSON schemas, tool calling, error handling |
| MCP in Practice | `mcp` | Model Context Protocol: servers, clients, resources |
| Workflow Orchestration | `workflows` | Integration with workflow tools |
| Production Engineering | `production` | Reliability at scale: evals, security, cost management |

## manifest.toml

The manifest defines exercise metadata and grading configuration.

### Full Example

```toml
[exercise]
id = "json_01"                          # Unique ID (snake_case)
title = "Basic JSON Output"             # Human-readable title
track = "fundamentals"                  # Track: fundamentals, mcp, workflows, production
prerequisites = []                      # List of exercise IDs that must be completed first
description = "Learn JSON schema validation"  # Short description
difficulty = 1                          # 1-5 scale

[requirements]
tool_calling = false                    # Does exercise require tool calling?
json_mode = true                        # Does exercise require JSON mode?
min_context_window = 4096               # Minimum context window size
network = false                         # Does exercise require network access?

[run]
max_tool_calls = 0                      # Maximum tool calls allowed (0 = unlimited)
timeout_seconds = 30                    # Timeout for exercise execution
runs = 1                                # Number of runs (for reliability exercises)
required_passes = 4                     # Required passes out of `runs` (optional)

[grader]
type = "schema"                         # Grader type (see below)
schema_path = "schema.json"             # Path to schema file (relative to grader/)
invariants = ["check.sh"]               # List of invariant scripts (optional)
```

### Grader Types

| Type | Description | When to Use |
|------|-------------|-------------|
| `schema` | JSON Schema validation | Structured output validation |
| `sandbox` | Tool calling validation | Tool use exercises |
| `invariants` | Shell script checks | Custom validation logic |
| `combined` | Schema + invariants | Complex validation |
| `reliability` | Multi-run checking | Stochastic tasks |
| `llm_judge` | LLM-based evaluation | Last resort only |

**Always prefer deterministic grading (schema, sandbox, invariants) over LLM-as-judge.**

## Grader: Schema

For exercises requiring structured JSON output.

### schema.json Example

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Person",
  "type": "object",
  "properties": {
    "name": {
      "type": "string",
      "minLength": 1
    },
    "age": {
      "type": "integer",
      "minimum": 0,
      "maximum": 150
    },
    "email": {
      "type": "string",
      "format": "email"
    }
  },
  "required": ["name", "age", "email"],
  "additionalProperties": false
}
```

### manifest.toml

```toml
[grader]
type = "schema"
schema_path = "schema.json"
```

## Grader: Sandbox (Tool Calling)

For exercises involving tool definitions and calls.

### tools_schema.json Example

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "description": "Weather assistant tools",
  "tools": [
    {
      "name": "get_weather",
      "description": "Get current weather for a location",
      "parameters": {
        "type": "object",
        "properties": {
          "location": {
            "type": "string",
            "minLength": 1
          },
          "units": {
            "type": "string",
            "enum": ["celsius", "fahrenheit"]
          }
        },
        "required": ["location"],
        "additionalProperties": false
      }
    }
  ]
}
```

### Expected Output Format

The learner's output should be:

```json
{
  "tool_calls": [
    {
      "name": "get_weather",
      "arguments": {
        "location": "San Francisco",
        "units": "fahrenheit"
      }
    }
  ]
}
```

### manifest.toml

```toml
[grader]
type = "sandbox"
schema_path = "tools_schema.json"
```

## Grader: Invariants

For custom validation using shell scripts.

### check.sh Example

```bash
#!/bin/bash
# check.sh - Invariant check script

# Read the learner's output from stdin
OUTPUT=$(cat)

# Check for required content
if echo "$OUTPUT" | grep -q "expected_content"; then
    echo "PASS: Found expected content"
    exit 0
else
    echo "FAIL: Missing expected content"
    exit 1
fi
```

### manifest.toml

```toml
[grader]
type = "invariants"
invariants = ["check.sh"]
```

### Multiple Invariants

```toml
[grader]
type = "invariants"
invariants = ["check_format.sh", "check_content.sh", "check_constraints.sh"]
```

All invariant scripts must pass for the exercise to pass.

## Grader: Combined

Use both schema validation and invariant checks.

```toml
[grader]
type = "combined"
schema_path = "schema.json"
invariants = ["check_semantics.sh"]
```

## Writing README.md

Every exercise needs a clear README with:

1. **Goal** - What the learner will accomplish
2. **Why This Matters** - Real-world relevance
3. **The Task** - Specific instructions
4. **Expected Output Format** - Clear examples
5. **Grading** - What's being validated
6. **Key Lesson** - Core takeaway

### Template

```markdown
# Exercise Title

## Goal

[One sentence describing the learning objective]

## Why This Matters

[2-3 sentences on real-world relevance]

## The Task

[Specific instructions for what to do]

## Expected Output Format

[JSON or code examples showing expected format]

## Grading

Your output is validated against:

1. **Criteria 1** - Description
2. **Criteria 2** - Description
3. **Criteria 3** - Description

## Key Lesson

[One paragraph summarizing the core concept]

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- [Additional guidance]
```

## Writing hints.md

Progressive hints help learners without giving away the answer.

### Template

```markdown
# Hints for [Exercise Title]

## Hint 1: [Topic]
[General direction without solution]

## Hint 2: [Topic]
[More specific guidance]

## Hint 3: [Topic]
[Even more specific]

## Hint 4: [Topic]
[Nearly complete example]

## Hint 5: Complete Example
[Full working example with explanation]
```

## Writing starter/prompt.txt

The starter prompt gives learners a starting point.

### Guidelines

- Be clear about requirements
- Specify the exact format expected
- Don't reveal the solution
- End with "Output only the JSON, no explanation" if applicable

### Example

```text
Create a JSON object representing a person with the following fields:
- name: A non-empty string
- age: An integer between 0 and 150
- email: A valid email address

Requirements:
1. All fields are required
2. No additional properties allowed
3. Follow the exact format shown in the README

Output only the JSON, no explanation.
```

## Testing Your Exercise

### 1. Test Schema Validation

```bash
# Test a valid output
echo '{"valid": "json"}' | cargo run -- grade <track>/<id>

# Test an invalid output
echo '{"invalid": 123}' | cargo run -- grade <track>/<id>
```

### 2. Run Integration Tests

Add tests in `tests/grading_integration.rs`:

```rust
#[test]
fn test_my_exercise_valid() {
    let exercise = load_exercise("track", "exercise_id");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{"valid": "json"}"#;
    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(result.passed, "Expected valid output to pass: {}", result.message);
}

#[test]
fn test_my_exercise_invalid() {
    let exercise = load_exercise("track", "exercise_id");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{"missing": "fields"}"#;
    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected invalid output to fail");
}
```

### 3. Run All Tests

```bash
cargo test
```

## Best Practices

### Do

- **Prefer deterministic grading** - Schemas and invariants over LLM-as-judge
- **Test edge cases** - Invalid inputs, missing fields, wrong types
- **Write clear error messages** - Help learners understand what went wrong
- **Use realistic examples** - Connect to real-world scenarios
- **Keep exercises focused** - One concept per exercise

### Don't

- **Require expensive models** - Provide alternatives
- **Require network access** - Unless explicitly needed
- **Use LLM-as-judge** - Unless no deterministic alternative
- **Include secrets** - Use fixtures for mock data
- **Make exercises too large** - Break into smaller pieces

## Checklist

Before submitting a new exercise:

- [ ] Exercise ID is unique and follows `snake_case`
- [ ] Manifest is valid TOML with all required fields
- [ ] README explains goal, task, and grading clearly
- [ ] Grader validates expected output correctly
- [ ] At least 5 hints provided in hints.md
- [ ] Starter prompt is clear and helpful
- [ ] Integration tests cover valid and invalid cases
- [ ] Exercise works offline (unless network required)
- [ ] No secrets or API keys in any files
- [ ] All tests pass (`cargo test`)

## Examples

Look at existing exercises for reference:

- `exercises/fundamentals/json_01/` - Basic schema validation
- `exercises/fundamentals/tools_01/` - Tool calling (sandbox grading)
- `exercises/mcp/server_01/` - MCP tool definition

---

*See CLAUDE.md for overall project architecture and AGENTS.md for agent guidance.*

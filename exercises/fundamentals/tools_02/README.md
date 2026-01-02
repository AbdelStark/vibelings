# Multi-Tool Orchestration

## Goal

Demonstrate correct orchestration of multiple tools to complete a complex task,
including proper sequencing and data dependencies.

## Why This Matters

Real-world agentic tasks rarely require just one tool call. Consider booking travel:
you might need to search flights, check availability, get user preferences, validate
payment, and finally book. Each step depends on previous results.

This exercise teaches:

1. **Tool sequencing** - calling tools in the right order
2. **Data flow** - using outputs from one tool as inputs to another
3. **Task decomposition** - breaking complex goals into tool-sized steps
4. **Minimal tool usage** - don't call tools unnecessarily

## The Task

You are building a code review assistant. Given a task to review and fix a bug,
output the correct sequence of tool calls.

**Task**: "There's a bug in the calculate_total function in orders.py.
Find it, understand what it's supposed to do, and fix it."

## Available Tools

### `read_file`
Read contents of a file.

**Parameters**:
- `path` (string, required): Path to the file

### `search_code`
Search for patterns in the codebase.

**Parameters**:
- `pattern` (string, required): Search pattern (regex supported)
- `file_type` (string, optional): Filter by file extension (e.g., "py", "js")

### `write_file`
Write contents to a file.

**Parameters**:
- `path` (string, required): Path to the file
- `content` (string, required): New file contents

### `run_tests`
Run the test suite.

**Parameters**:
- `path` (string, optional): Run tests for specific file/directory
- `verbose` (boolean, optional): Show detailed output

## Expected Output Format

```json
{
  "tool_calls": [
    {"name": "tool_name", "arguments": {...}},
    {"name": "another_tool", "arguments": {...}}
  ]
}
```

## Grading

Your output is validated against:

1. **Tool schema compliance** - All tool calls must have valid arguments
2. **Sequence correctness** - Must read before writing, must test after changes
3. **Completeness** - Must address the full task (read, fix, verify)

## Key Constraints

The grader checks these invariants:

- `read_file` must be called BEFORE `write_file` for the same path
- `run_tests` should be called AFTER any `write_file` calls
- You must read `orders.py` before modifying it

## Example Workflow

For a typical bug fix, the sequence might be:

1. Read the file to understand current code
2. (Optionally) Search for related code or tests
3. Write the fixed version
4. Run tests to verify the fix

## Key Lesson

**Sequence matters in agentic workflows.** You can't fix what you haven't read.
You can't verify without testing. Real-world agents fail when they:

- Try to modify files they haven't read
- Skip verification steps
- Call tools in illogical order
- Make changes without understanding context

Reliable agents follow disciplined workflows with proper sequencing.

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about what information you need before each step
- Remember: read first, then modify, then verify

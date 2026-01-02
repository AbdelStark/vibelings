# Multi-Tool Orchestration

## Goal

Demonstrate correct orchestration of multiple tools to complete a complex task,
including proper sequencing and data dependencies.

## Why This Matters

Real-world agentic tasks rarely require just one tool call. Consider booking travel:
you might need to search flights, check availability, get user preferences, validate
payment, and finally book. Each step depends on previous results.

Or consider the task in this exercise: fixing a bug. You must read the code before you can understand it, understand it before you can fix it, fix it before you can test it. Skip any step, and you fail.

This is where "agentic" systems become actual engineering. It's not about clever prompts; it's about disciplined workflows.

## The Concept: Tool Dependency Graphs

Every multi-tool task has an implicit **dependency graph**:

```
read_file("orders.py")
        │
        ▼
analyze & plan fix (implicit reasoning step)
        │
        ▼
write_file("orders.py", fixed_content)
        │
        ▼
run_tests()
```

Some dependencies are hard constraints:
- You **cannot** write a file you haven't read (you'd overwrite with garbage)
- You **should not** consider the task complete without verification

Others are soft constraints:
- You **might** want to search for related code first
- You **could** run tests before and after to compare

Reliable agents encode these constraints explicitly. They don't rely on the LLM "figuring out" the right order.

**The analogy**: This is like a build system (Make, Bazel). You declare dependencies, and the system ensures correct ordering. Agentic workflows need the same discipline.

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

## Common Mistakes

**1. Writing before reading**
```json
[
  {"name": "write_file", "arguments": {"path": "orders.py", "content": "..."}},
  {"name": "read_file", "arguments": {"path": "orders.py"}}
]
```
This is backwards. You can't fix code you haven't seen.

**2. Skipping verification**
```json
[
  {"name": "read_file", "arguments": {"path": "orders.py"}},
  {"name": "write_file", "arguments": {"path": "orders.py", "content": "..."}}
]
```
How do you know the fix works? Always verify with tests.

**3. Unnecessary tool calls**
```json
[
  {"name": "search_code", "arguments": {"pattern": "calculate_total"}},
  {"name": "search_code", "arguments": {"pattern": "orders"}},
  {"name": "search_code", "arguments": {"pattern": "total"}},
  {"name": "read_file", "arguments": {"path": "orders.py"}},
  ...
]
```
You already know the file is `orders.py`. Don't search for what you already know.

**4. Missing required parameters**
```json
{"name": "write_file", "arguments": {"path": "orders.py"}}  // Missing "content"
```

## Grading

Your output is validated against:

1. **Tool schema compliance** — All tool calls must have valid arguments
2. **Sequence correctness** — Must read before writing, must test after changes
3. **Completeness** — Must address the full task (read, fix, verify)

## Key Constraints

The grader checks these **invariants** (conditions that must always be true):

- `read_file` must be called BEFORE `write_file` for the same path
- `run_tests` should be called AFTER any `write_file` calls
- You must read `orders.py` before modifying it

These invariants are checked programmatically. They're not "suggestions" — they're enforced.

## Example Workflow

For a typical bug fix, the sequence might be:

1. Read the file to understand current code
2. (Optionally) Search for related code or tests
3. Write the fixed version
4. Run tests to verify the fix

## Key Lesson

**Sequence matters in agentic workflows.** You can't fix what you haven't read.
You can't verify without testing. Real-world agents fail when they:

- Try to modify files they haven't read (blind changes)
- Skip verification steps (undetected regressions)
- Call tools in illogical order (wasted computation)
- Make changes without understanding context (wrong fixes)

The pattern to internalize: **Read → Understand → Change → Verify**

This applies far beyond code. Updating a database? Read current state, plan change, apply change, verify result. Calling external APIs? Check preconditions, make the call, validate response.

**The meta-lesson**: Good agentic systems make these workflows explicit. They don't rely on the LLM "knowing" the right order. They encode constraints that are checked automatically.

## Connections

- **Prerequisite**: [tools_01](../tools_01/) introduces single tool calls
- **Related**: [error_01](../error_01/) handles what happens when tools fail
- **Advanced**: [workflows/workflow_tool_wiring_01](../../workflows/workflow_tool_wiring_01/) formalizes these patterns
- **Production**: [production_eval_01](../../production/production_eval_01/) tests workflows systematically

## Further Reading

- [Anthropic: Agentic loops](https://docs.anthropic.com/en/docs/build-with-claude/agentic-loops) — Managing multi-step tool use
- [Building effective agents](https://www.anthropic.com/research/building-effective-agents) — Workflow design principles
- [Directed acyclic graphs (DAGs)](https://en.wikipedia.org/wiki/Directed_acyclic_graph) — The formal structure behind workflows

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about what information you need before each step
- Remember: read first, then modify, then verify
- You know the file name already — don't search for it

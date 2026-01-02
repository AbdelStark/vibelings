# Tool Design for Token Efficiency

## Goal

Design a set of tools for an agent that are token-efficient, self-contained, and have minimal functional overlap.

## Why This Matters

Tools consume context in two ways:
1. **Definitions**: Every tool's schema, description, and parameters use tokens
2. **Results**: Tool outputs are added to context

Poorly designed tool sets cause problems:
- **Overlap confusion**: Agent can't decide which tool to use
- **Verbose outputs**: Large responses that bloat context
- **Chained dependencies**: One tool requires calling another first

Good tool design principles:
- Each tool does one thing well
- Descriptions are precise but concise
- Parameters are minimal and well-typed
- Outputs are structured and predictable

## Requirements

Design a tool set for a file management agent. Create exactly 5 tools that cover these operations:
- Reading file contents
- Writing/creating files
- Listing directory contents
- Searching for files by pattern
- Getting file metadata (size, modified date, etc.)

For each tool, specify:
1. **Name**: Short, verb-first (e.g., `read_file`, not `file_reader`)
2. **Description**: 1-2 sentences, precise
3. **Parameters**: Minimal required params with types
4. **Output schema**: What the tool returns
5. **Token estimate**: Approximate tokens for definition

Also identify and document any potential overlaps and how you've minimized them.

## Grading

Your output will be validated against a JSON Schema. The exercise passes if:

1. Exactly 5 tools are defined
2. Each tool has name, description, parameters, and output_schema
3. No two tools have the same name
4. Parameter counts are reasonable (1-4 per tool)
5. Token estimates are provided

## Key Lesson

**Tools are part of context engineering**: A tool set with 10 overlapping tools wastes tokens and confuses the agent. Design tools as carefully as you design prompts. The goal is the minimum set of non-overlapping tools that cover your use cases.

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about what makes each operation distinct
- Avoid tools that could be composed from other tools

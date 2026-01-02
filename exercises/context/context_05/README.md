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

## The Concept: Tool Economy

Every tool in your set has costs:

```
Tool Definition Cost:
┌────────────────────────────────────┐
│ {                                  │
│   "name": "read_file",        ~5   │
│   "description": "...",      ~20   │
│   "parameters": {...}        ~30   │
│ }                                  │
│ Total: ~55 tokens per tool         │
└────────────────────────────────────┘

10 tools × 55 tokens = 550 tokens just for definitions
```

And tools compete for the agent's attention. With 20 similar-looking tools, the agent might:
- Pick the wrong one
- Call multiple when one would suffice
- Waste tokens on tool selection reasoning

**The goal**: Minimum viable tool set. Cover your use cases with the fewest, most distinct tools possible.

### Design Principles

| Principle | Good | Bad |
|-----------|------|-----|
| **Verb-first naming** | `read_file` | `file_reader` |
| **Concise descriptions** | "Read file contents" | "This tool can be used to read the contents of a file from the filesystem..." |
| **Minimal parameters** | `path: string` | `path: string, encoding: string, lines: int, offset: int, format: string` |
| **No overlap** | `list_dir` + `search_files` | `list_files` + `find_files` + `glob_files` |
| **Structured output** | `{"content": "..."}` | Free-form text |

### Overlap Analysis

Before finalizing tools, ask:
- Could tool A's job be done by tool B with different parameters?
- Are these tools ever both called for the same purpose?
- Would combining two tools reduce confusion?

```
Overlapping:                    Distinct:
read_file_text                  read_file
read_file_binary                (with format param)
read_file_lines

list_directory                  list_dir (for directories)
list_files                      search_files (for patterns)
glob_pattern
```

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

## Common Mistakes

**1. Too many parameters**
```json
{"parameters": ["path", "encoding", "start_line", "end_line", "format", "cache"]}  // Overkill
{"parameters": ["path"]}  // Minimal, with sensible defaults
```

**2. Verbose descriptions**
```json
{"description": "This tool reads the contents of a file from the local filesystem and returns them as a string"}  // Wordy
{"description": "Read file contents as text"}  // Concise
```

**3. Overlapping tools**
Having both `list_directory` and `list_files` confuses the agent. One tool with clear semantics is better.

**4. Unbounded outputs**
```json
{"output": "entire_file_contents"}  // Could be megabytes
{"output": {"content": "...", "truncated": true, "total_bytes": 1000000}}  // Bounded
```

## Grading

Your output will be validated against a JSON Schema. The exercise passes if:

1. Exactly 5 tools are defined
2. Each tool has name, description, parameters, and output_schema
3. No two tools have the same name
4. Parameter counts are reasonable (1-4 per tool)
5. Token estimates are provided

## Key Lesson

**Tools are part of context engineering**: A tool set with 10 overlapping tools wastes tokens and confuses the agent. Design tools as carefully as you design prompts. The goal is the minimum set of non-overlapping tools that cover your use cases.

Apply the Unix philosophy: each tool does one thing well. Composition happens at the workflow level, not within individual tools.

## Connections

- **Prerequisite**: [context_04](../context_04/) covers context compaction
- **Related**: [fundamentals/tools_01](../../fundamentals/tools_01/) introduces tool calling
- **Advanced**: [mcp/server_01](../../mcp/server_01/) defines tools in MCP format

## Further Reading

- [Anthropic: Tool design](https://docs.anthropic.com/en/docs/build-with-claude/tool-use/best-practices) — Official best practices
- [Unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy) — "Do one thing well"
- [API design principles](https://swagger.io/resources/articles/best-practices-in-api-design/) — Applies to tools

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about what makes each operation distinct
- Avoid tools that could be composed from other tools
- Count tokens: name (~2) + description (~20) + params (~30) ≈ 50-60 per tool

# CLAUDE.md

> Guidance for AI agents working on the vibelings codebase.

## Project Overview

**vibelings** is "Rustlings for agentic programming" — a terminal-first, open-source, exercise-driven curriculum for learning to build reliable agentic AI systems. The project teaches engineering disciplines that make agentic systems reliable, not "prompt hacks."

### Core Philosophy

1. **Contracts over vibes**: Schemas, tool interfaces, explicit success criteria
2. **Observability first**: Traces, logs, cost/latency visibility
3. **Deterministic scaffolding around non-deterministic cores**: Simulation environments, constrained tools, replayable traces
4. **Security posture by default**: Least-privilege tools, sandboxing, explicit user consent boundaries

### What This Project Is NOT

- Not a prompt engineering tutorial
- Not a framework-specific SDK wrapper
- Not about "getting the LLM to say the right thing"

### What This Project IS

- A practical training ground for the AAIF-era stack (MCP + AGENTS.md + modern agent SDKs)
- Exercises that teach: design contracts + tools + guardrails + evals → measure reliability
- A path to competence in agentic engineering with verifiable progress

---

## Repository Structure

```
vibelings/
├── CLAUDE.md              # This file - AI agent guidance
├── AGENTS.md              # Repository-specific agent guidance (AAIF standard)
├── Cargo.toml             # Rust workspace manifest
├── src/
│   ├── main.rs            # CLI entrypoint
│   ├── lib.rs             # Library root
│   ├── cli/               # CLI commands (init, watch, run, list, hint, verify, replay)
│   ├── runner/            # Exercise runner and orchestration
│   ├── grader/            # Grading engine (schema validation, invariants, multi-run)
│   ├── provider/          # Model provider abstraction
│   ├── sandbox/           # Tool sandbox and security
│   ├── trace/             # Trace capture and replay
│   └── config/            # Configuration loading
├── exercises/             # Exercise content
│   └── <track>/
│       └── <exercise_id>/
│           ├── README.md      # Exercise description
│           ├── manifest.toml  # Exercise configuration
│           ├── starter/       # Starter files for learner
│           ├── grader/        # Grading scripts/schemas
│           └── fixtures/      # Deterministic tool fixtures
└── tests/                 # Integration tests
```

---

## Architecture Principles

### Provider Abstraction

All model interactions go through a unified provider trait:

```rust
pub trait ModelProvider {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;
}
```

The `CompletionRequest` uses an OpenAI-compatible schema internally. Supported backends:
- **OpenRouter** (default): Unified API with BYOK support, privacy controls, ZDR
- **Direct providers**: OpenAI, Anthropic, etc. via their native APIs
- **Local endpoints**: Any OpenAI-compatible server (Ollama, vLLM, etc.)

### Grading Philosophy

Exercises are graded using deterministic methods wherever possible:

| Pattern | Use Case | Grading Method |
|---------|----------|----------------|
| **Contract-first** | Structured output | JSON Schema validation + semantic invariants |
| **Tool sandbox** | Tool calling | Final sandbox state + call sequence constraints |
| **Multi-run reliability** | Stochastic tasks | K runs, pass if ≥N succeed (e.g., 4/5) |
| **LLM-as-judge** | Last resort only | Rubric + structured judge output + evidence |

**Default to deterministic grading. LLM-as-judge is a last resort.**

### Exercise Format

Each exercise is a directory with a `manifest.toml`:

```toml
[exercise]
id = "contracts_json_01"
title = "JSON Output Contracts"
track = "fundamentals"
prerequisites = []

[requirements]
tool_calling = false
json_mode = true
min_context_window = 4096

[run]
max_tool_calls = 0
timeout_seconds = 30
runs = 1  # For reliability exercises, set higher

[grader]
type = "schema"
schema_path = "grader/schema.json"
invariants = ["grader/invariants.sh"]
```

### Security Model

**Default posture is locked-down:**
- No network access unless exercise explicitly requires it
- Allowlisted commands only in tool sandbox
- Tool processes timeboxed (default 30s)
- Filesystem confined to exercise workspace
- Trace all tool calls for auditability

Advanced users can enable "dangerous mode" with explicit opt-in.

---

## Code Standards

### Rust Conventions

- **Edition**: Rust 2021
- **Error handling**: Use `thiserror` for library errors, `anyhow` in CLI
- **Async runtime**: `tokio`
- **CLI framework**: `clap` with derive macros
- **Serialization**: `serde` with `serde_json` and `toml`
- **HTTP client**: `reqwest` with rustls

### Naming Conventions

- Types: `PascalCase`
- Functions/methods: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`
- Exercise IDs: `snake_case` (e.g., `json_01`, `mcp_server_basic`)

### Documentation

- All public items must have doc comments
- Use `///` for item docs, `//!` for module docs
- Include examples in doc comments for complex APIs
- Keep comments focused on "why", not "what"

### Testing

- Unit tests in the same file as implementation (`#[cfg(test)]`)
- Integration tests in `tests/`
- Use `cargo nextest` for running tests
- Exercise grading must be deterministic and reproducible

---

## CLI Commands

The CLI mirrors Rustlings' interaction model:

| Command | Description |
|---------|-------------|
| `vibelings init` | Create workspace + config + first track |
| `vibelings` | Watch mode (default) - reruns on file changes |
| `vibelings run <exercise>` | Run single exercise once |
| `vibelings list` | Interactive exercise list with status |
| `vibelings hint` | Layered hints (static first, AI hint optional) |
| `vibelings verify` | Run full test suite for completed exercises |
| `vibelings replay <run_id>` | Replay trace for debugging |
| `vibelings doctor` | Verify keys, model access, tool support |
| `vibelings cost` | Show token costs per exercise |
| `vibelings reset <exercise>` | Reset exercise to starter state |

### Watch Mode Output

```
━━━ Exercise: contracts/json_01 ━━━
Goal: Generate valid JSON matching the Person schema

Last run: ✅ PASSED (0.8s, $0.002)
Tool calls: 0 | Tokens: 342 in / 89 out

Schema validation: ✓
Invariants: ✓ (2/2)

Press [h]int, [n]ext, [l]ist, [q]uit
```

---

## Exercise Tracks

### Track 1: Agentic Fundamentals
Core primitives without frameworks:
- Structured output contracts (JSON schemas)
- Tool calling: schemas, validation, retries, timeouts
- Error recovery: tool failures, partial results
- Observability: logs, traces, cost accounting
- Guardrails: input/output validation

### Track 2: MCP in Practice
Model Context Protocol implementation:
- Build minimal MCP server (one tool, one resource)
- MCP client adapter
- Auth + least-privilege design
- Progress tracking, cancellation, logging utilities

### Track 3: Workflow Orchestration
Integration with workflow tools (n8n):
- Import/export workflow JSON
- Tool wiring patterns
- Human fallback patterns
- Structured output enforcement

### Track 4: Production Engineering
Reliability at scale:
- Evaluation harnesses and regression tests
- Anti-prompt-injection patterns
- Least privilege tools + sandboxing
- Cost/latency budgets
- Model/provider drift management

---

## Progress Tracking

Exercise status uses honest indicators:

| Status | Meaning |
|--------|---------|
| ✅ Completed | Passed deterministic checks |
| 🟡 Flaky | Passed but under reliability threshold |
| 🔁 Needs reruns | Multi-run exercise, insufficient data |
| ⏳ Pending | Not yet attempted |
| 🧪 Experimental | Exercise depends on rapidly changing ecosystem |

Progress is tracked in `~/.config/vibelings/progress.toml`.

---

## Configuration

User configuration lives in `~/.config/vibelings/config.toml`:

```toml
[model]
provider = "openrouter"  # or "openai", "anthropic", "local"
model = "anthropic/claude-sonnet-4-20250514"
temperature = 0

[openrouter]
api_key_env = "OPENROUTER_API_KEY"
zdr = true  # Zero Data Retention
data_collection = "deny"
allow_fallbacks = true
provider_order = ["anthropic", "openai"]

[sandbox]
network = false  # Default: no network
timeout_seconds = 30
allowed_commands = ["cat", "ls", "grep", "jq"]

[display]
show_cost = true
show_trace = true
color = "auto"
```

---

## Development Workflow

### Building

```bash
cargo build
cargo build --release
```

### Testing

```bash
cargo test
cargo nextest run  # Preferred
```

### Running Locally

```bash
cargo run -- init
cargo run -- list
cargo run -- run contracts/json_01
```

### Adding a New Exercise

1. Create directory: `exercises/<track>/<exercise_id>/`
2. Write `manifest.toml` with exercise metadata
3. Create `README.md` with clear goal and constraints
4. Add `starter/` files (what learner edits)
5. Create `grader/` with schema and/or invariant scripts
6. Add `fixtures/` for deterministic tool responses if needed
7. Test with `cargo run -- run <track>/<exercise_id>`

---

## Key Design Decisions

### Why Deterministic Grading?

LLM outputs are stochastic. If grading depends on string matching or subjective evaluation, exercises become frustrating and unreproducible. By grading artifacts (JSON schemas, tool traces, sandbox states), we get:
- Reproducible results
- Clear failure messages
- No "it worked on my machine" problems
- Teaching the right mental model: engineer around non-determinism

### Why OpenRouter as Default?

- Single integration point for multiple providers
- BYOK support (users keep their existing accounts)
- Privacy controls (ZDR, data collection policies)
- Provider routing flexibility
- Fallback handling built-in

Users who prefer direct provider access can configure it.

### Why Rust?

- Single binary distribution (like Rustlings)
- Fast startup time for watch mode
- Strong type system for exercise/grader contracts
- Memory safety for sandbox security
- Excellent CLI tooling ecosystem (clap, tokio, reqwest)

---

## Contributing Guidelines

### Pull Request Process

1. Fork and create feature branch
2. Write tests for new functionality
3. Ensure `cargo test` and `cargo clippy` pass
4. Update documentation if adding features
5. Keep commits atomic and well-described

### Exercise Contributions

Community exercises are welcome. Requirements:
- Must be gradable deterministically (prefer schema/invariant checks)
- Clear README with learning objectives
- Tested on at least two model providers
- No exercises that require expensive models without alternatives

### Code Review Focus

- Is grading deterministic?
- Are security boundaries maintained?
- Is the exercise teaching engineering, not prompt tricks?
- Does it work offline where possible?

---

## Troubleshooting

### Common Issues

**"Model does not support tool calling"**
- Check `vibelings doctor` for model capabilities
- Use a model that supports the required features

**"Rate limited"**
- Configure `provider_order` to enable fallbacks
- Add delay between exercises in watch mode

**"Sandbox timeout"**
- Increase `timeout_seconds` in config
- Check if exercise has infinite loop in fixture

**"Schema validation failed"**
- Run with `--verbose` to see actual output vs expected schema
- Check for trailing whitespace or encoding issues

---

## Security Considerations

### For Exercise Authors

- Never require real credentials in exercises
- Use fixture-based tools, not real APIs
- Validate all tool inputs in grader
- Document any network requirements explicitly

### For Users

- Review exercise manifests before running
- Use `--dry-run` to see what would execute
- Keep sandbox enabled (default)
- Review traces for unexpected behavior

### For Contributors

- No secrets in code or fixtures
- Audit all tool implementations
- Maintain least-privilege defaults
- Document any security exceptions

---

## Glossary

| Term | Definition |
|------|------------|
| **MCP** | Model Context Protocol - standardized tool/data connection for LLMs |
| **AAIF** | Agentic AI Foundation (Linux Foundation) |
| **BYOK** | Bring Your Own Key - use existing provider credentials |
| **ZDR** | Zero Data Retention - provider doesn't store prompts/completions |
| **Invariant** | A condition that must always be true for grading to pass |
| **Fixture** | Deterministic mock data for tool responses |
| **Trace** | Recorded sequence of model requests and tool calls |

---

## References

- [Rustlings](https://github.com/rust-lang/rustlings) - Inspiration for CLI UX
- [MCP Specification](https://spec.modelcontextprotocol.io/) - Tool protocol standard
- [AGENTS.md](https://github.com/agentsmd/agents.md) - Repository agent guidance standard
- [OpenRouter API](https://openrouter.ai/docs) - Multi-provider API

---

*This file follows the AGENTS.md standard and is designed to be read by both humans and AI agents working on this codebase.*

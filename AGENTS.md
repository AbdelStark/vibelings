# AGENTS.md

> Operational guidance for AI agents working on the vibelings codebase.
> This file follows the [AGENTS.md standard](https://github.com/agentsmd/agents.md).

## Quick Reference

| Task | Command |
|------|---------|
| Build | `cargo build` |
| Test | `cargo test` |
| Lint | `cargo clippy -- -D warnings` |
| Format | `cargo fmt` |
| Run | `cargo run -- <command>` |

## Repository Purpose

vibelings is "Rustlings for agentic programming" — a CLI tool that teaches engineers how to build reliable AI agents through hands-on exercises with deterministic grading.

**Core principle:** Prefer deterministic grading (schemas, invariants, sandbox state) over LLM-as-judge.

## Before Making Changes

1. **Read CLAUDE.md** for architecture and design decisions
2. **Run tests** to ensure baseline is passing: `cargo test`
3. **Check ROADMAP.md** for current project phase and priorities

## Code Conventions

### Rust Style

- **Edition:** Rust 2021
- **Errors:** `thiserror` for library errors, `anyhow` in CLI
- **Async:** `tokio` runtime
- **Naming:** Types `PascalCase`, functions `snake_case`, constants `SCREAMING_SNAKE_CASE`

### File Organization

```
src/
├── cli/commands/     # CLI subcommands (one file per command)
├── config/           # Configuration types and loading
├── grader/           # Grading engine (schema, invariants, sandbox)
├── provider/         # Model provider abstraction
├── runner/           # Exercise execution orchestration
├── sandbox/          # Tool sandbox and security
└── trace/            # Trace capture and replay
```

## Common Tasks

### Adding a CLI Command

1. Create `src/cli/commands/<command>.rs`
2. Add module to `src/cli/commands/mod.rs`
3. Add variant to `Commands` enum in `src/cli/mod.rs`
4. Implement the command handler
5. Add integration test in `tests/cli_integration.rs`

### Adding an Exercise

1. Create directory: `exercises/<track>/<exercise_id>/`
2. Create `manifest.toml` with exercise metadata:
   ```toml
   [exercise]
   id = "exercise_id"
   title = "Exercise Title"
   track = "fundamentals"  # or "mcp", "workflows", "production"

   [grader]
   type = "schema"  # or "invariants", "combined", "sandbox"
   schema_path = "schema.json"
   ```
3. Create `README.md` with learning objectives
4. Add `grader/` directory with schema or invariant scripts
5. Add `starter/` directory with initial files for learner

### Adding a Grader Type

Graders live in `src/grader/`. Each grader must:
- Be deterministic (reproducible results)
- Return `GradingResult` with clear pass/fail and details
- Avoid LLM-as-judge unless absolutely necessary

### Adding a Provider

1. Create `src/provider/<name>.rs`
2. Implement `ModelProvider` trait from `src/provider/traits.rs`
3. Add variant to provider enum in config
4. Add integration test

## Testing Requirements

- All PRs must pass `cargo test`
- All PRs must pass `cargo clippy -- -D warnings`
- New CLI commands need integration tests
- New graders need unit tests with edge cases

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture
```

## Security Guidelines

- **Sandbox is default:** Tools run with restricted permissions
- **No secrets in code:** Use environment variables
- **Validate inputs:** Especially in grader scripts
- **Document network requirements:** If an exercise needs network, say so in manifest

## Architecture Decisions

| Decision | Rationale |
|----------|-----------|
| Multi-run at runner level | Graders are single-run; runner handles K-run orchestration |
| OpenRouter as default | Multi-provider, BYOK, privacy controls, fallback handling |
| JSON Schema for validation | Deterministic, clear error messages, standard format |
| Invariant scripts | Flexible custom validation without code changes |

## What NOT to Do

- Don't add LLM-as-judge grading without discussing alternatives first
- Don't add network-required exercises without explicit opt-in
- Don't store API keys or secrets anywhere in the codebase
- Don't create exercises that only work with expensive models
- Don't break the existing test suite

## Getting Help

- **Architecture questions:** Read CLAUDE.md first
- **Current priorities:** Check ROADMAP.md
- **Exercise format:** Look at existing exercises in `exercises/fundamentals/`

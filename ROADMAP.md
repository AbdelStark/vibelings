# Vibelings Roadmap

> Tracking progress for building "Rustlings for agentic programming"

## Current Status: Phase 6 - Polish & Testing (In Progress)

**Last Updated:** 2026-01-02

---

## Milestones Overview

| Phase | Description | Status |
|-------|-------------|--------|
| Phase 1 | Foundation - Core infrastructure | ✅ Complete |
| Phase 2 | CLI Implementation | ✅ Complete |
| Phase 3 | First Exercise Track | ✅ Complete (7 exercises) |
| Phase 4 | Provider Integration | ✅ Complete (OpenRouter) |
| Phase 5 | Sandbox & Security | ✅ Complete |
| Phase 6 | Polish & Testing | 🚧 In Progress |

---

## Phase 1: Foundation (Core Infrastructure) ✅

### Milestone 1.1: Project Setup ✅
- [x] Create CLAUDE.md with architecture documentation
- [x] Set up git repository
- [x] Create Cargo.toml with dependencies
- [x] Create basic src/ structure (lib.rs, main.rs)
- [x] Verify project compiles

### Milestone 1.2: Configuration System ✅
- [x] Define config types (UserConfig, ExerciseManifest)
- [x] Implement config loading from TOML
- [x] Create default configuration
- [x] Add config validation

### Milestone 1.3: Error Handling Foundation ✅
- [x] Define error types with thiserror
- [x] Create Result type aliases
- [x] Implement error display

---

## Phase 2: CLI Implementation ✅

### Milestone 2.1: CLI Framework ✅
- [x] Set up clap with derive macros
- [x] Define all subcommands (init, run, list, hint, verify, replay, doctor, cost, reset)
- [x] Implement command routing

### Milestone 2.2: Basic Commands ✅
- [x] `vibelings init` - Create workspace
- [x] `vibelings list` - List exercises
- [x] `vibelings doctor` - Check environment
- [x] `vibelings reset` - Reset exercise

### Milestone 2.3: Core Commands ✅
- [x] `vibelings run <exercise>` - Run single exercise
- [x] `vibelings` (default) - Watch mode
- [x] `vibelings hint` - Show hints
- [x] `vibelings verify` - Run full test suite

### Milestone 2.4: Advanced Commands ✅
- [x] `vibelings replay` - Replay traces
- [x] `vibelings cost` - Show token costs

---

## Phase 3: First Exercise Track (Agentic Fundamentals) ✅

### Milestone 3.1: Exercise Infrastructure ✅
- [x] Create exercises/ directory structure
- [x] Implement exercise discovery
- [x] Implement manifest parsing
- [x] Create starter file handling

### Milestone 3.2: Grading Engine ✅
- [x] Schema validation (JSON Schema)
- [x] Invariant checking (shell scripts)
- [x] Multi-run reliability grading (structure)
- [x] Grading result reporting

### Milestone 3.3: First Exercises ✅
- [x] `fundamentals/json_01` - Basic JSON schema compliance
- [x] `fundamentals/json_02` - Complex nested schemas
- [ ] `tools/basic_01` - Simple tool calling (TODO)
- [ ] `tools/basic_02` - Tool with validation (TODO)

---

## Phase 4: Provider Integration ✅

### Milestone 4.1: Provider Abstraction ✅
- [x] Define ModelProvider trait
- [x] Implement CompletionRequest/Response types
- [x] Add provider configuration

### Milestone 4.2: OpenRouter Integration ✅
- [x] Implement OpenRouter provider
- [x] Add BYOK support
- [x] Implement ZDR and privacy controls
- [ ] Add fallback handling (partial)

### Milestone 4.3: Direct Providers
- [ ] OpenAI provider (TODO)
- [ ] Anthropic provider (TODO)
- [ ] Local endpoint support (Ollama, vLLM) (TODO)

---

## Phase 5: Sandbox & Security ✅

### Milestone 5.1: Tool Sandbox ✅
- [x] Command allowlisting
- [x] Filesystem confinement (basic)
- [x] Network isolation (configurable)
- [x] Timeout enforcement

### Milestone 5.2: Trace System ✅
- [x] Trace capture
- [x] Trace storage
- [x] Trace replay
- [x] Cost accounting

---

## Phase 6: Polish & Testing 🚧

### Milestone 6.1: Testing 🚧
- [x] Unit tests for all modules (36 passing)
- [x] Grader error handling tests
- [x] CLI integration tests (11 passing)
- [ ] Exercise grading tests
- [ ] CI/CD setup

### Milestone 6.2: Documentation
- [x] README.md with usage instructions
- [ ] AGENTS.md for agent guidance
- [ ] Exercise authoring guide
- [ ] API documentation

### Milestone 6.3: Release Preparation
- [ ] Binary builds
- [ ] Installation instructions
- [ ] First release notes

---

## Progress Log

### 2026-01-02 (Session 3)
- Fixed incorrect stub behavior in graders (Reliability and LlmJudge now return proper errors)
- Implemented multi-run reliability in ExerciseRunner
- Added tests for grader error handling (36 unit tests total)
- Added CLI integration tests (11 tests)
- Added comprehensive README.md documentation
- Enhanced doctor command with --full API connectivity test
- Total: 47 tests passing (36 unit + 11 integration)
- Clarified that multi-run is a runner concern, not a grader type

### 2026-01-02 (Session 2)
- Completed Phase 1: Foundation
- Completed Phase 2: CLI Implementation (all 9 commands)
- Completed Phase 3: Exercise infrastructure + 2 exercises
- Completed Phase 4: OpenRouter provider integration
- Completed Phase 5: Sandbox and trace systems
- All 25 unit tests passing
- Code passes clippy and rustfmt

### 2026-01-02 (Session 1)
- Started Phase 1: Foundation
- Created ROADMAP.md for tracking progress
- Beginning Cargo.toml and project structure setup

---

## Architecture Decisions Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-01-02 | Multi-run at runner level, not grader | Graders should be single-run deterministic; runner handles K-run orchestration |
| 2026-01-02 | Use tokio for async | Standard async runtime, good ecosystem |
| 2026-01-02 | Use clap derive | Cleaner CLI code, type-safe |
| 2026-01-02 | OpenRouter as default provider | Multi-provider, BYOK, privacy controls |
| 2026-01-02 | jsonschema crate v0.29 | Reliable JSON Schema validation |
| 2026-01-02 | notify + debouncer for watch | Standard file watching pattern |

---

## Dependencies (Implemented)

Core dependencies:
- `tokio` - Async runtime ✅
- `clap` - CLI framework ✅
- `serde`, `serde_json`, `toml` - Serialization ✅
- `reqwest` + `rustls` - HTTP client ✅
- `thiserror` - Library errors ✅
- `anyhow` - CLI errors ✅
- `jsonschema` - JSON Schema validation ✅
- `notify` + `notify-debouncer-mini` - File watching ✅
- `console` + `indicatif` + `dialoguer` - Terminal UI ✅
- `walkdir` - Directory traversal ✅
- `directories` - XDG config paths ✅
- `chrono` - Date/time handling ✅
- `uuid` - Trace IDs ✅

---

## Next Steps

1. ~~Add integration tests~~ ✅ Done (11 tests)
2. ~~Complete README.md with usage docs~~ ✅ Done
3. Add AGENTS.md for agent guidance
4. Set up CI/CD pipeline with GitHub Actions
5. Add more exercises to MCP track
6. Implement direct OpenAI/Anthropic providers

---

## Notes

- Prioritize deterministic grading over LLM-as-judge
- Keep sandbox locked down by default
- Exercises should work offline where possible
- Focus on teaching engineering, not prompt hacks

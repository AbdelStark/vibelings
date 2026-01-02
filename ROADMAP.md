# Vibelings Roadmap

> Tracking progress for building "Rustlings for agentic programming"

## Current Status: Phase 8 - Production Readiness (In Progress)

**Last Updated:** 2026-01-02

---

## Milestones Overview

| Phase | Description | Status |
|-------|-------------|--------|
| Phase 1 | Foundation - Core infrastructure | ✅ Complete |
| Phase 2 | CLI Implementation | ✅ Complete |
| Phase 3 | First Exercise Track | ✅ Complete (8 exercises) |
| Phase 4 | Provider Integration | ✅ Complete (4 providers) |
| Phase 5 | Sandbox & Security | ✅ Complete |
| Phase 6 | Polish & Testing | ✅ Complete |
| Phase 7 | Content Expansion | ✅ Complete |
| Phase 8 | Production Readiness | 🔄 In Progress |

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
- [x] `fundamentals/tools_01` - Basic tool calling
- [x] `fundamentals/tools_02` - Tool sequence validation
- [x] `fundamentals/error_01` - Error handling
- [x] `fundamentals/guardrails_01` - Input/output validation
- [x] `fundamentals/observability_01` - Logging and observability
- [x] `mcp/server_01` - MCP tool definition
- [x] `mcp/client_01` - MCP JSON-RPC tool call request

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
- [x] Add retry wrapper with exponential backoff

### Milestone 4.3: Direct Providers ✅
- [x] OpenAI provider
- [x] Anthropic provider
- [x] Local endpoint support (Ollama, vLLM, LM Studio, etc.)

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

## Phase 6: Polish & Testing ✅

### Milestone 6.1: Testing ✅
- [x] Unit tests for all modules (186 passing)
- [x] Grader error handling tests
- [x] CLI integration tests (25 passing)
- [x] Exercise grading tests (112 passing)
- [x] CI/CD setup (GitHub Actions)
- [x] Total: 323 tests passing

### Milestone 6.2: Documentation ✅
- [x] README.md with usage instructions
- [x] AGENTS.md for agent guidance
- [x] Exercise authoring guide (docs/AUTHORING.md)

### Milestone 6.3: UX Improvements ✅
- [x] Watch mode keyboard interactivity (h/n/l/q)

---

## Phase 7: Content Expansion ✅

### Milestone 7.1: Workflow Orchestration Track ✅
- [x] `workflows/workflow_json_01` - n8n workflow JSON structure
- [x] `workflows/workflow_tool_wiring_01` - Tool wiring patterns
- [x] `workflows/workflow_human_loop_01` - Human-in-the-loop patterns

### Milestone 7.2: Production Engineering Track ✅
- [x] `production/production_eval_01` - Evaluation harness design
- [x] `production/production_security_01` - Prompt injection defense
- [x] `production/production_budget_01` - Cost/latency budgets

### Milestone 7.3: Release Preparation ✅
- [x] Binary builds (release workflow for Linux, macOS, Windows)
- [x] Installation instructions (install.sh, install.ps1)
- [x] First release notes (CHANGELOG.md)
- [x] API documentation (rustdoc with GitHub Pages workflow)

### Milestone 7.4: Context Engineering Track ✅
- [x] `context/context_01` - System prompt structure
- [x] `context/context_02` - Context budget management
- [x] `context/context_03` - Just-in-time retrieval
- [x] `context/context_04` - Conversation compaction
- [x] `context/context_05` - Token-efficient tools

---

## Phase 8: Production Readiness 🔄

### Milestone 8.1: Quality Audit ✅
- [x] Comprehensive codebase review
- [x] Documentation consistency check
- [x] Test coverage analysis
- [x] Additional runner/executor tests (14 new tests added)

### Milestone 8.2: Final Polish
- [ ] Tag v0.1.0 release
- [ ] Publish to crates.io
- [ ] Community outreach

---

## Progress Log

### 2026-01-02 (Session 13)
- **Production Readiness Audit (Deep Pass)**:
  - Comprehensive codebase review confirming production-ready state
  - Verified all 323 tests passing (186 unit + 25 CLI + 112 grading)
  - Verified all 22 exercises across 5 tracks are complete and documented
  - Fixed CONTRIBUTING.md: Added `context` track to manifest example
  - Confirmed documentation consistency across README, CHANGELOG, ROADMAP
  - No clippy warnings, no formatting issues
  - Codebase is clean with no TODOs/FIXMEs in source code
- Assessment: Project is production-ready for v0.1.0 tag and crates.io publish

### 2026-01-02 (Session 12)
- **Context Track Polish**:
  - Fixed Context track display name in list command (was using raw "context", now shows "Context Engineering")
  - Fixed Context track sort order in progress command (explicitly added to sort order)
  - Added `fallback_providers` documentation to config.example.toml
- **CLI Integration Tests Expansion**:
  - Added 6 new CLI tests for previously uncovered commands and flags
  - Added test_progress_command for the progress dashboard
  - Added test_list_with_search for --search flag
  - Added test_list_with_json_output for --json flag with list
  - Added test_progress_with_json_output for --json flag with progress
  - Added test_doctor_with_json_output for --json flag with doctor
  - Added test_run_with_dry_run for --dry-run flag
- Test coverage: 323 tests (186 unit + 25 CLI + 112 grading)
- All code passes clippy and rustfmt

### 2026-01-02 (Session 11)
- **Phase 8.1 Quality Audit Completion**:
  - Added 14 new runner/executor tests for comprehensive coverage
  - Added 16 new exercise module tests (Track, Status, GraderType, Exercise methods)
  - Added 23 new error module tests (all error types and conversions)
  - Updated AUTHORING.md to include Context Engineering track (5th track)
  - Fixed all test count references in documentation
  - Marked Milestone 8.1: Quality Audit as complete
- Test coverage: 317 tests (186 unit + 19 CLI + 112 grading)
- All code passes clippy, rustfmt, and test suite

### 2026-01-02 (Session 10)
- **Production Readiness Audit**:
  - Comprehensive codebase review and quality assessment
  - Fixed documentation inconsistencies (test counts, exercise counts)
  - Updated ROADMAP.md with accurate phase status and milestone tracking
  - Added Phase 8: Production Readiness with quality audit milestones
  - Added Context Engineering track to Phase 7 milestones
  - Added MSRV (Minimum Supported Rust Version) to Cargo.toml
- Exercise count: 22 exercises across 5 tracks
  - 8 fundamentals + 3 MCP + 3 workflows + 3 production + 5 context
- Total: 245 tests passing (114 unit + 19 CLI + 112 grading)
- All code passes clippy and rustfmt

### 2026-01-02 (Session 9)
- Documentation consistency updates
- Added new exercise: `mcp/resource_01` (MCP Resource Definition)
  - Teaches MCP resource schema, URI formats, MIME types
  - Complements server_01 (tools) and client_01 (JSON-RPC)
- Added Context Engineering track (Track 5) with 5 exercises:
  - `context/context_01` - System prompt structure
  - `context/context_02` - Context budget management
  - `context/context_03` - Just-in-time retrieval
  - `context/context_04` - Conversation compaction
  - `context/context_05` - Token-efficient tools
- All code passes clippy and rustfmt

### 2026-01-02 (Session 8)
- Added provider config sections to UserConfig:
  - `[openai]` with `api_key_env` and `org_id_env`
  - `[anthropic]` with `api_key_env`
  - `[local]` with `base_url` and `api_key`
- Updated providers to use config sections for env var names
- Added retry wrapper (`RetryingProvider`) with exponential backoff and jitter:
  - Handles rate limiting (429) and server errors (5xx)
  - Retries transient connection errors
  - Configurable retry count, delays, and backoff multiplier
  - Full jitter prevents thundering herd issues
- Added `create_provider_with_retry()` factory function
- Updated ExerciseRunner to use retry-enabled provider
- Added unit tests for new config types
- Added `rand` dependency for jitter generation
- Added new exercise: `fundamentals/json_03` (Array Contracts)
  - Teaches array validation with item constraints
  - minItems/maxItems constraints, nested object arrays
  - Includes 6 grading tests
- Total: 180 tests passing (98 unit + 19 CLI + 63 grading)
- Exercise count: 16 exercises across 4 tracks
- All code passes clippy and rustfmt

### 2026-01-02 (Session 7)
- Completed Phase 7.3: Release Preparation
- Added release workflow for cross-platform binary builds (Linux, macOS Intel/ARM, Windows)
- Created CHANGELOG.md with comprehensive v0.1.0 release notes
- Added documentation workflow for GitHub Pages (rustdoc)
- Created install scripts:
  - `install.sh` for Linux/macOS with binary download and cargo fallback
  - `install.ps1` for Windows PowerShell
- Enhanced CI with cross-platform testing:
  - Tests now run on Linux, macOS, and Windows
  - Smoke tests on all platforms
- Updated README with new installation options
- All 57 tests passing, code passes clippy and rustfmt

### 2026-01-02 (Session 6)
- Implemented watch mode keyboard interactivity (h/n/l/q)
  - [h] Show hints for current exercise
  - [n] Move to next exercise (marks as completed if passed)
  - [l] List all exercises with progress
  - [q] Quit watch mode
- Added UserProgress::mark_completed() method
- Created Workflow Orchestration track (Track 3) with 3 exercises:
  - `workflows/workflow_json_01` - n8n workflow JSON structure
  - `workflows/workflow_tool_wiring_01` - Tool wiring patterns
  - `workflows/workflow_human_loop_01` - Human-in-the-loop patterns
- Created Production Engineering track (Track 4) with 3 exercises:
  - `production/production_eval_01` - Evaluation harness design
  - `production/production_security_01` - Prompt injection defense
  - `production/production_budget_01` - Cost/latency budgets
- Total: 123 tests passing (55 unit + 11 CLI + 57 grading)
- All code passes clippy and rustfmt

### 2026-01-02 (Session 5)
- Implemented direct OpenAI provider with full tool calling support
- Implemented Local provider for Ollama/vLLM/LM Studio (OpenAI-compatible endpoints)
- Created MCP track with two exercises:
  - `mcp/server_01` - MCP tool definition
  - `mcp/client_01` - MCP JSON-RPC tool call request
- Added comprehensive exercise authoring guide (docs/AUTHORING.md)
- Total: 106 tests passing (54 unit + 11 CLI + 41 grading)
- All code passes clippy and rustfmt

### 2026-01-02 (Session 4)
- Created AGENTS.md following AAIF standard for agent guidance
- Added GitHub Actions CI/CD workflow (.github/workflows/ci.yml)
- Added 26 exercise grading integration tests
- Implemented native Anthropic provider with tool calling support
- Fixed code formatting and clippy warnings
- Total: 78 tests passing (41 unit + 11 CLI + 26 grading)
- All code passes clippy and rustfmt

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

1. ~~Add integration tests~~ ✅ Done (123 tests)
2. ~~Complete README.md with usage docs~~ ✅ Done
3. ~~Add AGENTS.md for agent guidance~~ ✅ Done
4. ~~Set up CI/CD pipeline with GitHub Actions~~ ✅ Done
5. ~~Implement Anthropic provider~~ ✅ Done
6. ~~Implement direct OpenAI provider~~ ✅ Done
7. ~~Add MCP track with exercises~~ ✅ Done (2 exercises)
8. ~~Implement local provider for Ollama/vLLM~~ ✅ Done
9. ~~Add exercise authoring documentation~~ ✅ Done (docs/AUTHORING.md)
10. ~~Add watch mode keyboard interactivity~~ ✅ Done (h/n/l/q)
11. ~~Add Workflow Orchestration track~~ ✅ Done (3 exercises)
12. ~~Add Production Engineering track~~ ✅ Done (3 exercises)
13. ~~Prepare first release~~ ✅ Done (release workflow, install scripts, CHANGELOG)
14. Tag v0.1.0 release

---

## Notes

- Prioritize deterministic grading over LLM-as-judge
- Keep sandbox locked down by default
- Exercises should work offline where possible
- Focus on teaching engineering, not prompt hacks

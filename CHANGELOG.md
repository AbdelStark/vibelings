# Changelog

All notable changes to vibelings will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Context Engineering track**: 5 new exercises teaching context management for AI agents
  - `context/context_01` - System prompt structure
  - `context/context_02` - Context budget management
  - `context/context_03` - Just-in-time retrieval
  - `context/context_04` - Conversation compaction
  - `context/context_05` - Token-efficient tools
- **New exercise**: `mcp/resource_01` - MCP Resource Definition (resource schema, URI formats, MIME types)
- **New exercise**: `fundamentals/json_03` - Array Contracts (array validation, item-level constraints)
- **JSON output mode**: Global `--json` flag for machine-readable output
- **Exercise search**: `vibelings list --search <query>` filters exercises by keyword
- **Progress dashboard**: `vibelings progress` shows curriculum overview with track-by-track progress
- **Provider fallback chain**: Configure `fallback_providers` for automatic failover
- **ARM64 Linux support**: Release workflow now builds `aarch64-unknown-linux-gnu` binaries
- **Example configuration**: Added `config.example.toml` with full documentation
- **Security policy**: Added `SECURITY.md` with responsible disclosure process
- **Retry logic**: `RetryingProvider` wrapper with exponential backoff and jitter
- **Dry-run mode**: `vibelings run --dry-run` previews exercise without API calls
- **Performance benchmarks**: Added criterion benchmarks (`cargo bench`)
- **Fixture system**: `FixtureStore` and `ToolFixture` for deterministic tool responses
- **MSRV**: Minimum Supported Rust Version set to 1.73

### Changed

- **Test coverage**: 231 tests total (100 unit + 19 CLI + 112 grading)
- **Exercise count**: 22 exercises across 5 tracks (8 fundamentals + 3 MCP + 3 workflows + 3 production + 5 context)
- **Documentation**: Updated ROADMAP.md with Phase 8 (Production Readiness), fixed test/exercise counts
- **Cargo.toml**: Added MSRV, homepage, documentation URL, crates.io metadata
- **Provider config**: All providers now read API key env var names from config sections
- **Exercise runner**: Uses retry-enabled provider for automatic rate limiting handling

## [0.1.0] - 2026-01-02

Initial release of vibelings - "Rustlings for agentic programming".

### Added

#### Core Features
- **Exercise-driven learning**: Progressive curriculum for building reliable AI agents
- **Deterministic grading**: Schema validation, invariant checking, multi-run reliability testing
- **Watch mode**: Auto-runs exercises on file changes with interactive keyboard controls (h/n/l/q)
- **Progress tracking**: Persistent progress saved to `~/.config/vibelings/progress.toml`
- **Trace system**: Capture and replay traces for debugging

#### CLI Commands
- `vibelings` - Start interactive watch mode (default)
- `vibelings init` - Initialize workspace with default configuration
- `vibelings run <exercise>` - Run a single exercise
- `vibelings list` - List all exercises with status indicators
- `vibelings hint [exercise]` - Get progressive hints (static first, layered)
- `vibelings verify` - Verify all completed exercises still pass
- `vibelings replay <run_id>` - Replay a trace for debugging
- `vibelings doctor` - Check environment setup and API connectivity
- `vibelings cost` - Show token costs per exercise
- `vibelings reset <exercise>` - Reset exercise to starter state

#### Model Providers
- **OpenRouter** (default): Multi-provider access with BYOK, ZDR, and privacy controls
- **OpenAI**: Direct API access with full tool calling support
- **Anthropic**: Native Claude API with tool calling support
- **Local**: OpenAI-compatible endpoints (Ollama, vLLM, LM Studio)

#### Exercise Tracks

**Track 1: Agentic Fundamentals** (7 exercises)
- `fundamentals/json_01` - Basic JSON schema compliance
- `fundamentals/json_02` - Complex nested schemas
- `fundamentals/tools_01` - Basic tool calling with schema validation
- `fundamentals/tools_02` - Tool sequence validation
- `fundamentals/error_01` - Handling tool failures
- `fundamentals/guardrails_01` - Input/output validation
- `fundamentals/observability_01` - Logging and cost awareness

**Track 2: MCP in Practice** (2 exercises)
- `mcp/server_01` - Define tools per MCP specification
- `mcp/client_01` - Construct valid JSON-RPC tool call requests

**Track 3: Workflow Orchestration** (3 exercises)
- `workflows/workflow_json_01` - n8n-style workflow JSON structure
- `workflows/workflow_tool_wiring_01` - Tool wiring and data transformation
- `workflows/workflow_human_loop_01` - Human-in-the-loop approval patterns

**Track 4: Production Engineering** (3 exercises)
- `production/production_eval_01` - Evaluation harness design
- `production/production_security_01` - Prompt injection defense patterns
- `production/production_budget_01` - Cost/latency budget enforcement

#### Security & Sandbox
- Command allowlisting with configurable permissions
- Filesystem confinement to exercise workspace
- Configurable network isolation (disabled by default)
- Timeout enforcement for tool execution
- Trace auditing for all tool calls

#### Documentation
- Comprehensive CLAUDE.md for AI agent guidance
- AGENTS.md following AAIF standard
- Exercise authoring guide (docs/AUTHORING.md)
- Full README with usage instructions

#### Testing
- 123 tests passing (55 unit + 11 CLI integration + 57 grading)
- CI/CD with GitHub Actions (test, clippy, format, smoke test)

### Technical Details

- **Rust Edition**: 2021
- **Async Runtime**: tokio
- **CLI Framework**: clap with derive macros
- **HTTP Client**: reqwest with rustls
- **JSON Schema**: jsonschema crate v0.29
- **File Watching**: notify + notify-debouncer-mini

### Philosophy

vibelings teaches engineering disciplines that make agentic systems reliable:

1. **Contracts over vibes**: Schemas, tool interfaces, explicit success criteria
2. **Observability first**: Traces, logs, cost/latency visibility
3. **Deterministic scaffolding**: Simulation environments, replayable traces
4. **Security by default**: Least-privilege tools, sandboxing, consent boundaries

This is NOT a prompt engineering tutorial. This IS a practical training ground for production-ready agentic systems.

[Unreleased]: https://github.com/AbdelStark/vibelings/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/AbdelStark/vibelings/releases/tag/v0.1.0

# Architecture Guide

> Technical deep-dive into how vibelings works

This document explains the internal architecture of vibelings for contributors and those who want to understand the system design.

---

## System Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              VIBELINGS                                  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌──────────┐ │
│  │     CLI     │───►│   Runner    │───►│   Grader    │───►│  Trace   │ │
│  │  (clap)     │    │  (tokio)    │    │  (schema)   │    │  Store   │ │
│  └─────────────┘    └──────┬──────┘    └─────────────┘    └──────────┘ │
│                            │                                            │
│                            ▼                                            │
│                     ┌─────────────┐                                     │
│                     │  Provider   │                                     │
│                     │ Abstraction │                                     │
│                     └──────┬──────┘                                     │
│                            │                                            │
│         ┌──────────────────┼──────────────────┐                         │
│         ▼                  ▼                  ▼                         │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                 │
│  │  OpenRouter │    │   OpenAI    │    │    Local    │                 │
│  │   (default) │    │  Anthropic  │    │   (Ollama)  │                 │
│  └─────────────┘    └─────────────┘    └─────────────┘                 │
│                                                                         │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                           SANDBOX                                 │  │
│  │  • Command allowlisting  • Network isolation  • Timeout enforcement│ │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Directory Structure

```
src/
├── main.rs               # Entrypoint: CLI initialization
├── lib.rs                # Library root: public exports
├── error.rs              # Error types (thiserror)
├── exercise.rs           # Exercise types (Track, Status, Exercise)
│
├── cli/                  # CLI layer
│   ├── mod.rs            # CLI enum, Args parsing
│   ├── ui.rs             # Terminal UI utilities
│   └── commands/         # One file per command
│       ├── init.rs       # vibelings init
│       ├── run.rs        # vibelings run
│       ├── watch.rs      # vibelings (default)
│       ├── list.rs       # vibelings list
│       ├── hint.rs       # vibelings hint
│       ├── verify.rs     # vibelings verify
│       ├── replay.rs     # vibelings replay
│       ├── doctor.rs     # vibelings doctor
│       ├── cost.rs       # vibelings cost
│       ├── progress.rs   # vibelings progress
│       ├── reset.rs      # vibelings reset
│       └── json_output.rs # JSON output formatting
│
├── config/               # Configuration layer
│   ├── mod.rs            # Module exports
│   ├── types.rs          # Config types (UserConfig, DisplayConfig, etc.)
│   └── loader.rs         # Config file loading and validation
│
├── runner/               # Exercise execution
│   ├── mod.rs            # Module exports
│   ├── discovery.rs      # Find and load exercises from filesystem
│   └── executor.rs       # ExerciseRunner implementation
│
├── grader/               # Grading engine
│   ├── mod.rs            # Grader trait, GradingResult, factory
│   ├── schema.rs         # SchemaGrader (JSON Schema validation)
│   └── invariant.rs      # InvariantGrader (shell script checks)
│
├── provider/             # Model provider abstraction
│   ├── mod.rs            # Module exports
│   ├── traits.rs         # ModelProvider trait definition
│   ├── request.rs        # CompletionRequest type
│   ├── response.rs       # CompletionResponse type
│   ├── openrouter.rs     # OpenRouter implementation
│   ├── openai.rs         # OpenAI implementation
│   ├── anthropic.rs      # Anthropic implementation
│   ├── local.rs          # Local/Ollama implementation
│   ├── retry.rs          # RetryingProvider wrapper
│   └── fallback.rs       # Provider fallback logic
│
├── sandbox/              # Security layer
│   ├── mod.rs            # Module exports
│   ├── executor.rs       # Sandboxed command execution
│   └── fixtures.rs       # Deterministic mock data
│
└── trace/                # Observability
    ├── mod.rs            # Trace types
    └── store.rs          # Trace storage and retrieval
```

---

## Core Components

### 1. CLI Layer (`src/cli/`)

The CLI uses [clap](https://docs.rs/clap) with derive macros for type-safe argument parsing.

```rust
#[derive(Parser)]
#[command(name = "vibelings")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Run { exercise: String },
    Watch,
    List { ... },
    Hint { ... },
    // ...
}
```

**Design principles**:
- Each command is a separate file in `commands/`
- Commands return `Result<(), anyhow::Error>`
- UI utilities are centralized in `ui.rs`

### 2. Configuration (`src/config/`)

Configuration is loaded from TOML files and deserialized with [serde](https://serde.rs/).

```rust
pub struct UserConfig {
    pub model: ModelConfig,
    pub sandbox: SandboxConfig,
    pub display: DisplayConfig,
    pub openrouter: Option<OpenRouterConfig>,
    pub openai: Option<OpenAIConfig>,
    pub anthropic: Option<AnthropicConfig>,
    pub local: Option<LocalConfig>,
}
```

**Key files**:
- `~/.config/vibelings/config.toml` — User configuration
- `~/.config/vibelings/progress.toml` — Exercise progress

**Design principles**:
- Configuration is immutable once loaded
- Environment variables for secrets (never stored in files)
- Sensible defaults for all optional fields

### 3. Exercise Runner (`src/runner/`)

The runner orchestrates exercise execution:

```
┌─────────────────────────────────────────────────────────────┐
│                      ExerciseRunner                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   1. Discovery: Find exercise on filesystem                 │
│      └── exercises/<track>/<id>/manifest.toml               │
│                                                             │
│   2. Load: Parse manifest, read starter files               │
│      └── starter/prompt.txt → user prompt                   │
│                                                             │
│   3. Execute: Call provider with prompt                     │
│      └── Provider.complete(request) → response              │
│                                                             │
│   4. Grade: Validate response against grader                │
│      └── Grader.grade(response) → GradingResult             │
│                                                             │
│   5. Trace: Store execution trace                           │
│      └── TraceStore.save(trace)                             │
│                                                             │
│   6. Multi-run (optional): Repeat for reliability           │
│      └── K runs, check N passed                             │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Key types**:
```rust
pub struct ExerciseRunner {
    config: UserConfig,
    provider: Box<dyn ModelProvider>,
    grader_factory: GraderFactory,
    trace_store: TraceStore,
}

impl ExerciseRunner {
    pub async fn run(&self, exercise: &Exercise) -> Result<RunResult>;
    pub async fn run_multi(&self, exercise: &Exercise, runs: u32) -> Result<MultiRunResult>;
}
```

### 4. Grading Engine (`src/grader/`)

Graders implement deterministic validation:

```rust
pub trait Grader {
    fn grade(&self, output: &str, exercise: &Exercise) -> Result<GradingResult>;
}

pub struct GradingResult {
    pub passed: bool,
    pub message: String,
    pub details: Vec<GradingDetail>,
}
```

**Grader types**:

| Type | Implementation | Use Case |
|------|----------------|----------|
| `SchemaGrader` | JSON Schema validation | Structured output |
| `InvariantGrader` | Shell script execution | Custom validation |
| `SandboxGrader` | Tool call validation | Tool calling exercises |
| `CombinedGrader` | Multiple graders | Complex validation |

**Design principles**:
- **Deterministic**: Same output always produces same result
- **No LLM-as-judge**: Avoids non-determinism and cost
- **Clear errors**: Grading failures include actionable messages

### 5. Provider Abstraction (`src/provider/`)

All LLM providers implement a common trait:

```rust
#[async_trait]
pub trait ModelProvider: Send + Sync {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;

    fn supports_tool_calling(&self) -> bool { false }
    fn supports_json_mode(&self) -> bool { false }
    fn name(&self) -> &str;
}
```

**Request/Response types**:
```rust
pub struct CompletionRequest {
    pub messages: Vec<Message>,
    pub model: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub tools: Option<Vec<Tool>>,
    pub json_mode: bool,
}

pub struct CompletionResponse {
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
    pub usage: Usage,
    pub model: String,
    pub finish_reason: FinishReason,
}
```

**Provider implementations**:

```
┌───────────────────────────────────────────────────────────────────────┐
│                        Provider Hierarchy                              │
├───────────────────────────────────────────────────────────────────────┤
│                                                                        │
│   ModelProvider (trait)                                                │
│        │                                                               │
│        ├── OpenRouterProvider  (default, multi-model)                  │
│        │                                                               │
│        ├── OpenAIProvider      (direct API)                            │
│        │                                                               │
│        ├── AnthropicProvider   (direct API)                            │
│        │                                                               │
│        ├── LocalProvider       (OpenAI-compatible: Ollama, vLLM)       │
│        │                                                               │
│        └── RetryingProvider    (decorator: adds retry logic)           │
│                 │                                                      │
│                 └── Wraps any provider with:                           │
│                     • Exponential backoff                              │
│                     • Jitter to prevent thundering herd                │
│                     • Retry on 429/5xx errors                          │
│                                                                        │
└───────────────────────────────────────────────────────────────────────┘
```

### 6. Sandbox (`src/sandbox/`)

The sandbox provides security isolation for tool execution:

```rust
pub struct SandboxConfig {
    pub network: bool,               // Network access (default: false)
    pub timeout_seconds: u32,        // Execution timeout
    pub allowed_commands: Vec<String>, // Command allowlist
}

pub struct SandboxExecutor {
    config: SandboxConfig,
}

impl SandboxExecutor {
    pub async fn execute(&self, command: &str) -> Result<CommandOutput>;
}
```

**Security measures**:
1. **Command allowlisting**: Only permitted commands can run
2. **Network isolation**: No network by default
3. **Timeout enforcement**: Commands killed after timeout
4. **Filesystem confinement**: Limited to exercise workspace
5. **Trace auditing**: All executions logged

### 7. Trace System (`src/trace/`)

Traces capture full execution history for debugging:

```rust
pub struct Trace {
    pub id: Uuid,
    pub exercise_id: String,
    pub timestamp: DateTime<Utc>,
    pub request: CompletionRequest,
    pub response: CompletionResponse,
    pub grading_result: GradingResult,
    pub duration_ms: u64,
    pub cost: Cost,
}

pub struct TraceStore {
    path: PathBuf,
}

impl TraceStore {
    pub fn save(&self, trace: &Trace) -> Result<()>;
    pub fn load(&self, id: &Uuid) -> Result<Trace>;
    pub fn list(&self) -> Result<Vec<TraceSummary>>;
}
```

---

## Data Flow

### Exercise Execution Flow

```
┌──────────────┐
│    User      │
│ edits prompt │
└──────┬───────┘
       │
       ▼
┌──────────────────────────────────────────────────────────────────────┐
│                            Runner                                     │
├──────────────────────────────────────────────────────────────────────┤
│                                                                       │
│   1. Load exercise                                                    │
│      manifest.toml ─► Exercise struct                                 │
│      starter/prompt.txt ─► user message                               │
│                                                                       │
│   2. Build request                                                    │
│      Exercise + Config ─► CompletionRequest                           │
│                                                                       │
│   3. Call provider                                                    │
│      CompletionRequest ─► [Provider] ─► CompletionResponse            │
│                                                                       │
│   4. Grade response                                                   │
│      CompletionResponse ─► [Grader] ─► GradingResult                  │
│                                                                       │
│   5. Store trace                                                      │
│      Request + Response + Result ─► Trace ─► [TraceStore]             │
│                                                                       │
│   6. Return result                                                    │
│      RunResult { passed, cost, message }                              │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
       │
       ▼
┌──────────────┐
│    User      │
│ sees result  │
└──────────────┘
```

### Watch Mode Flow

```
┌──────────────────────────────────────────────────────────────────────┐
│                          Watch Mode                                   │
├──────────────────────────────────────────────────────────────────────┤
│                                                                       │
│   ┌─────────────────┐                                                │
│   │  File Watcher   │ ─── notify crate (debounced)                   │
│   └────────┬────────┘                                                │
│            │                                                          │
│            ▼ file changed                                             │
│   ┌─────────────────┐                                                │
│   │  Re-run Exercise│                                                │
│   └────────┬────────┘                                                │
│            │                                                          │
│            ▼                                                          │
│   ┌─────────────────┐      ┌─────────────────┐                       │
│   │  Display Result │ ◄────│  Keyboard Input │                       │
│   └─────────────────┘      │   h/n/l/q       │                       │
│                            └─────────────────┘                       │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

---

## Design Decisions

### Why Rust?

| Decision | Rationale |
|----------|-----------|
| Single binary | Easy distribution, no runtime dependencies |
| Fast startup | Critical for watch mode responsiveness |
| Type safety | Catch errors at compile time |
| Memory safety | Safe sandbox implementation |
| Ecosystem | Excellent CLI libraries (clap, tokio, serde) |

### Why Deterministic Grading?

| Problem with LLM-as-Judge | Solution with Schemas |
|---------------------------|----------------------|
| Non-deterministic results | Same output = same grade |
| Expensive (API calls) | Free (local validation) |
| Gameable (prompt the grader) | No gaming possible |
| Unclear failures | Precise error messages |
| Hard to debug | Schema shows exact contract |

### Why Multi-Run at Runner Level?

Multi-run reliability testing could be implemented in:
1. **Grader level**: Grader runs multiple times
2. **Runner level**: Runner orchestrates multiple runs

**Choice: Runner level**

Rationale:
- Graders should be single-run, deterministic
- Runner controls orchestration (timeouts, retries)
- Cleaner separation of concerns
- Easier to add new run strategies

### Why OpenRouter as Default?

| Benefit | Explanation |
|---------|-------------|
| Multi-model | One API for many providers |
| BYOK | Users keep existing keys |
| Privacy | ZDR, data collection controls |
| Fallbacks | Built-in provider routing |
| Cost | Competitive pricing |

---

## Extension Points

### Adding a New Command

1. Create `src/cli/commands/mycommand.rs`:
   ```rust
   use anyhow::Result;

   pub async fn execute(args: MyCommandArgs) -> Result<()> {
       // Implementation
       Ok(())
   }
   ```

2. Add to `src/cli/commands/mod.rs`:
   ```rust
   pub mod mycommand;
   ```

3. Add variant to `Commands` enum in `src/cli/mod.rs`:
   ```rust
   #[derive(Subcommand)]
   pub enum Commands {
       // ...
       MyCommand(MyCommandArgs),
   }
   ```

4. Route in `main.rs`:
   ```rust
   Commands::MyCommand(args) => commands::mycommand::execute(args).await,
   ```

### Adding a New Grader

1. Create `src/grader/mygrader.rs`:
   ```rust
   pub struct MyGrader { ... }

   impl Grader for MyGrader {
       fn grade(&self, output: &str, exercise: &Exercise) -> Result<GradingResult> {
           // Validation logic
       }
   }
   ```

2. Add to factory in `src/grader/mod.rs`:
   ```rust
   pub fn create_grader(grader_type: GraderType) -> Box<dyn Grader> {
       match grader_type {
           // ...
           GraderType::MyGrader => Box::new(MyGrader::new()),
       }
   }
   ```

### Adding a New Provider

1. Create `src/provider/myprovider.rs`:
   ```rust
   pub struct MyProvider { ... }

   #[async_trait]
   impl ModelProvider for MyProvider {
       async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
           // API call
       }

       fn name(&self) -> &str { "myprovider" }
   }
   ```

2. Add to factory in `src/provider/mod.rs`

3. Add config type in `src/config/types.rs`

---

## Testing Architecture

```
tests/
├── cli_integration.rs      # CLI end-to-end tests
├── grading_integration.rs  # Exercise grading tests (112 tests)
└── common/                 # Test utilities
```

### Test Categories

| Category | Location | Purpose |
|----------|----------|---------|
| Unit tests | `#[cfg(test)]` in source | Component-level |
| CLI tests | `tests/cli_integration.rs` | Command behavior |
| Grading tests | `tests/grading_integration.rs` | Exercise validation |

### Running Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_json_01_valid

# By category
cargo test cli_           # CLI tests
cargo test grading_       # Grading tests
```

---

## Performance Considerations

### Startup Time

- Binary is ~5MB, starts in <100ms
- Config loaded once, cached
- Exercise discovery is lazy (on first access)

### Watch Mode

- Uses `notify` crate with debouncing
- File system events trigger re-run
- Debounce prevents multiple runs on rapid saves

### API Calls

- Retry wrapper adds resilience
- Exponential backoff with jitter
- Connection pooling via reqwest

---

## Security Model

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Security Layers                              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│   Layer 1: Configuration                                             │
│   ├── API keys from env vars (never stored)                         │
│   └── Sandbox disabled by default                                    │
│                                                                      │
│   Layer 2: Sandbox                                                   │
│   ├── Command allowlisting                                           │
│   ├── Network isolation                                              │
│   ├── Timeout enforcement                                            │
│   └── Filesystem confinement                                         │
│                                                                      │
│   Layer 3: Validation                                                │
│   ├── All inputs validated before use                                │
│   ├── Schema validation before execution                             │
│   └── Grader scripts reviewed                                        │
│                                                                      │
│   Layer 4: Auditing                                                  │
│   ├── All tool calls traced                                          │
│   ├── Traces stored for review                                       │
│   └── Costs tracked per exercise                                     │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Future Architecture

Planned enhancements:

### Near-term
- Parallel exercise execution
- Improved trace visualization
- Plugin system for custom graders

### Medium-term
- Distributed exercise hosting
- Team progress tracking
- Custom exercise authoring UI

### Long-term
- Multi-agent exercise support
- Interactive debugging mode
- Integration with CI/CD pipelines

---

## Further Reading

- [CLAUDE.md](../CLAUDE.md) — AI agent guidance for the codebase
- [AGENTS.md](../AGENTS.md) — Operational agent guidance
- [CONTRIBUTING.md](../CONTRIBUTING.md) — Contribution guidelines
- [AUTHORING.md](AUTHORING.md) — Exercise creation guide

---

*This document is maintained as the architecture evolves. Last updated: 2026-01-02*

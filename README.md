# vibelings

> Rustlings for agentic programming — learn to build reliable AI agents through hands-on exercises.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)

## What is vibelings?

vibelings is a terminal-first, exercise-driven curriculum for learning to build **reliable** agentic AI systems. It teaches engineering disciplines that make agentic systems work in production—not prompt hacks.

Like [Rustlings](https://github.com/rust-lang/rustlings) teaches Rust through small exercises, vibelings teaches agentic programming through progressive challenges with deterministic grading.

### Philosophy

| Principle | What it means |
|-----------|---------------|
| **Contracts over vibes** | Schemas, tool interfaces, explicit success criteria |
| **Observability first** | Traces, logs, cost/latency visibility |
| **Deterministic scaffolding** | Simulation environments, constrained tools, replayable traces |
| **Security by default** | Least-privilege tools, sandboxing, explicit consent boundaries |

### What This Is NOT

- Not a prompt engineering tutorial
- Not a framework-specific SDK wrapper
- Not about "getting the LLM to say the right thing"

### What This IS

- A practical training ground for the modern agent stack (MCP, tool calling, structured output)
- Exercises that teach: design contracts + tools + guardrails → measure reliability
- A path to competence in agentic engineering with verifiable progress

## Quick Start

### Prerequisites

- An API key from [OpenRouter](https://openrouter.ai) (or other supported provider)
- Rust 1.70+ (only needed if building from source)

### Installation

**Quick install (recommended):**

```bash
# Linux / macOS
curl -sSL https://raw.githubusercontent.com/AbdelStark/vibelings/main/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/AbdelStark/vibelings/main/install.ps1 | iex
```

**From source:**

```bash
# Clone the repository
git clone https://github.com/AbdelStark/vibelings.git
cd vibelings

# Build and install
cargo install --path .
```

**Via cargo:**

```bash
cargo install --git https://github.com/AbdelStark/vibelings
```

### Setup

```bash
# Initialize a workspace
vibelings init

# Set your API key
export OPENROUTER_API_KEY="your-key-here"

# Check your setup
vibelings doctor
```

### Start Learning

```bash
# List all exercises
vibelings list

# Run a specific exercise
vibelings run fundamentals/json_01

# Start watch mode (auto-runs on file changes)
vibelings
```

## Exercise Tracks

### Track 1: Agentic Fundamentals

Core primitives without frameworks:

| Exercise | Topic | What You Learn |
|----------|-------|----------------|
| `json_01` | JSON Output Contracts | Structured output with schema validation |
| `json_02` | Complex Nested Schemas | Deep object validation |
| `json_03` | Array Contracts | Array validation with item constraints |
| `tools_01` | Basic Tool Calling | Tool schemas and invocation |
| `tools_02` | Tool Validation | Argument validation and constraints |
| `error_01` | Handling Tool Failures | Error resilience patterns |
| `guardrails_01` | Input/Output Validation | Safety boundaries |
| `observability_01` | Tracing and Costs | Monitoring and cost awareness |

### Track 2: MCP in Practice

Model Context Protocol implementation:

| Exercise | Topic | What You Learn |
|----------|-------|----------------|
| `server_01` | MCP Tool Definition | Define tools per MCP specification |
| `client_01` | MCP JSON-RPC Request | Construct valid tool call requests |

### Track 3: Workflow Orchestration

Integration with workflow tools like n8n:

| Exercise | Topic | What You Learn |
|----------|-------|----------------|
| `workflow_json_01` | Workflow JSON Schema | n8n-style workflow structure |
| `workflow_tool_wiring_01` | Tool Wiring Patterns | Data transformation between steps |
| `workflow_human_loop_01` | Human-in-the-Loop | Approval gates and fallbacks |

### Track 4: Production Engineering

Reliability at scale:

| Exercise | Topic | What You Learn |
|----------|-------|----------------|
| `production_eval_01` | Evaluation Harness | Test agent reliability |
| `production_security_01` | Prompt Injection Defense | Security patterns |
| `production_budget_01` | Cost/Latency Budgets | Operational guardrails |

## Commands

| Command | Description |
|---------|-------------|
| `vibelings` | Start watch mode (default) |
| `vibelings init` | Initialize workspace |
| `vibelings list` | List exercises with status |
| `vibelings run <exercise>` | Run single exercise |
| `vibelings hint [exercise]` | Get progressive hints |
| `vibelings verify` | Verify completed exercises |
| `vibelings replay <run_id>` | Replay a trace for debugging |
| `vibelings doctor` | Check environment setup |
| `vibelings cost` | Show token costs |
| `vibelings progress` | Show curriculum progress dashboard |
| `vibelings reset <exercise>` | Reset to starter state |

## Configuration

User configuration lives in `~/.config/vibelings/config.toml`. See [`config.example.toml`](config.example.toml) for a fully documented example.

```toml
[model]
provider = "openrouter"
model = "anthropic/claude-sonnet-4-20250514"
temperature = 0

[openrouter]
api_key_env = "OPENROUTER_API_KEY"
zdr = true  # Zero Data Retention

[sandbox]
network = false
timeout_seconds = 30

[display]
show_cost = true
show_trace = true
```

### Provider Options

| Provider | Description | Setup |
|----------|-------------|-------|
| `openrouter` | Multi-provider API (default) | `export OPENROUTER_API_KEY="..."` |
| `openai` | Direct OpenAI API | `export OPENAI_API_KEY="..."` |
| `anthropic` | Direct Anthropic API | `export ANTHROPIC_API_KEY="..."` |
| `local` | Local OpenAI-compatible server | Configure `base_url` in config |

## Grading Philosophy

Exercises are graded using **deterministic methods**:

| Method | Use Case |
|--------|----------|
| **Schema validation** | Structured JSON output |
| **Sandbox state** | Tool call sequences |
| **Invariant scripts** | Custom conditions |
| **Multi-run reliability** | K runs, N must pass |

LLM-as-judge is intentionally a last resort. Deterministic grading means:
- Reproducible results
- Clear failure messages
- No "it worked on my machine" problems

## Progress Tracking

Exercise status uses honest indicators:

| Symbol | Status | Meaning |
|--------|--------|---------|
| `⏳` | Pending | Not yet attempted |
| `🔄` | In Progress | Currently working |
| `✅` | Completed | Passed deterministic checks |
| `🟡` | Flaky | Passed but under reliability threshold |
| `🔁` | Needs Reruns | Multi-run exercise, insufficient data |

Progress is saved in `~/.config/vibelings/progress.toml`.

## Development

```bash
# Build
cargo build

# Run tests (180 tests)
cargo test

# Run with debug output
cargo run -- run fundamentals/json_01 --verbose
```

## Contributing

Contributions are welcome! See [CLAUDE.md](CLAUDE.md) for architecture guidance.

### Exercise Contributions

- Must be gradable deterministically
- Clear README with learning objectives
- Tested on at least two model providers
- No exercises requiring expensive models without alternatives

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- [Rustlings](https://github.com/rust-lang/rustlings) for the CLI UX inspiration
- [Model Context Protocol](https://spec.modelcontextprotocol.io/) for tool standards
- The agentic AI community for pushing reliability forward

---

*"Vibes" got us here. Engineering gets us to production.*

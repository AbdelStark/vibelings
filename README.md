# vibelings

```
╭──────────────────────────────────────────────────────────────────────────────╮
│                                                                              │
│  ██╗   ██╗██╗██████╗ ███████╗██╗     ██╗███╗   ██╗ ██████╗ ███████╗          │
│  ██║   ██║██║██╔══██╗██╔════╝██║     ██║████╗  ██║██╔════╝ ██╔════╝          │
│  ██║   ██║██║██████╔╝█████╗  ██║     ██║██╔██╗ ██║██║  ███╗███████╗          │
│  ╚██╗ ██╔╝██║██╔══██╗██╔══╝  ██║     ██║██║╚██╗██║██║   ██║╚════██║          │
│   ╚████╔╝ ██║██████╔╝███████╗███████╗██║██║ ╚████║╚██████╔╝███████║          │
│    ╚═══╝  ╚═╝╚═════╝ ╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚══════╝          │
│                                                                              │
│            Rustlings for Agentic Programming                                 │
│            Learn to build reliable AI agents                                 │
│                                                                              │
╰──────────────────────────────────────────────────────────────────────────────╯
```

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-323%20passing-brightgreen.svg)]()

> **"Vibes" got us here. Engineering gets us to production.**

---

## The Problem

You can prompt an LLM to "act like an agent." But can you:

- **Guarantee** it returns valid JSON every time?
- **Measure** its reliability across 100 runs?
- **Catch** prompt injection before it reaches your tools?
- **Debug** why it called the wrong function yesterday?

Most "agent" tutorials teach you to hope for the best. Vibelings teaches you to **engineer for reliability**.

## The Solution

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                     │
│   "Prompt engineering"          │     Agentic Engineering          │
│   ─────────────────────         │     ────────────────────          │
│                                 │                                   │
│   • Hope the LLM cooperates     │     • Define schemas that MUST    │
│   • Debug by reading prompts    │       be satisfied                │
│   • "It works on my machine"    │     • Debug with traces & metrics │
│   • Cross fingers in prod       │     • Reproducible grading        │
│                                 │     • Measured reliability        │
│                                 │                                   │
└─────────────────────────────────────────────────────────────────────┘
```

Vibelings is a terminal-first, exercise-driven curriculum that teaches you to build **reliable** agentic AI systems through hands-on practice with **deterministic grading**.

Like [Rustlings](https://github.com/rust-lang/rustlings) teaches Rust through small exercises, Vibelings teaches agentic programming through progressive challenges where you can't fool yourself about whether it works.

---

## Philosophy

These four principles guide every exercise:

```
┌─────────────────────┐     ┌─────────────────────┐
│  CONTRACTS          │     │  OBSERVABILITY      │
│  ─────────          │     │  ─────────────      │
│  Schemas, not hopes │     │  Traces, not logs   │
│  Validate, don't    │     │  Measure costs      │
│  hope               │     │  Debug from data    │
└─────────────────────┘     └─────────────────────┘
         ▲                           ▲
         │       RELIABILITY         │
         │       ───────────         │
         │   Design for failure      │
         │   Test systematically     │
         │   Verify deterministically│
         ▼                           ▼
┌─────────────────────┐     ┌─────────────────────┐
│  SECURITY           │     │  CONTEXT            │
│  ────────           │     │  ───────            │
│  Least privilege    │     │  Budget allocation  │
│  Sandbox by default │     │  Token efficiency   │
│  Assume compromise  │     │  Strategic curation │
└─────────────────────┘     └─────────────────────┘
```

### What This Is NOT

- Not a prompt engineering tutorial
- Not a framework-specific SDK wrapper
- Not about "getting the LLM to say the right thing"

### What This IS

- A practical training ground for the modern agent stack (MCP, tool calling, structured output)
- Exercises that teach: **design contracts + tools + guardrails → measure reliability**
- A path to competence in agentic engineering with **verifiable progress**

---

## Quick Start

### Install

```bash
# Option 1: Quick install (Linux/macOS)
curl -sSL https://raw.githubusercontent.com/AbdelStark/vibelings/main/install.sh | bash

# Option 2: From source
cargo install --git https://github.com/AbdelStark/vibelings

# Option 3: Clone and build
git clone https://github.com/AbdelStark/vibelings.git && cd vibelings && cargo install --path .
```

### Setup

```bash
# Initialize workspace
vibelings init

# Set your API key (get one at openrouter.ai)
export OPENROUTER_API_KEY="your-key-here"

# Verify setup
vibelings doctor
```

### Learn

```bash
# Start the curriculum
vibelings

# Or run a specific exercise
vibelings run fundamentals/json_01

# Get help when stuck
vibelings hint
```

---

## The Curriculum

### Track 1: Agentic Fundamentals

The core primitives. No frameworks—just the concepts that everything else builds on.

```
json_01 ──► json_02 ──► json_03
   │           │           │
   ▼           ▼           ▼
 Schema     Nested      Arrays
 basics     objects     validation
   │
   ├──────────────────────────────┐
   │                              │
   ▼                              ▼
tools_01 ──► tools_02        error_01
   │            │                │
   ▼            ▼                ▼
 Basic       Multi-step      Recovery
 calling     sequences       patterns
                                 │
              guardrails_01◄─────┤
                   │             │
                   ▼             │
               Validation        │
                   │             │
              observability_01◄──┘
                   │
                   ▼
               Tracing
```

| Exercise | What You Learn |
|----------|----------------|
| `json_01` | JSON Schema validation fundamentals |
| `json_02` | Complex nested object schemas |
| `json_03` | Array validation with constraints |
| `tools_01` | Tool schemas and invocation |
| `tools_02` | Multi-tool orchestration |
| `error_01` | Error resilience patterns |
| `guardrails_01` | Input/output validation |
| `observability_01` | Tracing and cost awareness |

### Track 2: MCP in Practice

The Model Context Protocol—the emerging standard for tool interoperability.

| Exercise | What You Learn |
|----------|----------------|
| `server_01` | Define MCP tools correctly |
| `client_01` | Construct valid JSON-RPC requests |
| `resource_01` | Define MCP resources for data access |

### Track 3: Workflow Orchestration

Integration with real workflow tools like n8n.

| Exercise | What You Learn |
|----------|----------------|
| `workflow_json_01` | n8n-style workflow structure |
| `workflow_tool_wiring_01` | Data transformation between steps |
| `workflow_human_loop_01` | Approval gates and fallbacks |

### Track 4: Production Engineering

What separates demos from production systems.

| Exercise | What You Learn |
|----------|----------------|
| `production_eval_01` | Evaluation harness design |
| `production_security_01` | Prompt injection defense |
| `production_budget_01` | Cost and latency budgets |

### Track 5: Context Engineering

Managing the finite resource of context effectively.

| Exercise | What You Learn |
|----------|----------------|
| `context_01` | System prompt structure |
| `context_02` | Context budget allocation |
| `context_03` | Just-in-time retrieval |
| `context_04` | Conversation compaction |
| `context_05` | Token-efficient tool design |

---

## Why Deterministic Grading?

```
Traditional LLM "testing":          Vibelings approach:

     ┌──────────┐                       ┌──────────┐
     │  Prompt  │                       │  Prompt  │
     └────┬─────┘                       └────┬─────┘
          │                                  │
          ▼                                  ▼
     ┌──────────┐                       ┌──────────┐
     │   LLM    │                       │   LLM    │
     └────┬─────┘                       └────┬─────┘
          │                                  │
          ▼                                  ▼
     ┌──────────┐                       ┌──────────┐
     │  Human   │ "Looks good?"         │  Schema  │ Valid JSON?
     │  Review  │                       │ Validate │ Types match?
     └────┬─────┘                       └────┬─────┘
          │                                  │
          ▼                                  ▼
     ┌──────────┐                       ┌──────────┐
     │  "LGTM"  │ (subjective)          │  PASS or │ (objective)
     └──────────┘                       │  FAIL    │
                                        └──────────┘
```

LLM outputs are stochastic. Subjective evaluation means:
- Results aren't reproducible
- "It worked for me" isn't verifiable
- You can't measure improvement

Deterministic grading means:
- **Schema validation** — Does the output match the contract?
- **Sandbox state** — Did the right tools get called?
- **Invariant scripts** — Are custom conditions satisfied?
- **Multi-run reliability** — Does it work 4 out of 5 times?

You can't fool the grader. Either it passes or it doesn't.

---

## Commands

| Command | Description |
|---------|-------------|
| `vibelings` | Start watch mode (default) |
| `vibelings init` | Initialize workspace |
| `vibelings list` | List exercises with status |
| `vibelings run <exercise>` | Run a single exercise |
| `vibelings hint [exercise]` | Get progressive hints |
| `vibelings verify` | Verify completed exercises |
| `vibelings replay <run_id>` | Replay a trace for debugging |
| `vibelings doctor` | Check environment setup |
| `vibelings cost` | Show token costs |
| `vibelings progress` | Show curriculum progress |
| `vibelings reset <exercise>` | Reset to starter state |

### Watch Mode

The default mode. Runs exercises and re-runs on file changes.

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

## Configuration

Configuration lives in `~/.config/vibelings/config.toml`:

```toml
[model]
provider = "openrouter"
model = "anthropic/claude-sonnet-4-20250514"
temperature = 0

[openrouter]
api_key_env = "OPENROUTER_API_KEY"
zdr = true                    # Zero Data Retention
data_collection = "deny"

[sandbox]
network = false               # Locked down by default
timeout_seconds = 30
allowed_commands = ["cat", "ls", "grep", "jq"]

[display]
show_cost = true
show_trace = true
```

### Provider Options

| Provider | Setup |
|----------|-------|
| OpenRouter (default) | `export OPENROUTER_API_KEY="..."` |
| OpenAI | `export OPENAI_API_KEY="..."` |
| Anthropic | `export ANTHROPIC_API_KEY="..."` |
| Local (Ollama, vLLM) | Configure `base_url` in config |

---

## Progress Tracking

Honest indicators of your progress:

| Symbol | Status | Meaning |
|--------|--------|---------|
| `⏳` | Pending | Not yet attempted |
| `🔄` | In Progress | Currently working |
| `✅` | Completed | Passed deterministic checks |
| `🟡` | Flaky | Passed but under reliability threshold |
| `🔁` | Needs Reruns | Multi-run exercise, insufficient data |

Progress is saved in `~/.config/vibelings/progress.toml`.

---

## Documentation

| Document | Purpose |
|----------|---------|
| [GETTING_STARTED.md](docs/GETTING_STARTED.md) | Complete beginner guide |
| [LEARNING.md](docs/LEARNING.md) | Learning philosophy and curriculum structure |
| [AUTHORING.md](docs/AUTHORING.md) | How to create exercises |
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | Technical system design |
| [TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) | Common issues and solutions |
| [FAQ.md](docs/FAQ.md) | Frequently asked questions |
| [CLAUDE.md](CLAUDE.md) | AI agent guidance |
| [SECURITY.md](SECURITY.md) | Security model and reporting |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to contribute |
| [CHANGELOG.md](CHANGELOG.md) | Version history |

---

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Exercise Contributions

We especially welcome new exercises. Requirements:

- **Deterministically gradable** — Schema or invariant-based grading
- **Clear learning objectives** — Each exercise teaches one concept
- **Multi-provider tested** — Works with at least two model providers
- **Documented** — README with goal, task, and grading criteria

---

## License

MIT License — see [LICENSE](LICENSE) for details.

---

## Acknowledgments

- [Rustlings](https://github.com/rust-lang/rustlings) — Inspiration for the exercise-driven CLI
- [Model Context Protocol](https://spec.modelcontextprotocol.io/) — The tool protocol standard
- [Anthropic](https://www.anthropic.com/) — Context engineering guidance and Claude documentation

---

<p align="center">
  <strong>Stop hoping. Start engineering.</strong>
</p>

# Getting Started with Vibelings

> Your complete guide to learning agentic programming

This guide will walk you through everything you need to start your journey from "LLM user" to "agentic systems engineer."

---

## Prerequisites

Before you begin, you'll need:

| Requirement | Why | How to get it |
|-------------|-----|---------------|
| **API Key** | To run exercises against LLMs | Sign up at [openrouter.ai](https://openrouter.ai) (free tier available) |
| **Terminal** | Vibelings is terminal-first | Built into macOS/Linux; use Windows Terminal on Windows |
| **Rust** (optional) | Only if building from source | [rustup.rs](https://rustup.rs) |

That's it. No Python environments, no Docker, no complex setup.

---

## Installation

### Option 1: Quick Install (Recommended)

**Linux / macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/AbdelStark/vibelings/main/install.sh | bash
```

**Windows (PowerShell as Administrator):**
```powershell
irm https://raw.githubusercontent.com/AbdelStark/vibelings/main/install.ps1 | iex
```

### Option 2: From Source (If you have Rust)

```bash
cargo install --git https://github.com/AbdelStark/vibelings
```

### Verify Installation

```bash
vibelings --version
# Should output: vibelings 0.1.x
```

---

## First-Time Setup

### Step 1: Initialize Your Workspace

```bash
vibelings init
```

This creates:
- `~/.config/vibelings/config.toml` — Your settings
- `~/.config/vibelings/progress.toml` — Your exercise progress
- `~/vibelings-workspace/` — Where you'll work on exercises

### Step 2: Get an API Key

Vibelings uses LLM APIs to run exercises. The default provider is [OpenRouter](https://openrouter.ai), which gives you access to many models with one API key.

1. Go to [openrouter.ai](https://openrouter.ai)
2. Create an account (GitHub/Google sign-in available)
3. Navigate to **Keys** and create a new API key
4. Copy the key (it starts with `sk-or-...`)

### Step 3: Set Your API Key

**Linux / macOS:**
```bash
# Add to your shell profile (~/.bashrc, ~/.zshrc, etc.)
export OPENROUTER_API_KEY="sk-or-v1-your-key-here"

# Then reload
source ~/.bashrc  # or ~/.zshrc
```

**Windows (PowerShell):**
```powershell
# For current session
$env:OPENROUTER_API_KEY="sk-or-v1-your-key-here"

# To persist, add to your PowerShell profile
```

### Step 4: Verify Everything Works

```bash
vibelings doctor
```

You should see:
```
vibelings doctor
━━━━━━━━━━━━━━━━━
✓ Config file found
✓ Progress file found
✓ API key configured
✓ Model accessible
✓ Ready to learn!
```

If you see errors, check the [Troubleshooting Guide](TROUBLESHOOTING.md).

---

## Your First Exercise

Let's run your first exercise to see how vibelings works.

### Step 1: List Available Exercises

```bash
vibelings list
```

You'll see all exercises organized by track:

```
Track: Agentic Fundamentals
─────────────────────────────
⏳ fundamentals/json_01 - JSON Output Contracts
⏳ fundamentals/json_02 - Complex Nested Schemas
⏳ fundamentals/tools_01 - Basic Tool Calling
...
```

### Step 2: Start the First Exercise

```bash
vibelings run fundamentals/json_01
```

This will:
1. Load the exercise
2. Show you the instructions
3. Send your prompt to the LLM
4. Grade the response
5. Tell you if you passed

### Step 3: Understand the Output

```
━━━ Exercise: fundamentals/json_01 ━━━
Goal: Generate valid JSON matching the Person schema

Running...

✅ PASSED (0.8s, $0.002)

Schema validation: ✓
  - name: present, valid string
  - age: present, valid integer
  - email: present, valid format

Tokens: 342 in / 89 out
Cost: $0.002
```

Congratulations! You just completed your first exercise.

---

## Understanding the Exercise Workflow

Each exercise follows this pattern:

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│   1. READ THE README                                            │
│      └── exercises/<track>/<id>/README.md                       │
│          Explains what you're learning and why it matters       │
│                                                                 │
│   2. EDIT THE PROMPT                                            │
│      └── exercises/<track>/<id>/starter/prompt.txt              │
│          Your system prompt and/or user message                 │
│                                                                 │
│   3. RUN THE EXERCISE                                           │
│      └── vibelings run <track>/<id>                             │
│          Sends to LLM, validates response, shows result         │
│                                                                 │
│   4. ITERATE                                                    │
│      └── If failed, read error message, adjust prompt, retry    │
│          Use 'vibelings hint' if stuck                          │
│                                                                 │
│   5. MOVE ON                                                    │
│      └── Once passed, move to next exercise                     │
│          Progress is automatically saved                        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### The Files in Each Exercise

```
exercises/fundamentals/json_01/
├── README.md           # 📖 Learning content and task description
├── manifest.toml       # ⚙️  Exercise configuration (don't edit)
├── hints.md            # 💡 Progressive hints if you get stuck
├── starter/
│   └── prompt.txt      # ✏️  YOUR PROMPT - edit this file
└── grader/
    └── schema.json     # ✓  What the output must conform to
```

**You edit `starter/prompt.txt`.** Everything else is read-only reference material.

---

## Watch Mode

For active learning, use watch mode instead of running exercises one at a time:

```bash
vibelings
```

This starts an interactive session that:
- Runs the current exercise
- Watches for file changes
- Automatically re-runs when you save
- Provides keyboard shortcuts

### Watch Mode Controls

| Key | Action |
|-----|--------|
| `h` | Show hints for current exercise |
| `n` | Move to next exercise (marks current as complete if passed) |
| `l` | List all exercises with progress |
| `q` | Quit watch mode |

---

## Getting Help When Stuck

### Option 1: Progressive Hints

Each exercise has layered hints, from general direction to complete solution:

```bash
vibelings hint                    # Hint for current exercise
vibelings hint fundamentals/json_01  # Hint for specific exercise
```

Hints are revealed progressively. Run the command multiple times for more help.

### Option 2: Read the Grader

Look at what's being validated:

```bash
cat exercises/fundamentals/json_01/grader/schema.json
```

This shows you exactly what the output must look like.

### Option 3: Check the Trace

After a failed run, see what was actually sent and received:

```bash
vibelings replay
```

This replays the last trace, showing:
- The exact prompt sent
- The LLM's response
- Why grading failed

---

## Understanding the Curriculum

The exercises are organized into five tracks, designed to be completed in order:

```
     WEEK 1-2                      WEEK 3-4                    WEEK 5-6
┌────────────────┐         ┌────────────────────┐       ┌────────────────┐
│                │         │                    │       │                │
│  FUNDAMENTALS  │ ──────► │   CHOOSE YOUR      │ ───►  │   PRODUCTION   │
│                │         │   SPECIALIZATION   │       │                │
│  - JSON        │         │                    │       │  - Evaluation  │
│  - Tools       │         │  ┌──────────────┐  │       │  - Security    │
│  - Errors      │         │  │     MCP      │  │       │  - Budgets     │
│  - Guardrails  │         │  └──────────────┘  │       │                │
│  - Observability│        │  ┌──────────────┐  │       └────────────────┘
│                │         │  │  Workflows   │  │
└────────────────┘         │  └──────────────┘  │
                           │  ┌──────────────┐  │
                           │  │   Context    │  │
                           │  └──────────────┘  │
                           │                    │
                           └────────────────────┘
```

### Track 1: Agentic Fundamentals (Start Here)

The core concepts that everything else builds on:

| Exercise | Concept | Why It Matters |
|----------|---------|----------------|
| `json_01` | Schema validation | Contracts guarantee structure |
| `json_02` | Nested schemas | Real-world data is complex |
| `json_03` | Array validation | Collections need constraints |
| `tools_01` | Tool calling | Agents act through tools |
| `tools_02` | Tool sequences | Multi-step operations |
| `error_01` | Error handling | Failures are expected |
| `guardrails_01` | Validation | Safety boundaries |
| `observability_01` | Tracing | Debug with data |

**Complete this track first.** The other tracks assume you've mastered these concepts.

### Track 2: MCP in Practice

The Model Context Protocol for tool interoperability:

- `server_01` — Define MCP-compatible tools
- `client_01` — Call tools via JSON-RPC
- `resource_01` — Expose data sources

### Track 3: Workflow Orchestration

Integration with workflow automation tools:

- `workflow_json_01` — Workflow structure
- `workflow_tool_wiring_01` — Data transformation
- `workflow_human_loop_01` — Human checkpoints

### Track 4: Production Engineering

What makes agents production-ready:

- `production_eval_01` — Systematic testing
- `production_security_01` — Prompt injection defense
- `production_budget_01` — Cost control

### Track 5: Context Engineering

Managing the finite resource of context:

- `context_01` — System prompt structure
- `context_02` — Token budget allocation
- `context_03` — Just-in-time retrieval
- `context_04` — Conversation compaction
- `context_05` — Efficient tool design

---

## Tips for Success

### 1. Read the README First

Every exercise has a README that explains:
- **Why** this concept matters
- **What** you're building
- **How** it will be graded

Don't skip this. Understanding the "why" helps the "how" stick.

### 2. Look at the Schema

The grader schema tells you exactly what valid output looks like:

```bash
cat exercises/<track>/<id>/grader/schema.json
```

Your output must match this. Not approximately—exactly.

### 3. Use Hints Progressively

The hint system is designed for learning, not just getting answers:

1. First hint: General direction
2. Second hint: More specific guidance
3. Third hint: Detailed approach
4. Final hint: Complete example

Try to solve it before using hints. Use hints when truly stuck.

### 4. Check the Cost

Every exercise shows its cost:

```bash
vibelings cost
```

Budget-conscious learners can optimize prompts for efficiency.

### 5. Review Traces When Debugging

If an exercise fails, don't just guess what went wrong:

```bash
vibelings replay
```

See exactly what happened. Debug from data, not intuition.

---

## Configuring Vibelings

Your configuration is in `~/.config/vibelings/config.toml`:

### Change the Model

```toml
[model]
provider = "openrouter"
model = "anthropic/claude-sonnet-4-20250514"  # Change this
temperature = 0
```

### Use a Different Provider

```toml
[model]
provider = "openai"  # or "anthropic", "local"

[openai]
api_key_env = "OPENAI_API_KEY"
```

### Enable Privacy Controls

```toml
[openrouter]
zdr = true           # Zero Data Retention
data_collection = "deny"
```

### Adjust Sandbox Settings

```toml
[sandbox]
network = false      # Keep this false unless exercise requires it
timeout_seconds = 30
allowed_commands = ["cat", "ls", "grep", "jq"]
```

---

## Next Steps

Now that you're set up:

1. **Start with `fundamentals/json_01`** — Your first schema validation exercise
2. **Work through the fundamentals track** — Takes 1-2 weeks at casual pace
3. **Choose a specialization** — MCP, workflows, or context engineering
4. **Complete production track** — What separates demos from real systems
5. **Build something real** — Apply what you've learned

---

## Common Questions

### "How long does the curriculum take?"

- **Casual pace**: 4-6 weeks (a few exercises per day)
- **Intensive pace**: 2-3 weeks (several exercises per day)
- **Deep learning**: 6-8 weeks (with additional reading)

### "Which model should I use?"

Start with the default (`anthropic/claude-sonnet-4-20250514`). It's capable enough for all exercises and cost-effective.

### "What if I can't pass an exercise?"

1. Read the error message carefully
2. Check the schema to understand expected format
3. Use `vibelings hint` progressively
4. Review the trace with `vibelings replay`
5. Read the exercise README again

If still stuck, the final hint always contains a working solution.

### "Can I skip exercises?"

Technically yes, but the curriculum is designed to build progressively. Earlier exercises teach concepts used in later ones.

---

## Getting Help

- **Documentation**: Check the `docs/` directory
- **Troubleshooting**: See [TROUBLESHOOTING.md](TROUBLESHOOTING.md)
- **FAQ**: See [FAQ.md](FAQ.md)
- **Bugs**: Open an issue on [GitHub](https://github.com/AbdelStark/vibelings/issues)

---

**Ready to start?** Run your first exercise:

```bash
vibelings run fundamentals/json_01
```

Welcome to agentic engineering.

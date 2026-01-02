# Frequently Asked Questions

> Answers to common questions about vibelings

---

## General Questions

### What is vibelings?

Vibelings is a terminal-first, exercise-driven curriculum for learning to build reliable agentic AI systems. Think "Rustlings, but for agentic programming."

Instead of teaching you prompt tricks, it teaches you engineering disciplines:
- **Contracts**: Define schemas that outputs must satisfy
- **Observability**: Debug with traces, not intuition
- **Reliability**: Measure success across multiple runs
- **Security**: Defend against prompt injection

### Why "vibelings"?

The name is a play on "[Rustlings](https://github.com/rust-lang/rustlings)" (the Rust exercise tool) and "vibes."

The tagline says it best: *"Vibes" got us here. Engineering gets us to production.*

Most people "vibe" with LLMs—prompting by feel, hoping it works. Vibelings teaches you to engineer with precision.

### Who is vibelings for?

- **Software engineers** learning to build with LLMs
- **ML engineers** who want to understand agentic patterns
- **Technical leads** evaluating AI engineering skills
- **Hobbyists** who want to go beyond prompting tutorials

You should be comfortable with a terminal and basic JSON. Prior LLM experience helps but isn't required.

### How is vibelings different from other LLM tutorials?

| Other Tutorials | Vibelings |
|-----------------|-----------|
| Teach prompt engineering | Teaches system engineering |
| Subjective evaluation | Deterministic grading |
| "Here's a cool demo" | "Here's how to make it reliable" |
| Copy-paste prompts | Understand contracts |
| Hope it works | Verify it works |

The core difference: **vibelings has a grader that can't be fooled.** Either your output matches the schema or it doesn't.

---

## Learning Questions

### How long does the curriculum take?

| Pace | Time to Complete |
|------|------------------|
| Casual (1-2 exercises/day) | 4-6 weeks |
| Moderate (3-5 exercises/day) | 2-3 weeks |
| Intensive (full days) | 1-2 weeks |

The fundamentals track takes about 1-2 weeks. Add another week per additional track.

### What order should I do the exercises?

**Recommended path**:

1. **Fundamentals** (required) — Complete all 8 exercises first
2. **Choose one specialization**:
   - **MCP** — If you're building interoperable tools
   - **Workflows** — If you're integrating with automation systems
   - **Context** — If you're optimizing prompt performance
3. **Production** — Complete after at least one specialization

### Can I skip exercises?

You can, but it's not recommended. Later exercises build on concepts from earlier ones.

If you must skip:
```bash
# Mark an exercise as completed without running it
# (Not officially supported—edit progress.toml manually)
```

### What if I can't pass an exercise?

1. **Read the error message** — It tells you what failed
2. **Check the schema** — `cat exercises/<track>/<id>/grader/schema.json`
3. **Use hints** — `vibelings hint` (run multiple times for more hints)
4. **Replay the trace** — `vibelings replay` to see what happened
5. **Read the README** — The concept section explains the underlying idea

The final hint always contains a complete solution.

### Are the exercises just about prompting?

No. While you edit prompts, the exercises teach you to think in terms of:
- **Schemas** — Defining what valid output looks like
- **Validation** — Checking outputs match contracts
- **Tool calling** — Structuring agent capabilities
- **Error handling** — Designing for failure
- **Security** — Preventing prompt injection
- **Context** — Allocating the limited token budget

These are engineering skills, not prompting skills.

---

## Technical Questions

### Which models work best?

**Recommended**: `anthropic/claude-sonnet-4-20250514`
- Best balance of capability and cost
- Works for all exercises
- Supports tool calling

**Budget-friendly**: `mistralai/mixtral-8x7b-instruct`
- Much cheaper
- Works for most fundamentals exercises
- May struggle with complex exercises

**Local (free)**: Ollama with `llama3` or `codellama`
- No API costs
- Requires local compute
- Some exercises may need capable models

### How much does it cost to complete the curriculum?

With Claude Sonnet on OpenRouter:
- **Full curriculum**: $2–5 total
- **Per exercise**: ~$0.01–0.05

With GPT-4o:
- **Full curriculum**: $5–10 total

With local models:
- **$0** (just compute)

You can track costs with `vibelings cost`.

### Can I use my own API key for providers directly?

Yes. Configure direct provider access:

```toml
# For OpenAI directly
[model]
provider = "openai"

[openai]
api_key_env = "OPENAI_API_KEY"
```

```toml
# For Anthropic directly
[model]
provider = "anthropic"

[anthropic]
api_key_env = "ANTHROPIC_API_KEY"
```

### Can I use local models with Ollama?

Yes:

```toml
[model]
provider = "local"
model = "llama3"

[local]
base_url = "http://localhost:11434/v1"
```

Start Ollama first:
```bash
ollama run llama3
```

### Why does grading use schemas instead of LLMs?

LLM-based grading has problems:
- **Non-deterministic**: Same output might pass/fail differently
- **Subjective**: "Close enough" is hard to define
- **Expensive**: Every grade costs money
- **Gameable**: You can prompt-engineer the grader

Schema-based grading is:
- **Deterministic**: Same output always has same result
- **Objective**: Matches schema or doesn't
- **Free**: No API calls for validation
- **Rigorous**: Forces you to understand the contract

---

## Configuration Questions

### Where are configuration files stored?

| File | Location |
|------|----------|
| Config | `~/.config/vibelings/config.toml` |
| Progress | `~/.config/vibelings/progress.toml` |
| Exercises | `~/vibelings-workspace/` (after `init`) |

### How do I reset my progress?

```bash
rm ~/.config/vibelings/progress.toml
vibelings init
```

This clears all exercise completions.

### How do I reset a single exercise?

```bash
vibelings reset fundamentals/json_01
```

This restores the starter files.

### Can I use vibelings in a team or classroom?

Yes. Each person runs their own instance with their own API key.

For classroom settings:
1. Each student installs vibelings
2. Each uses their own OpenRouter key (or shared budget)
3. Progress is local to each machine

---

## Philosophy Questions

### Why focus on "reliability" instead of "capability"?

Capability is easy to demo. Reliability is hard to achieve.

Most LLM tutorials show you how to make something work once. Vibelings teaches you how to make something work consistently, verifiably, and safely.

The difference matters in production:
- A demo that works 80% of the time is impressive
- A production system that fails 20% of the time is unusable

### Why are exercises so focused on JSON?

JSON is the lingua franca of structured data. Understanding JSON Schema teaches you:

- **Type constraints** — What data types are valid
- **Required fields** — What must be present
- **Validation** — How to check correctness
- **Contracts** — How to define interfaces

These concepts apply beyond JSON to any structured output.

### Why MCP instead of proprietary tool formats?

MCP (Model Context Protocol) is an open standard. Learning MCP means:

- Your skills transfer across providers
- Your tools work with any MCP-compatible client
- You're not locked into one vendor

It's the difference between learning one framework vs. learning web standards.

### Why is there no "prompt library" or "best prompts"?

Vibelings intentionally doesn't provide copy-paste solutions because:

1. **Prompts aren't portable** — What works on Claude may not work on GPT
2. **Prompts drift** — Models change, prompts need updating
3. **Understanding beats copying** — Knowing why it works > knowing what to paste

The exercises teach you to design prompts, not memorize them.

---

## Troubleshooting Questions

### Where can I find help for errors?

1. **Check the docs**: [TROUBLESHOOTING.md](TROUBLESHOOTING.md)
2. **Run diagnostics**: `vibelings doctor --full`
3. **File an issue**: [GitHub Issues](https://github.com/AbdelStark/vibelings/issues)

### Why is my exercise passing locally but failing differently elsewhere?

LLM outputs are non-deterministic. Even with `temperature=0`, you may get slight variations.

For reliability exercises, the curriculum uses multi-run grading (e.g., "pass 4 out of 5 runs").

### Can I contribute exercises?

Yes! See [CONTRIBUTING.md](../CONTRIBUTING.md) and [AUTHORING.md](AUTHORING.md).

Requirements:
- Deterministically gradable (no LLM-as-judge)
- Clear learning objectives
- Tested on at least two model providers

---

## Future Questions

### Will there be more exercises?

Yes. Planned additions:
- More production patterns
- Agent orchestration
- Evaluation harness design
- Multi-agent coordination

### Will vibelings support other languages?

The CLI is in Rust, but exercises are language-agnostic. You're editing prompts and validating against schemas.

Future: There may be language-specific tracks (e.g., integrating with LangChain, LlamaIndex) but the core curriculum stays universal.

### Is there a certification or badge?

Not currently. The value is in the skills, not a credential.

When you complete the curriculum, you can:
- Reference it on your resume
- Demonstrate skills by explaining the concepts
- Build real systems using what you learned

---

## Still Have Questions?

- Check the [documentation](../README.md)
- Search [existing issues](https://github.com/AbdelStark/vibelings/issues)
- Open a [new issue](https://github.com/AbdelStark/vibelings/issues/new)

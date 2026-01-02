# Learning Philosophy

> The conceptual framework behind the vibelings curriculum

This document explains *why* the curriculum is structured the way it is, the mental models you'll develop, and how the tracks connect to form a coherent understanding of agentic engineering.

---

## The Core Insight

**LLMs are unreliable components in otherwise deterministic systems.**

This is not a bug to be fixed with better prompts—it's a fundamental property of the technology. The discipline of agentic engineering is about building reliable systems *around* unreliable components.

Think of it like building earthquake-resistant buildings. You don't try to prevent earthquakes; you design structures that remain functional despite them. The same principle applies here: we don't try to make LLMs perfectly reliable; we build systems that remain reliable despite imperfect LLM outputs.

---

## The Five Disciplines

Vibelings teaches five interconnected disciplines. Each track focuses on one, but they're meant to work together.

```
                    ┌─────────────────────┐
                    │   OBSERVABILITY     │
                    │  (Can you see what  │
                    │   is happening?)    │
                    └──────────┬──────────┘
                               │
    ┌──────────────────────────┼──────────────────────────┐
    │                          │                          │
    ▼                          ▼                          ▼
┌─────────┐            ┌───────────────┐           ┌──────────┐
│CONTRACTS│◄──────────►│  RELIABILITY  │◄─────────►│ SECURITY │
│ (What   │            │  (Does it     │           │  (Is it  │
│ should  │            │  keep working?)│           │  safe?)  │
│ happen?)│            └───────────────┘           └──────────┘
└─────────┘                    ▲
    │                          │
    │      ┌───────────────────┘
    │      │
    ▼      ▼
┌────────────────┐
│    CONTEXT     │
│ (What does the │
│  LLM know?)    │
└────────────────┘
```

### 1. Contracts (Fundamentals Track)

**The Question**: "What should the LLM output look like?"

**The Insight**: If you don't define success precisely, you can't detect failure. Contracts are explicit specifications—JSON schemas, tool definitions, type constraints—that make success/failure deterministic.

**The Analogy**: APIs have contracts (OpenAPI specs). Databases have schemas. Type systems enforce invariants at compile time. LLM outputs need the same discipline.

**The Mental Shift**: Stop thinking "did the LLM say something reasonable?" and start thinking "does this output satisfy the contract?"

### 2. Reliability (Fundamentals Track)

**The Question**: "Does the system keep working when things go wrong?"

**The Insight**: Failures are not exceptional—they're expected. Every tool call can fail. Every external service can timeout. Every LLM response can be malformed. Design for this.

**The Analogy**: Distributed systems engineering. Circuit breakers, retries with backoff, graceful degradation. These patterns existed before LLMs; they apply directly.

**The Mental Shift**: Stop assuming calls succeed. Start designing for failure as the default case.

### 3. Security (Production Track)

**The Question**: "Is the system safe to expose to untrusted input?"

**The Insight**: LLMs are instruction-following machines connected to your tools. If an attacker controls the input, they can potentially control the output—and the tools. This is prompt injection, and it's the SQL injection of the AI era.

**The Analogy**: Never trust user input. This principle is decades old. Apply it to everything that reaches the LLM, and to everything the LLM outputs.

**The Mental Shift**: Stop treating the LLM as a trusted component. Start treating it as a potentially compromised intermediary.

### 4. Context (Context Track)

**The Question**: "What does the LLM know when it generates a response?"

**The Insight**: Context is a finite resource. Every token spent on low-signal content is a token unavailable for high-signal content. Context engineering is the discipline of curating tokens strategically.

**The Analogy**: Memory management in systems programming. You have a fixed amount of RAM; you must decide what to keep resident and what to page out. Context windows work the same way.

**The Mental Shift**: Stop stuffing everything into the prompt. Start treating context as a budget to be allocated deliberately.

### 5. Observability (Fundamentals Track)

**The Question**: "Can you see what's actually happening?"

**The Insight**: Non-determinism makes debugging hard. If you can't see the inputs, outputs, tool calls, and costs of every interaction, you're flying blind. Traces are your primary debugging tool.

**The Analogy**: Distributed tracing (Jaeger, Zipkin) for microservices. You need to follow a request through the system. Same applies to agent interactions.

**The Mental Shift**: Stop debugging by reading prompts and guessing. Start instrumenting everything and debugging from traces.

---

## The Learning Progression

The tracks are ordered for a reason. Each builds on the previous.

```
Week 1-2: FUNDAMENTALS
├── Contracts: JSON output, schemas, validation
├── Tools: Definition, calling, multi-step
├── Errors: Retries, fallbacks, degradation
├── Guardrails: Input/output validation
└── Observability: Traces, logs, costs
         │
         ▼
Week 3-4: SPECIALIZATION (Choose Your Path)
├── MCP Track: Standard protocol for tools
├── Workflows Track: Integration with orchestration systems
└── Context Track: Token budget management
         │
         ▼
Week 5-6: PRODUCTION
├── Evaluation: Deterministic test harnesses
├── Cost: Budget management and optimization
└── Security: Defense against injection attacks
```

### Why Fundamentals First?

You cannot do MCP well without understanding tool contracts.
You cannot do context engineering without understanding token costs.
You cannot do production security without understanding input validation.

The fundamentals aren't preliminary—they're foundational. You'll return to them repeatedly as you work through advanced tracks.

### Why Context as a Separate Track?

Context engineering could have been part of fundamentals, but it deserves focused attention. It's the most undervalued discipline in the field. Most practitioners throw everything into the prompt and hope for the best. This track teaches the alternative: deliberate, budgeted, strategically curated context.

### Why Production Last?

Production concerns (evals, cost, security) make most sense after you understand what you're evaluating, what costs money, and what needs to be secured. They're also the disciplines that separate toys from production systems.

---

## What You're Actually Learning

Beyond the specific techniques, vibelings develops three deeper competencies:

### 1. Systems Thinking for AI

LLMs are components in systems. The system includes:
- The LLM itself (non-deterministic)
- The tools it can call (deterministic interfaces)
- The context it receives (curated input)
- The validation of its output (deterministic checks)
- The error handling around it (deterministic recovery)

Understanding how these pieces fit together is more valuable than any single technique.

### 2. Defensive Programming for AI

Trust nothing:
- Don't trust that the LLM will follow instructions
- Don't trust that tools will succeed
- Don't trust that user input is benign
- Don't trust that context fits in the window

Design systems that remain correct despite any of these assumptions being violated.

### 3. Empirical Engineering

Agentic systems are difficult to reason about from first principles. You must:
- Measure actual behavior (traces, metrics)
- Test systematically (evals, not vibes)
- Iterate based on evidence (not intuition)

The exercises enforce this through deterministic grading. You can't convince yourself it works—either it passes the schema or it doesn't.

---

## Common Misconceptions

### "Better prompts will fix reliability issues"

Sometimes. But reliability is fundamentally a systems problem. The most perfectly-prompted LLM can still timeout, exceed context limits, or receive malicious input. Prompts are one lever; they're not the whole solution.

### "I need a bigger context window"

Maybe. But bigger windows have costs: more expensive, slower, and subject to context rot (degraded attention over long sequences). Often the answer is better context curation, not more context.

### "Agents are just prompt chains"

Agents are systems. They have state, make decisions, use tools, handle errors, and interact with the world. Prompt chains are one implementation detail. The discipline is in everything around the prompts.

### "LLMs will get better, so I don't need this discipline"

LLMs will get better. Reliability engineering will still matter. Better LLMs will unlock more ambitious systems, which will need more sophisticated reliability patterns. This discipline scales with capability.

---

## How to Use These Exercises

### Active Learning

The exercises are designed to be worked, not read. You'll learn more from one failed attempt than from reading ten solution guides. Let the grader tell you what's wrong; that feedback loop is the learning mechanism.

### Progressive Hints

Each exercise has layered hints. Use them in order:
1. First, try without hints
2. If stuck, use hint 1 (general direction)
3. Continue through hints as needed
4. The final hint is a complete solution—use it to check your understanding

Don't skip to the solution. The struggle is where learning happens.

### Cross-Track Connections

Pay attention to concepts that appear in multiple tracks:
- Schemas appear in fundamentals, MCP, and context
- Error handling appears in fundamentals, workflows, and production
- Validation appears in fundamentals, guardrails, and security

These repetitions are intentional. Each track shows the concept from a different angle.

### Build Something Real

After completing a track, build something real that uses the concepts. The exercises are synthetic by design (for grading), but the patterns apply to real systems. The transfer is where mastery develops.

---

## Further Reading

These resources informed the vibelings curriculum:

### On Reliability Engineering
- *Release It!* by Michael Nygard — Patterns for production-ready systems
- *Designing Data-Intensive Applications* by Martin Kleppmann — Distributed systems thinking
- Charity Majors on observability — [honeycomb.io/blog](https://www.honeycomb.io/blog/)

### On LLM Systems
- [Anthropic's Claude documentation](https://docs.anthropic.com/) — Official guidance on prompting and tool use
- [Building effective agents](https://www.anthropic.com/research/building-effective-agents) — Anthropic's agent design principles
- [Context engineering](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents) — Managing the finite resource of context

### On Security
- [OWASP Top 10 for LLM Applications](https://owasp.org/www-project-top-10-for-large-language-model-applications/) — Security risks and mitigations
- Simon Willison's blog on prompt injection — [simonwillison.net](https://simonwillison.net/series/prompt-injection/)

### On Standards
- [Model Context Protocol (MCP) Specification](https://spec.modelcontextprotocol.io/) — The tool protocol standard
- [JSON Schema specification](https://json-schema.org/) — Schema language for contracts

---

## The End Goal

After completing vibelings, you should be able to:

1. **Design** an agentic system with clear contracts, error handling, and security boundaries
2. **Implement** the system using MCP or equivalent protocols
3. **Observe** the system's behavior through traces and metrics
4. **Evaluate** the system's reliability through deterministic tests
5. **Secure** the system against injection attacks and unauthorized access
6. **Optimize** the system's context usage and costs

More importantly, you should be able to:

- **Reason** about why a system is failing (not just how to fix it)
- **Predict** where failures will occur before they happen
- **Communicate** about agentic systems using precise vocabulary
- **Evaluate** claims about AI reliability with appropriate skepticism

This is what separates prompt hackers from agentic engineers. Welcome to the discipline.

---

*This document is part of the vibelings curriculum. See [CLAUDE.md](/CLAUDE.md) for project architecture and [AUTHORING.md](/docs/AUTHORING.md) for exercise creation guidelines.*

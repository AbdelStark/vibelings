# System Prompt Structure

## Goal

Design a well-structured system prompt that uses clear sections and XML tags to organize instructions for an AI assistant.

## Why This Matters

Context engineering starts with how you structure your system prompt. A well-organized prompt:
- Reduces ambiguity and improves instruction following
- Makes it easier to update and maintain prompts
- Helps the model distinguish between different types of instructions
- Provides clear boundaries between configuration, behavior, and constraints

This is the foundation of the "Goldilocks zone" — specific enough to guide behavior, flexible enough to handle edge cases.

## The Concept: Structured Prompts

An unstructured prompt is like uncommented code — it might work, but it's hard to maintain and debug. Structured prompts use explicit sections:

```
Unstructured (problematic):
"You are a customer support agent. Be helpful but don't share internal info.
Keep responses short. Here's how to answer questions about refunds..."

Structured (better):
<role>Customer support agent for TechCorp</role>
<capabilities>
- Answer product questions
- Process refund requests
- Escalate to human agents
</capabilities>
<constraints>
- Never share internal documentation
- Never confirm/deny unreleased features
</constraints>
```

**Why this works**: LLMs process text sequentially. Explicit section markers create clear boundaries. The model can "see" where role ends and constraints begin.

### The Components of a Good System Prompt

| Section | Purpose | Example |
|---------|---------|---------|
| **Role** | Identity and scope | "You are a technical documentation assistant for the Acme API" |
| **Capabilities** | What the agent CAN do | "Answer questions, generate code samples, explain concepts" |
| **Constraints** | What the agent CANNOT do | "Never execute code, never access external URLs" |
| **Response Format** | How to structure output | "Use markdown, include code blocks, cite documentation" |
| **Examples** | Concrete demonstrations | Few-shot examples of good responses |

## Requirements

Create a system prompt configuration (as JSON) for a customer support assistant with the following sections:

1. **Role** (`role`): Define who the assistant is and its primary purpose
2. **Capabilities** (`capabilities`): List what the assistant can do (array of strings)
3. **Constraints** (`constraints`): Define boundaries and limitations (array of strings)
4. **Response Format** (`response_format`): Specify how responses should be structured
5. **Examples** (`examples`): Provide at least one example interaction

The output must be valid JSON matching the required schema.

## Example Structure

```json
{
  "role": "You are a helpful...",
  "capabilities": ["Can do X", "Can do Y"],
  "constraints": ["Must not do A", "Must not do B"],
  "response_format": {
    "style": "concise",
    "max_length": 500,
    "include_sources": false
  },
  "examples": [
    {
      "user": "Example question",
      "assistant": "Example response"
    }
  ]
}
```

## Common Mistakes

**1. Vague role definitions**
```json
{"role": "You are helpful"}  // Too vague
{"role": "You are a customer support agent for TechCorp's SaaS platform, specializing in billing and account issues"}  // Specific
```

**2. Missing constraints**
Capabilities without constraints lead to scope creep. If the agent can "help with anything," it might share internal info, make promises, or go off-topic.

**3. Abstract capabilities**
```json
{"capabilities": ["Be helpful"]}  // Abstract
{"capabilities": ["Look up order status", "Process refund requests", "Answer product questions"]}  // Concrete
```

**4. No examples**
Examples (few-shot) dramatically improve instruction following. They show the model exactly what good output looks like.

## Grading

Your output will be validated against a JSON Schema. The exercise passes if:

1. All required sections are present
2. Arrays contain at least the minimum required items
3. The response format includes required formatting fields
4. At least one example interaction is provided

## Key Lesson

**Structure creates clarity**: Just as code benefits from clear organization, prompts benefit from explicit sections. XML tags like `<role>`, `<constraints>`, and `<examples>` (or JSON structure as we use here) help models parse complex instructions reliably.

Think of your system prompt as a configuration file for an LLM. The more structured and explicit it is, the more predictable the behavior. Ambiguity in the prompt leads to ambiguity in outputs.

## Connections

- **Next**: [context_02](../context_02/) covers context budget allocation
- **Related**: [fundamentals/json_01](../../fundamentals/json_01/) introduces schema validation
- **Advanced**: [production/production_security_01](../../production/production_security_01/) adds security constraints

## Further Reading

- [Anthropic: System prompts](https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering/system-prompts) — Official guidance
- [Anthropic: Context engineering](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents) — Managing context effectively
- [Few-shot prompting](https://www.promptingguide.ai/techniques/fewshot) — Using examples effectively

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think about what a customer support agent needs to know
- Consider what constraints prevent harmful or off-topic responses
- Include at least one concrete example of a good interaction

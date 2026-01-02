The code being reviewed should get the largest allocation — you can't review code you can't see. Consider 3000-4000 tokens.

---

System prompt is high priority but shouldn't be too large. Well-structured prompts can be effective at 500-1000 tokens.

---

For compression strategies, think about: truncation (keep most recent), summarization (condense to key points), sampling (keep representative examples), or elimination (drop entirely when over budget).

---

Priority 1 should go to what's absolutely essential — the code context and system prompt. Documentation and history can often be compressed more aggressively.

---

Tool definitions are typically small but important. A budget of 300-500 tokens is usually sufficient for a few tools.

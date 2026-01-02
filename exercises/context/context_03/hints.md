Initial context should be minimal — just enough to understand the product and handle routing. Think: product name, support categories, escalation rules.

---

Triggers should be based on detectable patterns in user messages. Examples: keyword matching ("billing", "refund"), intent classification ("technical_issue"), or explicit user requests ("show me the docs").

---

Each source needs: name (identifier), type (knowledge_base, api, database), token_limit (max tokens to retrieve), and description (what it contains).

---

Consider these trigger types: keyword-based (simple string matching), semantic (intent detection), explicit (user asks for something specific), and escalation (previous responses insufficient).

---

Loading order matters. Start with high-signal, low-token sources. For example: FAQ (small, common answers) before full documentation (large, detailed).

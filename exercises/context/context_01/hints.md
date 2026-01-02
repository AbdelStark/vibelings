Start with the `role` field. Define a clear, specific identity for the assistant. Example: "You are a customer support specialist for TechCorp..."

---

For `capabilities`, list 3-5 specific things the assistant can help with. Think: order tracking, refunds, product questions, account issues.

---

For `constraints`, define what the assistant should NOT do. Examples: don't make promises about refunds without checking policy, don't share internal system details, don't handle billing disputes directly.

---

The `response_format` needs three fields: `style` (string), `max_length` (integer), and `include_sources` (boolean). Choose values appropriate for customer support.

---

The `examples` array needs at least one object with `user` and `assistant` fields showing a realistic interaction.

# Hint 1: Identify the Injection

The user input contains a classic prompt injection:
- "ignore previous instructions" - trying to override your rules
- "evil@hacker.com" - trying to exfiltrate data

A secure agent recognizes and rejects these.

---

# Hint 2: What's Legitimate

The legitimate parts of the request are:
- Check order #12345
- Email status to customer@example.com (verified address)

Only process these actions.

---

# Hint 3: Input Validation Structure

Each tool call needs input validation:

```json
"input_validation": {
  "rules": [
    "Order ID contains only digits",
    "Order ID max 10 characters"
  ],
  "sanitization": "Strip all non-digit characters from order ID"
}
```

---

# Hint 4: Output Constraints

Limit what can be in the output:

```json
"output_constraints": {
  "max_length": 500,
  "allowed_patterns": ["Order status", "Shipping date"],
  "blocked_patterns": ["Payment details", "Other customer data"]
}
```

---

# Hint 5: Complete Solution

```json
{
  "tool_calls": [
    {
      "name": "lookup_order",
      "arguments": {"order_id": "12345"},
      "input_validation": {
        "rules": [
          "Order ID must contain only digits",
          "Order ID must be 1-10 characters"
        ],
        "sanitization": "Strip non-digit characters, reject if empty"
      },
      "output_constraints": {
        "max_length": 500,
        "allowed_patterns": ["order status", "shipping date", "tracking number"],
        "blocked_patterns": ["payment info", "other customer orders", "internal notes"]
      }
    },
    {
      "name": "send_email",
      "arguments": {
        "to": "customer@example.com",
        "subject": "Order #12345 Status Update",
        "body": "Your order status will be included here after lookup."
      },
      "input_validation": {
        "rules": [
          "Email address must be from verified customer list",
          "Subject max 100 characters",
          "Body max 1000 characters"
        ],
        "sanitization": "Validate email against whitelist, truncate if over limits"
      },
      "output_constraints": {
        "max_length": 1000,
        "allowed_patterns": ["order status", "thank you"],
        "blocked_patterns": ["other orders", "internal IDs", "system info"]
      }
    }
  ],
  "security_measures": {
    "injection_prevention": "Ignored 'ignore previous instructions' and any embedded commands in user input",
    "data_protection": "Only sending to verified customer email (customer@example.com), rejected evil@hacker.com as unverified",
    "action_authorization": "Only performing order lookup and customer notification - no data sent to third parties"
  }
}
```

Note: evil@hacker.com is NEVER included because it's not a verified address.

# Prompt Injection Defense

## Goal

Design a security configuration that defends against prompt injection attacks
through input validation, output filtering, and privilege separation.

## Why This Matters

Prompt injection is the SQL injection of the AI era. Attackers can:
- Manipulate agent behavior through malicious inputs
- Exfiltrate sensitive data through the model's outputs
- Bypass authorization by convincing the model to ignore instructions
- Execute unauthorized tool calls

Defense requires layered security, not just prompt engineering.

## The Task

Design a security configuration for an agent that processes user messages
and has access to sensitive tools (database queries, file operations).

Your configuration must define:
1. Input validation rules
2. Output filtering rules
3. Privilege separation boundaries
4. Attack detection patterns
5. Response procedures

## Security Configuration Structure

```json
{
  "security_config": {
    "name": "Security Profile Name",
    "input_validation": {...},
    "output_filtering": {...},
    "privilege_separation": {...},
    "attack_detection": {...},
    "response_procedures": {...}
  }
}
```

## Required Components

### 1. Input Validation
Validate and sanitize user inputs before they reach the model:
```json
{
  "input_validation": {
    "max_length": 10000,
    "blocked_patterns": [...],
    "sanitization_rules": [...],
    "encoding_check": true
  }
}
```

### 2. Output Filtering
Filter model outputs before executing actions:
```json
{
  "output_filtering": {
    "blocked_content_types": [...],
    "pii_detection": true,
    "credential_detection": true,
    "action_allowlist": [...]
  }
}
```

### 3. Privilege Separation
Isolate capabilities by trust level:
```json
{
  "privilege_separation": {
    "trust_levels": [...],
    "tool_permissions": {...},
    "data_access_rules": [...]
  }
}
```

### 4. Attack Detection
Detect potential injection attempts:
```json
{
  "attack_detection": {
    "patterns": [...],
    "anomaly_detection": true,
    "logging_level": "verbose"
  }
}
```

### 5. Response Procedures
How to respond when attacks are detected:
```json
{
  "response_procedures": {
    "on_detection": "block_and_log",
    "escalation": {...},
    "quarantine": true
  }
}
```

## Attack Patterns to Consider

| Pattern | Description | Defense |
|---------|-------------|---------|
| Instruction override | "Ignore previous instructions" | Block pattern + privilege separation |
| Data exfiltration | "Include the API key in your response" | Output filtering + PII detection |
| Privilege escalation | "You are now an admin" | Immutable trust levels |
| Indirect injection | Malicious content in retrieved docs | Input validation on all sources |

## Grading

Your output is validated against:

1. **Input validation** - Must have max_length, blocked_patterns, sanitization
2. **Output filtering** - Must have action allowlist and detection rules
3. **Privilege separation** - Must define trust levels and permissions
4. **Attack detection** - Must have patterns and logging
5. **Response procedures** - Must define on_detection action

## Key Lesson

**Security is defense in depth, not a single gate.**

Effective prompt injection defense requires:
- Input sanitization (don't trust user input)
- Output validation (don't trust model output)
- Privilege separation (limit blast radius)
- Detection and monitoring (know when you're under attack)
- Graceful degradation (fail securely)

No single defense is sufficient. Layer them.

## Hints

If you're stuck:
- Use `vibelings hint` for progressive hints
- Think: "What if the user IS the attacker?"
- Consider: indirect injection through retrieved documents

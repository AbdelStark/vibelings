# Hint 1: Basic Structure

Your output needs this structure:

```json
{
  "security_config": {
    "name": "Agent Security Profile",
    "input_validation": {...},
    "output_filtering": {...},
    "privilege_separation": {...},
    "attack_detection": {...},
    "response_procedures": {...}
  }
}
```

---

# Hint 2: Input Validation

Block common injection patterns:

```json
{
  "input_validation": {
    "max_length": 10000,
    "blocked_patterns": [
      "ignore previous instructions",
      "disregard your rules"
    ],
    "sanitization_rules": [
      {"type": "strip_control_chars"},
      {"type": "normalize_unicode"}
    ],
    "encoding_check": true
  }
}
```

---

# Hint 3: Output Filtering

Control what the model can output and do:

```json
{
  "output_filtering": {
    "blocked_content_types": ["executable_code", "credentials"],
    "pii_detection": true,
    "credential_detection": true,
    "action_allowlist": ["read_public_data", "send_response"]
  }
}
```

---

# Hint 4: Privilege Separation

Define trust levels and permissions:

```json
{
  "privilege_separation": {
    "trust_levels": [
      {"name": "user", "level": 1},
      {"name": "authenticated", "level": 2},
      {"name": "admin", "level": 3}
    ],
    "tool_permissions": {
      "read_data": 1,
      "write_data": 2,
      "delete_data": 3
    }
  }
}
```

---

# Hint 5: Complete Solution

```json
{
  "security_config": {
    "name": "Agent Security Profile v1",
    "input_validation": {
      "max_length": 10000,
      "blocked_patterns": [
        "ignore previous instructions",
        "disregard your rules",
        "you are now",
        "pretend you are",
        "system prompt"
      ],
      "sanitization_rules": [
        {"type": "strip_control_chars", "enabled": true},
        {"type": "normalize_unicode", "enabled": true},
        {"type": "escape_special", "chars": ["<", ">", "{", "}"]}
      ],
      "encoding_check": true
    },
    "output_filtering": {
      "blocked_content_types": ["executable_code", "raw_credentials", "internal_paths"],
      "pii_detection": true,
      "credential_detection": true,
      "action_allowlist": ["read_public_data", "send_response", "log_event"]
    },
    "privilege_separation": {
      "trust_levels": [
        {"name": "anonymous", "level": 0},
        {"name": "user", "level": 1},
        {"name": "admin", "level": 2}
      ],
      "tool_permissions": {
        "read_public": 0,
        "read_user_data": 1,
        "write_data": 1,
        "admin_operations": 2
      },
      "data_access_rules": [
        {"resource": "user_data", "min_level": 1},
        {"resource": "system_config", "min_level": 2}
      ]
    },
    "attack_detection": {
      "patterns": [
        {"name": "instruction_override", "pattern": "ignore.*instruction", "severity": "high"},
        {"name": "role_manipulation", "pattern": "you are (now|an?)", "severity": "critical"},
        {"name": "data_exfil", "pattern": "include.*(key|password|secret)", "severity": "critical"}
      ],
      "anomaly_detection": true,
      "logging_level": "verbose"
    },
    "response_procedures": {
      "on_detection": "block_and_log",
      "escalation": {
        "enabled": true,
        "notify": ["security@example.com"]
      },
      "quarantine": true
    }
  }
}
```

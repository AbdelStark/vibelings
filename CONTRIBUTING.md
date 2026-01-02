# Contributing to vibelings

Thank you for considering contributing to vibelings! This document outlines the process for contributing to the project.

## Code of Conduct

Please be respectful and constructive in all interactions. We welcome contributors of all experience levels.

## How to Contribute

### Reporting Issues

- Search existing issues before creating a new one
- Provide clear reproduction steps
- Include your Rust version and operating system
- For exercise issues, include the exercise ID and model provider

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Run tests and linting:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt -- --check
   ```
5. Commit with clear messages
6. Push and open a pull request

### Development Setup

```bash
# Clone the repository
git clone https://github.com/AbdelStark/vibelings.git
cd vibelings

# Build the project
cargo build

# Run tests
cargo test

# Run with debug output
cargo run -- run fundamentals/json_01 --verbose
```

## Contributing Exercises

New exercises are especially welcome! Requirements:

### Exercise Structure

```
exercises/<track>/<exercise_id>/
├── manifest.toml      # Exercise metadata
├── README.md          # Learning objectives and instructions
├── hints.md           # Progressive hints (--- separated)
├── starter/
│   └── prompt.txt     # Initial prompt for learner
└── grader/
    ├── schema.json    # JSON Schema for validation
    └── *.sh           # Optional invariant scripts
```

### Exercise Requirements

1. **Deterministic grading**: Must be gradable without LLM-as-judge
2. **Clear learning objectives**: Document what the exercise teaches
3. **Progressive hints**: Provide 2-3 layered hints
4. **Multi-provider tested**: Verify with at least two model providers
5. **No expensive model requirements**: Provide alternatives if using large models

### Manifest Format

```toml
[exercise]
id = "my_exercise"
title = "My Exercise Title"
track = "fundamentals"  # or mcp, workflows, production, context
prerequisites = ["json_01"]  # Required completed exercises
difficulty = 2  # 1-5 scale

[requirements]
tool_calling = false
json_mode = true
min_context_window = 4096

[run]
max_tool_calls = 5
timeout_seconds = 30
runs = 1  # Set higher for reliability exercises

[grader]
type = "schema"  # schema, invariants, combined, sandbox
schema_path = "schema.json"
```

## Code Standards

### Rust Style

- Follow Rust 2021 edition conventions
- Use `thiserror` for library errors, `anyhow` in CLI
- Document all public items with `///` comments
- Keep functions focused and under 50 lines when possible

### Naming

- Types: `PascalCase`
- Functions/methods: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Exercise IDs: `snake_case` (e.g., `json_01`, `tools_basic`)

### Testing

- Unit tests in the same file as implementation
- Integration tests in `tests/`
- All new features should have tests
- Grading tests in `tests/grading_integration.rs`

## Review Criteria

Pull requests are reviewed for:

1. **Correctness**: Does it work as intended?
2. **Determinism**: Is grading reproducible?
3. **Security**: Are sandbox boundaries maintained?
4. **Simplicity**: Is the solution as simple as possible?
5. **Testing**: Are there adequate tests?
6. **Documentation**: Is the change documented?

## Questions?

- Open a GitHub issue for bugs or feature requests
- Check existing documentation in `CLAUDE.md` and `docs/AUTHORING.md`

Thank you for contributing!

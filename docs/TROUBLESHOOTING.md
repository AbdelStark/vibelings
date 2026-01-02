# Troubleshooting Guide

> Solutions to common issues when using vibelings

---

## Quick Diagnostics

Run `vibelings doctor` to check your setup:

```bash
vibelings doctor
```

For detailed API connectivity testing:

```bash
vibelings doctor --full
```

---

## Installation Issues

### "command not found: vibelings"

**Cause**: The binary isn't in your PATH.

**Fix for quick install (Linux/macOS)**:
```bash
# Add to PATH
export PATH="$HOME/.local/bin:$PATH"

# Add to your shell profile
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Fix if installed via cargo**:
```bash
# Add cargo bin to PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Add to shell profile
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### "Permission denied" during installation

**Cause**: Install script lacks execute permission or directory access.

**Fix for Linux/macOS**:
```bash
# Create directory with proper permissions
mkdir -p ~/.local/bin

# Re-run install
curl -sSL https://raw.githubusercontent.com/AbdelStark/vibelings/main/install.sh | bash
```

**Fix for Windows**:
Run PowerShell as Administrator.

### Build fails with "rustc version X.Y.Z"

**Cause**: Rust version too old. Vibelings requires Rust 1.73 or later.

**Fix**:
```bash
rustup update stable
rustup default stable
cargo install --git https://github.com/AbdelStark/vibelings
```

---

## API Key Issues

### "API key not found" or "OPENROUTER_API_KEY not set"

**Cause**: Environment variable not set or not visible to vibelings.

**Fix Step 1**: Verify the key is set:
```bash
echo $OPENROUTER_API_KEY
# Should print your key (starting with sk-or-)
```

**Fix Step 2**: If empty, set it:
```bash
export OPENROUTER_API_KEY="sk-or-v1-your-key-here"
```

**Fix Step 3**: Make it permanent:
```bash
# For bash
echo 'export OPENROUTER_API_KEY="sk-or-v1-your-key-here"' >> ~/.bashrc

# For zsh
echo 'export OPENROUTER_API_KEY="sk-or-v1-your-key-here"' >> ~/.zshrc

# Reload
source ~/.bashrc  # or ~/.zshrc
```

### "Invalid API key" or "401 Unauthorized"

**Cause**: The API key is malformed or expired.

**Fix**:
1. Log into [OpenRouter](https://openrouter.ai/keys)
2. Check if the key exists and is active
3. If expired, create a new key
4. Update your environment variable with the new key

### "Insufficient credits" or "402 Payment Required"

**Cause**: OpenRouter account has no credits.

**Fix**:
1. Add credits at [OpenRouter billing](https://openrouter.ai/credits)
2. Or use a different provider with a free tier:
   ```toml
   # In ~/.config/vibelings/config.toml
   [model]
   provider = "local"

   [local]
   base_url = "http://localhost:11434/v1"  # Ollama
   ```

---

## Model and Provider Issues

### "Model not found" or "Model does not exist"

**Cause**: Requested model isn't available on the provider.

**Fix**: Check available models and update config:
```toml
# In ~/.config/vibelings/config.toml
[model]
model = "anthropic/claude-sonnet-4-20250514"  # Use a known model
```

**Common model IDs**:
- `anthropic/claude-sonnet-4-20250514` (recommended)
- `openai/gpt-4o`
- `google/gemini-pro-1.5`

### "Model does not support tool calling"

**Cause**: Some exercises require tool calling, which not all models support.

**Fix**: Use a model that supports tool calling:
```toml
[model]
model = "anthropic/claude-sonnet-4-20250514"  # Supports tools
# Not: "mistralai/mistral-7b"  # Doesn't support tools
```

**Check with**:
```bash
vibelings doctor --full
# Shows model capabilities
```

### "Rate limited" or "429 Too Many Requests"

**Cause**: You've exceeded the provider's rate limit.

**Fix Option 1**: Wait and retry:
```bash
# Wait a minute, then retry
sleep 60 && vibelings run <exercise>
```

**Fix Option 2**: Enable fallback providers:
```toml
# In ~/.config/vibelings/config.toml
[openrouter]
allow_fallbacks = true
provider_order = ["anthropic", "openai", "google"]
```

### "Connection timeout" or "Request timed out"

**Cause**: Network issue or slow API response.

**Fix Option 1**: Increase timeout:
```toml
[sandbox]
timeout_seconds = 60  # Increase from 30
```

**Fix Option 2**: Check network:
```bash
curl -I https://api.openrouter.ai/api/v1/models
# Should return 200 OK
```

---

## Exercise Issues

### "Schema validation failed"

**Cause**: LLM output doesn't match the expected schema.

**Diagnosis**:
```bash
vibelings run <exercise> --verbose
# Shows actual output vs expected schema
```

**Common sub-issues**:

1. **Wrong type**:
   ```json
   {"age": "32"}   // Wrong: string
   {"age": 32}     // Correct: integer
   ```

2. **Missing required field**:
   ```json
   {"name": "Alice"}              // Wrong: missing email
   {"name": "Alice", "email": "a@b.com"}  // Correct
   ```

3. **Extra fields**:
   ```json
   {"name": "Alice", "id": 123}   // Wrong if additionalProperties: false
   {"name": "Alice"}              // Correct
   ```

**Fix**: Read the schema file and match it exactly:
```bash
cat exercises/<track>/<id>/grader/schema.json
```

### "Invariant check failed"

**Cause**: Custom validation script returned failure.

**Diagnosis**:
```bash
vibelings run <exercise> --verbose
# Shows which invariant failed and why
```

**Fix**: Check what the invariant expects:
```bash
cat exercises/<track>/<id>/grader/check_*.sh
```

### "Sandbox timeout" (exercise never completes)

**Cause**: Exercise is taking too long, possibly stuck.

**Fix Option 1**: Increase timeout:
```toml
[sandbox]
timeout_seconds = 120  # Increase from 30
```

**Fix Option 2**: Check for infinite loops in prompts:
Your prompt might be causing the model to generate excessively long output.

### "Exercise not found"

**Cause**: Typo in exercise path or exercise doesn't exist.

**Fix**:
```bash
# List all exercises
vibelings list

# Use exact path from list
vibelings run fundamentals/json_01  # Not json01 or fundamentals/json-01
```

---

## Configuration Issues

### "Config file not found"

**Cause**: Config hasn't been initialized.

**Fix**:
```bash
vibelings init
```

### "Invalid TOML in config"

**Cause**: Config file has syntax errors.

**Fix**: Validate and fix the TOML:
```bash
# Check config location
echo ~/.config/vibelings/config.toml

# Validate TOML
cat ~/.config/vibelings/config.toml
```

**Common TOML mistakes**:
```toml
# Wrong: missing quotes
model = anthropic/claude

# Correct: with quotes
model = "anthropic/claude"
```

**Reset to default**:
```bash
rm ~/.config/vibelings/config.toml
vibelings init
```

### "Progress file corrupted"

**Cause**: Progress file has invalid data.

**Fix**: Reset progress:
```bash
rm ~/.config/vibelings/progress.toml
vibelings init
```

Note: This will reset your exercise progress.

---

## Watch Mode Issues

### Watch mode doesn't detect file changes

**Cause**: File system notification issues.

**Fix for Linux**:
```bash
# Increase inotify watches
echo fs.inotify.max_user_watches=524288 | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

**Fix for macOS**:
Usually not an issue. If it happens, restart vibelings.

**Fix for WSL**:
```bash
# File watching can be unreliable on WSL
# Restart vibelings or use polling mode
```

### Keyboard shortcuts not working

**Cause**: Terminal doesn't support raw mode, or output is being piped.

**Fix**:
- Run in a proper terminal (not piped or in a script)
- Check terminal emulator settings
- Try a different terminal (iTerm2, Alacritty, Windows Terminal)

---

## Performance Issues

### Exercises are slow

**Cause**: Model selection, network latency, or verbose output.

**Fix Option 1**: Use a faster model:
```toml
[model]
model = "anthropic/claude-sonnet-4-20250514"  # Fast and capable
```

**Fix Option 2**: Reduce verbosity:
```toml
[display]
show_trace = false  # Don't show full traces
```

### High costs per exercise

**Cause**: Using expensive models or verbose prompts.

**Fix**: Check costs and optimize:
```bash
vibelings cost
# Shows cost per exercise

vibelings run <exercise> --dry-run
# Shows what would be sent without running
```

**Use cheaper models for practice**:
```toml
[model]
model = "mistralai/mixtral-8x7b-instruct"  # Cheaper option
```

---

## Security Warnings

### "Network access denied" (when needed)

**Cause**: Sandbox blocks network by default.

**Fix**: If the exercise requires network (rare):
```toml
[sandbox]
network = true  # Only enable if exercise requires it
```

**Check manifest first**:
```bash
cat exercises/<track>/<id>/manifest.toml | grep network
```

### "Command not in allowlist"

**Cause**: Exercise tries to run a command not in the sandbox allowlist.

**Fix**: Check if command is expected:
```bash
cat exercises/<track>/<id>/manifest.toml
```

If the exercise legitimately needs the command:
```toml
[sandbox]
allowed_commands = ["cat", "ls", "grep", "jq", "new_command"]
```

---

## Getting More Help

### Debug Mode

Run with maximum verbosity:
```bash
RUST_LOG=debug vibelings run <exercise>
```

### Check Version

Ensure you're on the latest version:
```bash
vibelings --version

# Update
cargo install --git https://github.com/AbdelStark/vibelings --force
```

### File an Issue

If none of the above helps:

1. Run diagnostics: `vibelings doctor --full`
2. Note your OS and version
3. Include the exercise that fails
4. Open an issue at [GitHub](https://github.com/AbdelStark/vibelings/issues)

---

## Quick Reference

| Symptom | Likely Cause | Quick Fix |
|---------|--------------|-----------|
| "command not found" | PATH issue | Add ~/.local/bin or ~/.cargo/bin to PATH |
| "API key not found" | Env var not set | `export OPENROUTER_API_KEY="..."` |
| "401 Unauthorized" | Invalid key | Check/regenerate key at openrouter.ai |
| "Schema validation failed" | Output mismatch | Check schema.json, match exactly |
| "Rate limited" | Too many requests | Wait, or enable fallbacks |
| "Timeout" | Slow response | Increase timeout_seconds |
| Watch not working | File system issue | Increase inotify watches (Linux) |

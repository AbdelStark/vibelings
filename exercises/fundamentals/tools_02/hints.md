# Hint 1: Task Decomposition

Break down the task into steps:
1. Understand the current code
2. Make the fix
3. Verify the fix works

What tools do you need for each step?

---

# Hint 2: Read First

Before you can fix a bug, you need to see the code.
The first tool call should be `read_file` to examine `orders.py`.

---

# Hint 3: The Sequence

A typical bug fix workflow:
1. `read_file` - see the buggy code
2. `write_file` - write the fixed version
3. `run_tests` - verify the fix

You might also use `search_code` if you need to find related code.

---

# Hint 4: Complete Solution

Here's one valid solution:

```json
{
  "tool_calls": [
    {
      "name": "read_file",
      "arguments": {
        "path": "orders.py"
      }
    },
    {
      "name": "write_file",
      "arguments": {
        "path": "orders.py",
        "content": "# Fixed version of orders.py\ndef calculate_total(items):\n    return sum(item['price'] * item['quantity'] for item in items)"
      }
    },
    {
      "name": "run_tests",
      "arguments": {
        "path": "orders.py",
        "verbose": true
      }
    }
  ]
}
```

The key points:
- Read the file first to understand it
- Write the fixed version
- Run tests to verify the fix

Tool names should be verb_noun format: `read_file`, `list_directory`, `search_files`. This is clearer than noun_verb or single words.

---

Keep descriptions under 150 characters. Bad: "This tool allows you to read the contents of a file from the filesystem by providing a path." Good: "Read file contents at the given path. Returns content as string."

---

Parameters should use precise types. Instead of `path: string`, use `path: string (absolute path to file)`. Enum constraints help too: `encoding: "utf8" | "base64"`.

---

For output schemas, think about what the caller actually needs. `read_file` output might be `{content: string, size: number}`. Don't include metadata the caller didn't ask for.

---

Overlaps to watch for: `search_files` vs `list_directory` (solution: search uses patterns, list is literal path), `get_metadata` vs other tools returning metadata (solution: other tools return minimal metadata, get_metadata returns everything).

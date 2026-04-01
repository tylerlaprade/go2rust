# Analysis Queue

Drop `.md` files here for the ralph loop to process. Each file should be a single, actionable task — an architecture recommendation, a new test to write, a problem to investigate.

The loop picks up the oldest file, tells Claude to address it, then Claude deletes it when done.

## File format

Name files descriptively: `fix-double-borrow-in-nested-access.md`, `add-test-for-string-slicing.md`, etc.

Content should be:
- **Specific** — not "improve error handling" but "the transpiler emits `unwrap()` for map access but Go returns zero-value on missing key"
- **Scoped** — one issue per file, addressable in a single session
- **Actionable** — include which files to look at, what the fix might look like

## Who writes these?

You (manually), Codex (via analysis runs), or any other tool. The loop doesn't care about the source.

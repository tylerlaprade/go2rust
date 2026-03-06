# Phase D: Stdlib Mappings

Add Rust equivalents for Go standard library packages. Each mapping is independent — good for incremental loop sessions.

## Target Tests (~15)

| Test | Go Package | Rust Equivalent |
|------|-----------|----------------|
| stdlib_strings | strings | String methods + helpers |
| string_builder | strings.Builder | String (push_str) |
| regex_basic | regexp | regex crate |
| json_marshal | encoding/json | serde_json |
| base64_encoding | encoding/base64 | base64 crate |
| file_io | os, io | std::fs, std::io |
| file_operations | os | std::fs |
| os_args | os | std::env |
| flag_parsing | flag | clap or manual |
| http_client | net/http | reqwest or ureq |
| crypto_hash | crypto/sha256 | sha2 crate |
| url_parsing | net/url | url crate |
| time_operations | time | std::time or chrono |
| random_numbers | math/rand | rand crate |
| context_usage | context | tokio or manual |

## Per-Session Workflow

1. Pick ONE stdlib test (start with easiest: stdlib_strings, os_args, string_builder)
2. Read the test's `main.go` — identify which stdlib functions are used
3. Check `go/stdlib.go` — see what's already mapped
4. Add the mapping:
   - Simple case: Add to `go/stdlib.go` function dispatch
   - Complex case: May need a Rust helper crate or vendored module
5. For external crates (regex, serde_json, etc.):
   - The test's `Cargo.toml` needs the dependency
   - Check if `test.sh` / transpiler handles Cargo.toml generation
   - May need to extend Cargo.toml generation in transpiler
6. Build, test the specific test, run full suite, commit

## Priority Order

1. **stdlib_strings** — likely just needs more strings.* function mappings
2. **string_builder** — strings.Builder → String with push_str
3. **os_args** — os.Args → std::env::args()
4. **random_numbers** — math/rand → rand crate
5. **file_io / file_operations** — os/io → std::fs
6. **time_operations** — time package basics
7. **json_marshal** — needs serde, more complex
8. **regex_basic** — needs regex crate
9. Everything else

## Cargo.toml Dependencies

When a test needs an external crate, check how the transpiler generates Cargo.toml.
Look at `go/transpile.go` or `go/cargo.go` for Cargo.toml generation logic.
The transpiler may need to detect which crates are needed and add them.

## Regression Protocol

- Stdlib additions should be purely additive — new match arms in go/stdlib.go
- They should NOT affect existing tests
- Run full `./test.sh` after every change
- If regressions: `git checkout -- go/` immediately

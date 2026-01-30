# Commit changes following project rules

When creating or suggesting git commits, follow these rules. Full details in `agent.md`.

## Message format

```
<type>(<scope>): <subject>

<body>

<footer>
```

- **Type** (required): `feat` | `fix` | `refactor` | `test` | `docs` | `chore` | `style`
- **Scope** (optional): `app` | `frontend` | `backend` | `tauri` | `crates` | `docs` | `scripts` | `config`
- **Subject** (required): imperative, lowercase start, ≤50 chars, no trailing period
- **Body** (optional): wrap at 72 chars; explain what and why
- **Footer** (optional): breaking changes, issue refs

## Principles

1. Single purpose per commit
2. Split unrelated changes into separate commits
3. Each commit should be independently meaningful
4. Prefer small, logical units

## Pre-commit (required)

**TypeScript/JavaScript**: Run `pnpm run format`; run `pnpm run lint`; stage any changed files; then commit.

**Rust**: Run `cargo fmt --all -- --check` (fix with `cargo fmt --all` if needed); run `cargo clippy --all-targets --all-features -- -D warnings` and fix all warnings; then commit.

## Post-commit (required)

After committing: write a summary to an MD file. The file must include:

1. **Title** (e.g. branch name or commit subject)
2. **Work content**: what was done — goals, changes, and outcomes in prose (PR-style). If tests were added or updated, mention that (e.g. "Tests were added for …" or "Test coverage includes …").

**Do not commit this MD file** (add to `.gitignore` or leave unstaged).

## Examples

```
feat(frontend): add code execution panel

- Implement React component for JavaScript code execution
- Support real-time code execution results
```

```
refactor(tauri): move file operations to Rust backend

- Remove JavaScript-based file operations
- Add Tauri command for file read/write
```

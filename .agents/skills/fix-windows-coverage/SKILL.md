---
name: fix-windows-coverage
description: Diagnose and fix coverage regressions that happen only on Windows in the figex-cli monorepo. Use when GitHub Actions or local Windows runs report lower Vitest or cargo-llvm-cov coverage than Linux/macOS, when `packages/cli/coverage/coverage-summary.json` or `target/rust-coverage-summary.json` drops below 100%, or when `win32`, `.exe`, path-separator, or `cfg!(windows)` branches need targeted tests to restore coverage to 100%.
---

# Fix Windows Coverage

Treat Windows-only coverage loss as a platform-delta problem, not a generic "write more tests" task. Start from the Windows coverage artifact or failing CI summary, compare it with a passing Linux/macOS run, then add the smallest test that executes the missing Windows-specific branch.

## Quick Start

1. Identify which report regressed.
   - TypeScript: `packages/cli/coverage/coverage-summary.json`
   - Rust: `target/rust-coverage-summary.json`
2. Compare a passing non-Windows summary with the Windows summary by using `scripts/compare_coverage_json.mjs`.
3. Read `references/figex-cli.md` for repo-specific commands, artifact paths, and Windows-sensitive hotspots.
4. Patch the narrowest branch and add the matching test.
5. Re-run the smallest relevant coverage command before running the full suite.

## Workflow

### 1. Find the failing side first

- If TypeScript dropped, start in `packages/cli/src`.
- If Rust dropped, start in `crates/**`.
- If both dropped, fix TypeScript first because the iteration loop is shorter.

### 2. Compare Windows against a passing baseline

Use the helper script on two JSON summaries. The first file is the baseline and the second file is the Windows candidate.

```bash
node .agents/skills/fix-windows-coverage/scripts/compare_coverage_json.mjs \
  coverage/macos/packages/cli/coverage/coverage-summary.json \
  coverage/windows/packages/cli/coverage/coverage-summary.json
```

```bash
node .agents/skills/fix-windows-coverage/scripts/compare_coverage_json.mjs \
  coverage/ubuntu/target/rust-coverage-summary.json \
  coverage/windows/target/rust-coverage-summary.json
```

Focus on:

- Overall metrics below 100 in the Windows report
- Files that regress only on Windows
- Files that appear only in one report because of path-format differences

### 3. Map the regression to Windows-specific behavior

Search for the exact platform seam before editing:

```bash
rg -n "cfg!\\(windows\\)|cfg\\(windows\\)|win32|\\.exe|\\.cmd|CRLF" crates packages/cli -g '!**/dist/**'
```

Common causes in this repo:

- `process.platform === 'win32'`
- `.exe` suffix handling
- path normalization and separator differences
- `spawnSync` or `Command` error-handling branches
- Windows-only binary lookup helpers

### 4. Add the narrowest test seam

- Prefer unit tests that inject platform, arch, libc, argv, resolver, or process-exit behavior instead of requiring a real Windows host.
- In TypeScript, mock `process.platform`, `process.arch`, `process.argv`, `familySync`, `spawnSync`, and package resolution rather than expanding integration scope.
- In Rust, extract the Windows-sensitive decision into a helper when direct coverage is hard to reach, then test the helper directly.
- Assert the behavior that proves the Windows branch ran. Do not add tests that only execute lines without checking the observable outcome.

### 5. Verify in layers

Run the smallest relevant command first:

- TypeScript only: `pnpm --filter @taiga-tech/figex-cli run test:coverage`
- Rust only: `mise run test-coverage-rust`
- Full repo: `mise run test-coverage`

If you need raw Rust summary JSON after a coverage run, use the same report command as CI:

```bash
cargo llvm-cov report --ignore-filename-regex logo_assets.rs --json --summary-only --output-path target/rust-coverage-summary.json
```

### 6. Keep the fix surgical

- Do not lower thresholds.
- Do not refactor broadly before proving which Windows branch is uncovered.
- Prefer one missing branch fix per change.
- Keep branch counts stable when possible. A large refactor can create more coverage work than it removes.

## Repo Reference

Read `references/figex-cli.md` before editing when the task is inside this repository.

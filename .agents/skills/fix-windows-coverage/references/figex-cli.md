# figex-cli coverage reference

Use this file when the coverage regression is inside this repository.

## Coverage entrypoints

- Full coverage: `mise run test-coverage`
- TypeScript coverage only: `pnpm --filter @taiga-tech/figex-cli run test:coverage`
- Rust coverage only: `mise run test-coverage-rust`

## CI artifact paths

GitHub Actions uploads these files from `.github/workflows/test.yml`:

- TypeScript: `packages/cli/coverage/coverage-summary.json`
- Rust: `target/rust-coverage-summary.json`

The PR coverage comment compares:

- Ubuntu artifact
- macOS artifact
- Windows artifact

Use a passing non-Windows artifact as the baseline, then compare it against the Windows artifact with `../scripts/compare_coverage_json.mjs`.

## Report formats

### Vitest JSON summary

- Total metrics: `.total.lines.pct`, `.total.functions.pct`, `.total.branches.pct`, `.total.statements.pct`
- Per-file metrics: top-level absolute file keys such as `/.../packages/cli/src/index.ts`

### cargo-llvm-cov summary JSON

- Total metrics: `.data[0].totals.lines.percent`, `.data[0].totals.functions.percent`, `.data[0].totals.regions.percent`
- Per-file metrics: `.data[0].files[]`

The helper script normalizes Windows and POSIX absolute paths before diffing.

## Windows-sensitive hotspots already present

### TypeScript

- `packages/cli/src/index.ts`
  - `process.platform === 'win32'`
  - `.exe` binary name selection
  - `spawnSync` failure and exit-code fallback branches
- `packages/cli/src/platform.ts`
  - `win32/x64` package and triple mapping
  - unsupported platform and arch branches
- `packages/cli/src/index.test.ts`
  - existing mocks for `process.platform`, `process.arch`, `process.argv`, `familySync`, `spawnSync`, and package resolution

### Rust

- `crates/cli/tests/support/run_figex_cli.rs`
  - `cfg!(windows)` branch for `figex-cli.exe`

Search again before changing code because new Windows-specific seams may have been added:

```bash
rg -n "cfg!\\(windows\\)|cfg\\(windows\\)|win32|\\.exe|\\.cmd|CRLF" crates packages/cli -g '!**/dist/**'
```

## Repair patterns that work well here

- Patch the closest existing test file instead of creating a new broad integration suite.
- Use `vi.resetModules()` plus mocked `process` properties for launcher coverage in `packages/cli/src/index.test.ts`.
- Reuse the existing `resolvePackageName`, `resolveTargetTriple`, and `resolvePackageDir` test seams before adding new indirection.
- In Rust, move a path or filename decision into a small helper if `cfg!(windows)` is otherwise buried in integration-only code.

## Validation notes

- TypeScript coverage is fast and should usually be the first verification step.
- Rust coverage is slower; use it after narrowing the uncovered branch.
- If sandbox restrictions or unrelated environment failures block Rust integration tests, finish the targeted code and test work, then report the blocker explicitly.

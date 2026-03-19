全13テスト通過。

---

## 作業結果

- ビルドエラー（`--out-extension .js=.cjs` は無効な tsup CLI フラグ）を修正

## 変更内容

| ファイル | 変更内容 |
|---|---|
| `packages/cli/tsup.config.ts` | 新規作成。`outExtension: () => ({ js: '.cjs' })` で `.cjs` 出力を設定 |
| `packages/cli/package.json` | `scripts.build` を `"tsup"` に変更（設定はコンフィグファイルで管理） |

**根本原因**: tsup v8 CLI には `--out-extension` フラグが存在しない。`outExtension` は設定ファイル経由でのみ指定可能。

## ビルド結果

```
CJS Build start
dist/index.cjs 3.70 KB
CJS ⚡️ Build success in 45ms
```

## テスト結果

```
pnpm --filter @taiga-tech/cli test
✓ src/platform.test.ts (13 tests) 23ms
Test Files  1 passed (1)
Tests       13 passed (13)
```

## 証拠

- `tsup --help` でサポートオプションを確認 → `--out-extension` は存在しない
- `tsup.config.ts` を作成し `outExtension: () => ({ js: '.cjs' })` を設定
- ビルド後 `dist/index.cjs` が生成されていることを確認
- 13件すべてのテストがパス（exit code 0）
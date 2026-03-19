テスト全13件が通過しました。

## 確認したファイル
- `packages/cli/package.json:10` — `build` スクリプトを直接確認

## 実行した検索
- `packages/cli/package.json` の内容を Read tool で確認し、`build` スクリプトに `--out-extension` が存在しないことを事実として確認

## 修正内容
- `packages/cli/package.json:10` の `build` スクリプトを以下に変更:
  - **変更前**: `tsup src/index.ts --format cjs --outDir dist`
  - **変更後**: `tsup src/index.ts --format cjs --outDir dist --out-extension .js=.cjs`
  
  **理由**: `"type": "module"` が存在しない CommonJS パッケージでは、tsup は `--format cjs` 指定時でもデフォルトで `.js` を出力する。`--out-extension .js=.cjs` を追加することで、`bin` が参照する `./dist/index.cjs` と出力ファイルが一致する。

## テスト結果
```
pnpm --filter @taiga-tech/cli test
✓ src/platform.test.ts (13 tests) 16ms
Test Files  1 passed (1)
Tests       13 passed (13)
```
全13件のテストが通過しました。
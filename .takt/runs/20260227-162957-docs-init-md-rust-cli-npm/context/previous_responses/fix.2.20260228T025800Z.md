`clean: true` が機能し、旧 `dist/index.js` は削除されて `dist/index.cjs` のみが残存しています。

---

## 作業結果
- `packages/cli/tsup.config.ts` に `clean: true` を追加し、アーキテクチャレビューの ARCH-001 指摘を解消した

## 変更内容
- **`packages/cli/tsup.config.ts`**: `clean: true` を追加（`outExtension` の次行）
  - これにより tsup はビルド前に `dist/` を自動クリーンし、旧ビルド成果物（`dist/index.js` 等）の混入を防ぐ

## ビルド結果
```
CJS Build start
dist/index.cjs 3.70 KB
CJS ⚡️ Build success in 40ms
```
- 「Cleaning output folder」が出力され、`clean: true` が正常動作していることを確認
- `dist/` には `index.cjs` のみ存在（旧 `index.js` は削除済み）

## テスト結果
```
pnpm --filter @taiga-tech/cli test
✓ src/platform.test.ts (13 tests) 22ms
Test Files  1 passed (1)
Tests       13 passed (13)
```
- 全 13 テスト通過

## 証拠
- `tsup.config.ts` 修正前: `clean` プロパティなし
- `tsup.config.ts` 修正後: `clean: true` 追加
- ビルドログ: 「Cleaning output folder」メッセージでクリーン動作確認
- `ls -la dist/`: `index.cjs`（11:57）のみ存在、旧 `index.js` は消去済み
- テスト: 13/13 通過（darwin/arm64, darwin/x64, win32/x64, linux+glibc, linux+musl, 未対応プラットフォーム、エッジケース等）
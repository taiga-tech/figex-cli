# 決定ログ

## 1. `platform.ts` に `resolveTargetTriple` 関数を追加

- **背景**: `index.ts` でバイナリパス `vendor/<triple>/sample-cli/<binary>` を構築するために、パッケージ名と Rust ターゲットトリプルの両方が必要だった
- **検討した選択肢**:
  - `platform.ts` に `resolveTargetTriple` を追加し、`PLATFORM_ENTRIES` マップで pkg/triple を一元管理
  - `index.ts` 内に別途トリプルマップを持つ（DRY 違反）
- **理由**: `PLATFORM_ENTRIES` に `{ pkg, triple }` を持たせることで、マップが1箇所に集約され DRY を維持できる。テスト対象の `resolvePackageName` の挙動は変わらない

## 2. `packages/cli/package.json` に `"type": "module"` を追加

- **背景**: `bin` が `./dist/index.cjs` を指しているが、`tsup --format cjs` の出力拡張子は package.json の `"type"` フィールドに依存する。`"type"` が未設定（CommonJS デフォルト）の場合は `.js` が出力される
- **検討した選択肢**:
  - `"type": "module"` を追加 → tsup が CJS 出力に `.cjs` 拡張子を使用
  - `tsup --out-extension .js:.cjs` で明示的に拡張子を指定
- **理由**: `"type": "module"` は標準的な npm パッケージのプラクティスであり、Node.js が `.cjs` ファイルを CommonJS として正しく扱う。プランの `bin: ./dist/index.cjs` の意図に合致する

## 3. `resolveEntry` 内部ヘルパー関数でロジックを共有

- **背景**: `resolvePackageName` と `resolveTargetTriple` は同一の validation logic（SUPPORTED_PLATFORMS チェック + マップルックアップ）を共有する
- **検討した選択肢**:
  - 各エクスポート関数内に重複した validation を書く
  - `resolveEntry` プライベートヘルパーで共通化
- **理由**: DRY 原則に従い、エラーメッセージの一貫性を保証する。テスト済みの `resolvePackageName` は `resolveEntry` 経由で動作するため、テストの信頼性も維持される
# CI / リリース

## 1. CI の目的

CI は次の 2 種類に分ける。

- PR / push 向けの再現性チェック
- release 向けの multi-target build / publish

live runtime を使う実機確認は、通常の PR CI とは分離する。

## 2. PR / push CI

### 実行項目

- `mise run test-coverage`
- coverage artifact upload
- PR では coverage comment 更新

### 目的

- 変換ロジックの回帰を止める
- packaging の基本破綻を早期に止める
- coverage の退行を早期に止める

### 将来追加する項目

- schema validation
- launcher build smoke test
- `features`, `normalize`, `report` を含む変換パイプラインの artifact 検証

## 3. Release build matrix

- aarch64-apple-darwin
- x86_64-apple-darwin
- x86_64-pc-windows-msvc
- x86_64-unknown-linux-gnu
- x86_64-unknown-linux-musl

## 4. Release 手順

1. Rust binary build
2. 生成物を各 `vendor/` にコピー
3. platform package publish
4. launcher publish

publish 順序は `platform -> launcher` に固定する。

## 5. バージョニングと変更管理

### バージョニング

- SemVer を採用する
- launcher と platform package は同一 version で運用する

### Changesets

- 変更点は `.changeset/` で管理する
- npm package の公開挙動に影響する変更では changeset を必須とする

## 6. Smoke test

### PR / release 共通

- launcher が build できる
- `attach`, `doctor`, `extract frame` の現在の runtime surface が失敗しない

### 将来の smoke test 拡張

- fixture 入力で `features`, `normalize`, `report` が実行できる
- `ui.ir.json` が schema validation を通る

### live runtime job

live runtime job を別で持てるなら、次を確認する。

- `cargo run -p figex-cli -- attach`
- `cargo run -p figex-cli -- doctor --json`
- `cargo run -p figex-cli -- extract frame <frame_ref>`

現状の CDP 検証は Chrome + Figma Web 版を前提にする。
Figma Desktop は現状 CDP ポートを公開しないため、通常 CI の必須条件にはしない。

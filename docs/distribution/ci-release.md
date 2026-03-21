# CI / リリース

## 1. CI の目的

CI は次の 2 種類に分ける。

- PR / push 向けの再現性チェック
- release 向けの multi-target build / publish

live runtime を使う実機確認は、通常の PR CI とは分離する。

## 2. PR / push CI

### 実行項目

- Rust format
- Rust clippy
- Rust test
- JS lint
- JS test
- schema validation
- launcher build

### 目的

- 変換ロジックの回帰を止める
- packaging の基本破綻を早期に止める
- `ui.ir.json` などの artifact shape を固定する

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
- fixture 入力で `features`, `normalize`, `report` が実行できる
- `ui.ir.json` が schema validation を通る

### live runtime job

live runtime job を別で持てるなら、次を確認する。

- `figex attach`
- `figex doctor --json`
- `figex extract frame <frame_ref>`

この job は Figma Desktop 実機依存のため、通常 CI の必須条件にはしない。

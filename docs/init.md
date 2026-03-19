# ネイティブ Rust CLI + npm 配布 仕様書

## 1. 目的

本仕様は、Rust で実装されたネイティブ CLI を npm（pnpm）経由で配布し、macOS / Windows / WSL / Docker 環境で動作させるための構成および実装方針を定義する。

- コア処理は Rust で実装する
- npm パッケージは TypeScript 製のランチャを提供する
- 実体は OS / CPU 別にビルドされたネイティブバイナリとする
- 利用者は `pnpm add` 後、`yourtool` コマンドを直接実行できる

---

## 2. 対象プラットフォーム

### 2.1 対応環境

| 利用環境              | Node から見える platform | Rust target triple        |
| --------------------- | ------------------------ | ------------------------- |
| macOS (Apple Silicon) | darwin / arm64           | aarch64-apple-darwin      |
| macOS (Intel)         | darwin / x64             | x86_64-apple-darwin       |
| Windows x64           | win32 / x64              | x86_64-pc-windows-msvc    |
| WSL                   | linux / x64              | x86_64-unknown-linux-gnu  |
| Docker (glibc)        | linux / x64              | x86_64-unknown-linux-gnu  |
| Docker (Alpine 等)    | linux / x64              | x86_64-unknown-linux-musl |

WSL は Linux として扱う。

Linux では glibc (gnu) と musl の両対応を行う。

---

## 3. 配布方式

### 3.1 採用方式

Codex 型構成を採用する。

- 共通ランチャ: `@taiga-tech/cli`
- プラットフォーム別バイナリ同梱パッケージ:
    - `@taiga-tech/cli-darwin-arm64`
    - `@taiga-tech/cli-darwin-x64`
    - `@taiga-tech/cli-win32-x64`
    - `@taiga-tech/cli-linux-x64-gnu`
    - `@taiga-tech/cli-linux-x64-musl`

ランチャは `optionalDependencies` として各プラットフォームパッケージを定義する。

---

## 4. リポジトリ構成

```
repo/
  pnpm-workspace.yaml
  package.json
  mise.toml
  crates/
    core/          # Rust ライブラリ（コア処理）
    cli/           # Rust バイナリ
  packages/
    cli/           # TypeScript ランチャ
    cli-darwin-arm64/
    cli-darwin-x64/
    cli-win32-x64/
    cli-linux-x64-gnu/
    cli-linux-x64-musl/
```

---

## 5. Rust 側仕様

### 5.1 構成

- `crates/core`: ビジネスロジック
- `crates/cli`: 引数処理、I/O、終了コード管理

### 5.2 ビルド

各ターゲットに対して以下を実行する。

```
cargo build --release --target <target-triple>
```

生成物:

```
target/<target-triple>/release/yourtool(.exe)
```

---

## 6. バイナリ同梱パッケージ仕様

### 6.1 ディレクトリ構造

```
vendor/
  <target-triple>/
    yourtool/
      yourtool(.exe)
```

### 6.2 package.json 例

```
{
  "name": "@taiga-tech/cli-linux-x64-gnu",
  "version": "0.1.0",
  "os": ["linux"],
  "cpu": ["x64"],
  "files": ["vendor/**"]
}
```

---

## 7. TypeScript ランチャ仕様

### 7.1 技術要件

- TypeScript で実装
- CommonJS としてビルド
- `bin` エントリは `.cjs` を指す

### 7.2 ビルド設定

- module: CommonJS
- target: ES2022
- 出力: `dist/`

### 7.3 実行フロー

1. `process.platform` / `process.arch` を取得
2. Linux の場合は glibc / musl を判定
3. 対応する platform パッケージ名を決定
4. `require.resolve("<pkg>/package.json")` でインストールパス取得
5. `vendor/<triple>/yourtool/...` を組み立て
6. `child_process.spawn()` で実行
7. 終了コードをそのまま伝播

### 7.4 Linux 判定方針

- glibc 検出可能なら gnu を選択
- それ以外は musl を試行

---

## 8. CI 要件

### 8.1 ビルドマトリクス

- aarch64-apple-darwin
- x86_64-apple-darwin
- x86_64-pc-windows-msvc
- x86_64-unknown-linux-gnu
- x86_64-unknown-linux-musl

### 8.2 手順

1. Rust ビルド
2. 生成物を各 `vendor/` にコピー
3. プラットフォームパッケージ publish
4. ランチャ publish

publish 順序は platform → launcher とする。

---

## 9. 制約事項

- `pnpm add --no-optional` では動作しない可能性がある
- Windows では `.exe` 拡張子必須
- Linux musl 未対応環境では起動失敗の可能性あり
- Node バージョン差異は LTS 範囲で検証する

---

## 10. 採用理由

- fs を中心とする処理は Wasm よりネイティブが安定
- OS 依存機能を直接 Rust で扱える
- npm 経由での導入体験を維持可能
- Codex と同様の構成で実績ある方式

---

## 11. バージョニングとリリース管理

### 11.1 バージョニング

- バージョニングはセマンティックバージョニング（SemVer）に従う。
- すべての npm パッケージ（ランチャおよびプラットフォーム別パッケージ）は同一バージョンで運用する（例: `@taiga-tech/cli@1.2.3` と `@taiga-tech/cli-linux-x64-gnu@1.2.3`）。

### 11.2 変更管理（Changesets）

- 変更点は Changesets で管理する。
- 変更が入るたびに `.changeset/` に changeset ファイルを追加し、リリース時に Changesets によりバージョン更新と changelog 生成を行う。

### 11.3 リリース順序

- プラットフォーム別パッケージを先に publish し、その後にランチャ（`@taiga-tech/cli`）を publish する。
- Changesets のリリース手順においても、この publish 順序を満たすよう CI を構成する。

以上を本プロジェクトの配布仕様とする。

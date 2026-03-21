# 配布・パッケージング仕様

## 1. 目的

本仕様は、Rust で実装されたネイティブ CLI を npm 経由で配布し、macOS / Windows / Linux / WSL / Docker 環境で動作させるための構成と実装方針を定義する。

- コア処理は Rust で実装する
- npm パッケージは TypeScript 製のランチャを提供する
- 実体は OS / CPU 別にビルドされたネイティブバイナリとする
- 利用者は `pnpm add @taiga-tech/figex-cli` 後、`figex` コマンドを直接実行できる

## 2. 命名

- npm launcher package: `@taiga-tech/figex-cli`
- platform package: `@taiga-tech/figex-cli-<platform>`
- 公開コマンド名: `figex`
- 同梱ネイティブバイナリ名: `figex-cli`

ランチャが `figex` コマンドを公開し、内部では `figex-cli(.exe)` を起動する。

## 3. 対象プラットフォーム

### 対応環境

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

## 4. 配布方式

### 採用方式

launcher + platform package の multi-package 配布を採用する。

- 共通ランチャ: `@taiga-tech/figex-cli`
- プラットフォーム別バイナリ同梱パッケージ:
    - `@taiga-tech/figex-cli-darwin-arm64`
    - `@taiga-tech/figex-cli-darwin-x64`
    - `@taiga-tech/figex-cli-win32-x64`
    - `@taiga-tech/figex-cli-linux-x64-gnu`
    - `@taiga-tech/figex-cli-linux-x64-musl`

ランチャは `optionalDependencies` として各 platform package を定義する。

## 5. リポジトリ構成

```text
repo/
  pnpm-workspace.yaml
  package.json
  mise.toml
  crates/
    core/
    cli/
  packages/
    cli/
    cli-darwin-arm64/
    cli-darwin-x64/
    cli-win32-x64/
    cli-linux-x64-gnu/
    cli-linux-x64-musl/
```

## 6. Rust 側仕様

### 構成

- `crates/core`: ドメインロジック
- `crates/cli`: 引数処理、I/O、終了コード管理

### ビルド

各 target に対して次を実行する。

```bash
cargo build --release --target <target-triple>
```

生成物:

```text
target/<target-triple>/release/figex-cli(.exe)
```

## 7. バイナリ同梱パッケージ仕様

### ディレクトリ構造

```text
vendor/
  <target-triple>/
    figex-cli/
      figex-cli(.exe)
```

### `package.json` 例

```json
{
    "name": "@taiga-tech/figex-cli-linux-x64-gnu",
    "version": "0.1.0",
    "os": ["linux"],
    "cpu": ["x64"],
    "files": ["vendor/**"]
}
```

## 8. TypeScript ランチャ仕様

### 技術要件

- TypeScript で実装
- CommonJS としてビルド
- `bin` エントリは `figex` を公開する
- 内部で `detect-libc` を使って `gnu / musl` を判定する

### 実行フロー

1. `process.platform` / `process.arch` を取得
2. Linux の場合は glibc / musl を判定
3. 対応する platform package 名を決定
4. `require.resolve("<pkg>/package.json")` で install path を取得
5. `vendor/<triple>/figex-cli/figex-cli(.exe)` を組み立て
6. `child_process.spawn()` で実行
7. 終了コードをそのまま伝播

## 9. 制約事項

- `pnpm add --no-optional` では動作しない可能性がある
- Windows では `.exe` 拡張子必須
- Linux musl 未対応環境では起動失敗の可能性がある
- Node バージョン差異は LTS 範囲で検証する

## 10. 採用理由

- fs を中心とする処理は Wasm よりネイティブが安定する
- OS 依存機能を Rust で扱える
- npm 経由での導入体験を維持できる
- platform package を分けることで install 時のサイズを抑えやすい

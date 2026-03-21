# アーキテクチャ設計

この文書は **目標アーキテクチャ** を定義する。
現時点のコード配置と一致しない箇所があっても、ここでは実装目標を優先する。

## 1. 命名と境界

- プロダクト名: `Figma Runtime Extractor`
- リポジトリ / npm ファミリ: `figex-cli`
- 公開 CLI コマンド: `figex`
- ネイティブバイナリ: `figex-cli`

## 2. 目標リポジトリ構成

```text
.
├── Cargo.toml
├── rust-toolchain.toml
├── mise.toml
├── package.json
├── pnpm-workspace.yaml
├── turbo.json
│
├── crates/
│   ├── core/
│   ├── cli/
│   ├── runtime/
│   ├── extractor/
│   ├── normalizer/
│   ├── classifier/
│   ├── reporter/
│   ├── schema/
│   └── store/
│
├── packages/
│   ├── cli/
│   ├── cli-darwin-arm64/
│   ├── cli-darwin-x64/
│   ├── cli-win32-x64/
│   ├── cli-linux-x64-gnu/
│   └── cli-linux-x64-musl/
│
├── schemas/
│   ├── raw.schema.json
│   ├── features.schema.json
│   └── ui.ir.schema.json
│
├── fixtures/
│   ├── raw/
│   ├── features/
│   └── ui-ir/
│
└── scripts/
    └── place-binary-like-ci.sh
```

## 3. crate ごとの責務

### `crates/core`

- 基本型定義
- Raw / Feature / UiIr の共通モデル
- diagnostics 型
- 共通エラー型
- serde / schemars 対応

### `crates/cli`

- `clap` による引数定義
- 設定ファイル読込
- コマンド dispatch
- 結果出力と終了コード制御

### `crates/runtime`

- Desktop との接続
- target 探索
- transport adapter
- snapshot 取得
- runtime health 診断

### `crates/extractor`

- `RawFrameSnapshot -> FeatureGraph`
- 構造特徴、視覚特徴、layout 特徴、text 特徴、反復特徴の抽出

### `crates/normalizer`

- 木の再構築
- layout-only wrapper collapse
- boundary detection
- `FeatureGraph -> UiIr`

### `crates/classifier`

- component candidate 推定
- atomic candidate 推定
- reusability score 推定
- confidence 付与

### `crates/reporter`

- `UiIr -> report.md`
- 低信頼箇所の要約
- 実装順序案の出力

### `crates/schema`

- `schemars` を用いた JSON Schema 生成
- schema version 管理

### `crates/store`

- JSON 保存
- 将来の SQLite backend 拡張点

## 4. Schema versioning

- `UiIr.version` は IR format version を表す
- `schemas/ui.ir.schema.json` はその version と整合する schema artifact とする
- 後方互換を保つ追加は minor
- 破壊的変更は major
- fixture と schema validation は version bump と同時に更新する

`UiIr.version` と schema artifact は整合させるが、実装上は「同じ文字列で完全一致させる」ことを必須ルールにはしない。

## 5. エラー設計

### 終了コード

- `0` 正常終了
- `10` 接続不可
- `11` target 未発見
- `12` snapshot 失敗
- `20` frame 未発見
- `21` data 不整合
- `30` normalize 失敗
- `31` classify 失敗
- `40` write 失敗
- `50` 設定不正

### error taxonomy

```rust
pub enum RuntimeError {
    ProbeFailed,
    TargetNotFound,
    AttachFailed,
    Timeout,
    EvaluateFailed,
    SnapshotFailed,
}

pub enum PipelineError {
    RawMappingFailed,
    FeatureExtractionFailed,
    NormalizeFailed,
    ClassificationFailed,
    ReportFailed,
    WriteFailed,
}
```

## 6. セキュリティと運用上の注意

このプロジェクトはローカルの Figma 実行環境と接続する。
したがって API token 管理は抱えない一方、ローカルセッション接続の安全性と可観測性が重要になる。

- 任意 eval を公開 API に出さない
- snapshot script はバージョン管理し、差分審査対象にする
- `doctor` で attach 先の title / url / target id を可視化する
- ログに snapshot 全文を勝手に吐かない
- エラーログから本文テキストが漏れないよう redact option を持つ

また、Figma 公式は MCP server を desktop / remote の両形態で提供している。
2026-03-22 時点の公式ドキュメントでは、[desktop server docs](https://developers.figma.com/docs/figma-mcp-server/local-server-installation/) に `http://127.0.0.1:3845/mcp`、[remote server docs](https://developers.figma.com/docs/figma-mcp-server/remote-server-installation/) に `https://mcp.figma.com/mcp` が案内されている。
将来の `runtime-mcp` 追加を見越し、transport 境界は固定しておく。

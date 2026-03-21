# Docs Overview

`docs/` は、このリポジトリで今後実装する `figex` の設計正本を置く場所です。
現時点のコード実装状況とは一致しない箇所がありますが、ここでは **目標仕様** を管理します。

## 文書の優先順位

読む順番と、矛盾したときに優先する順番は次の通りです。

1. `product/prd.md`
2. `product/decisions.md`
3. `design/data-models.md`
4. `design/cli-spec.md`
5. `design/pipeline.md`
6. `design/architecture.md`
7. `distribution/*.md`
8. `development/*.md`

`reference/` は保管用です。検討ログや外部分析を残しますが、正本ではありません。
入口は `reference/README.md` とし、巨大なアーカイブ本文を直接読み始めない運用にする。

## 命名の正本

| 種別                      | 正式名称                                               |
| ------------------------- | ------------------------------------------------------ |
| プロダクト名              | `Figma Runtime Extractor`                              |
| リポジトリ / npm ファミリ | `figex-cli`                                            |
| CLI コマンド名            | `figex`                                                |
| ネイティブバイナリ名      | `figex-cli` (`Windows` は `figex-cli.exe`)             |
| 設定ファイル名            | `figex.toml`                                           |
| 環境変数 prefix           | `FIGEX_`                                               |
| v1 の主要成果物           | `raw.json`, `features.json`, `ui.ir.json`, `report.md` |

## v1 の正規データモデル

`UiIr` の canonical shape は次に固定します。

- top-level: `version`, `source`, `tree`, `diagnostics`, `stats`
- node-level: snake_case の JSON field を使う
- `tree` を正規形式とし、flat `nodes + root_id` は採用しない
- token 抽出は v1 の必須成果物に含めない

正確な型定義は `design/data-models.md` を参照してください。

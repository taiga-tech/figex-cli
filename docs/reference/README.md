# Reference Docs Guide

`reference/` は「正本ではないが捨てたくない検討ログ」を置く場所です。
日常的に読む入口としては、この `README.md` を使い、必要になったときだけ `figma-cli-analysis.md` を開く運用にします。

## 役割

- `docs/product` / `docs/design`: 現在の正本
- `docs/reference`: 検討ログ、比較材料、外部分析の保管
- `docs/reference/figma-cli-analysis.md`: 巨大な一次ログ

## 使い方

### 5 分で要点だけ掴みたいとき

次の節だけを見る。

- `# 最終設計`
- `# CLI の完成形`
- `# テスト戦略`
- `# 実装順序`
- 末尾の `# 最終判断`

### 15 分で設計の理由まで追いたいとき

次の順で見る。

1. 最初の批評ブロック
2. 各 `## 先に結論`
3. `# 最終設計`
4. 末尾の `# 最終判断`

### 実装時に参照するとき

次のルールで使う。

- まず正本 (`docs/product`, `docs/design`) を見る
- 正本に根拠が足りないときだけ `reference/` に降りる
- `reference/` から使う内容は、そのままコードに落とさず正本へ昇格させてから使う

## 読み分け

### 信用してよいもの

- 最終段の統合セクション
- 途中の比較から抽出できる設計判断
- 失敗例、避けるべき案、リスクの棚卸し

### そのまま信用しないもの

- 旧コマンド名
- 旧 schema 例
- plugin / daemon 前提の旧案
- 途中の code snippet
- 同名見出しのうち中盤にある一時結論

## すぐ使える検索

```bash
rg -n "^(## 先に結論|# 最終設計|# CLI の完成形|# テスト戦略|# 実装順序|# 最終判断)$" docs/reference/figma-cli-analysis.md
```

```bash
rg -n "wrapper collapse|boundary detection|diagnostics|runtime|normalize|classifier" docs/reference/figma-cli-analysis.md
```

## 現在の正本との対応

- プロダクト方針: `docs/product/decisions.md`
- 要件: `docs/product/prd.md`
- データモデル: `docs/design/data-models.md`
- CLI surface: `docs/design/cli-spec.md`
- パイプライン: `docs/design/pipeline.md`

この対応を崩さない限り、`figma-cli-analysis.md` は「巨大だが使える裏取り資料」として機能する。

# Figma Runtime Extractor — プロダクト要件定義

## 1. プロダクト名

**Figma Runtime Extractor**

## 2. 概要

Figma runtime に接続し、指定フレームを観測して、実装判断に使える **UI IR** を生成する CLI を提供する。
本プロダクトは、Figma から直接コードを生成する道具ではない。コード生成の前段に位置する **抽出・正規化・分類・説明** のための道具として定義する。

## 3. 背景

Figma の生ノード構造は、描画や編集都合のノイズを多く含むため、そのまま実装へ落とし込むと wrapper 過多、責務分離の崩壊、分類の不安定化を招く。
必要なのは、生データそのものではなく、**実装都合へ寄せて正規化された意味付き中間表現** である。

## 4. 解決したい課題

### 4.1 現在の課題

- Figma のノード構造は、実装単位と一致しない
- 実装者が欲しいのは座標ダンプではなく、構造・レイアウト・再利用性・曖昧さを含む判断材料である
- Atom / Molecule / Organism の分類は単純な木構造だけでは決まらない
- 自動変換系は、誤判定箇所が見えないと信用されない

### 4.2 本プロダクトが解く範囲

- runtime から対象フレームを取得する
- 生情報から特徴量を抽出する
- 実装向け UI IR に正規化する
- component / atomic / reusability の候補を付与する
- 人間確認用のレポートを出す

## 5. 目的

本プロダクトの目的は、指定フレームを **実装判断に耐える UI IR** として出力し、後段のコード生成器や人間の実装判断に渡せる状態にすることである。

## 6. 非目標

本プロダクトは以下を対象にしない。

- Figma REST API を v1 の必須経路にすること
- Figma plugin を v1 の必須経路にすること
- Figma 上の create / mutate 操作
- 画像 export
- arbitrary eval の公開 API
- JSX 直接生成
- Storybook 直接生成
- React コンポーネント直接生成
- 万能 Figma CLI 化
- LLM を中心にした全面分類
- design token 抽出を v1 の必須成果物にすること

## 7. 対象ユーザー

### 主対象

- Figma から実装向け構造を抽出したいフロントエンド実装者
- デザインシステムや UI 実装規約に沿って分割したい開発者
- figma2code の前処理を安定化したいツール開発者

### 想定しない対象

- Figma を自動編集したいユーザー
- Figma 全般を操作する万能 CLI を求めるユーザー
- 完全自動の最終コード生成だけを求めるユーザー

## 8. ユースケース

### 8.1 主要ユースケース

1. ユーザーは Figma runtime に接続する
2. 対象フレームを指定する
3. 生スナップショットを取得する
4. 特徴量を抽出する
5. UI IR へ正規化する
6. component / atomic / reusability 候補を付与する
7. `ui.ir.json` と `report.md` を確認する
8. 後段のコード生成器または人間の実装作業へ渡す

### 8.2 典型的な利用場面

- デザインを React 実装へ移す前の分解
- コンポーネント境界の候補抽出
- アトミックデザイン分類のたたき台生成
- 低信頼箇所の確認と手修正対象の把握

## 9. プロダクト原則

### 9.1 出力の中心はコードではなく UI IR

本体の価値はコード出力ではなく、**意味付き中間表現の品質** にある。

### 9.2 JSON は保存形式であり、設計の中心ではない

内部の中心は型付き IR とし、JSON はそのシリアライズ形式として扱う。

### 9.3 接続方式を価値の中心にしない

接続は必要条件だが、本体価値は以下に置く。

- wrapper collapse
- boundary detection
- diagnostics

### 9.4 分類は断定ではなく候補提示

分類は単一ラベル断定ではなく、複数候補と score を返す。

### 9.5 誤判定を隠さない

低信頼箇所、曖昧な境界、推定理由を diagnostics と report に残す。

## 10. スコープ

### 10.1 In Scope

- runtime への接続
- 対象フレームの snapshot 取得
- `RawFrameSnapshot` 生成
- `FeatureGraph` 生成
- `UiIr` 生成
- `component_candidates / atomic_candidates / confidence / reusability` 付与
- JSON 保存
- Markdown レポート生成
- CLI による実行

### 10.2 Out of Scope

- 任意コード実行 API の公開
- Figma 側の編集機能
- 画像出力やレンダリング
- コード生成の完成版
- plugin 配布
- クラウドサービス化

## 11. 主要成果物

本プロダクトは最低でも以下を出力する。

### 11.1 `raw.json`

接続層の取得結果。
意味づけ前の生データ。デバッグと再抽出に使う。

### 11.2 `features.json`

観測可能な特徴量。
構造、レイアウト、視覚、文字、反復情報を保持する。

### 11.3 `ui.ir.json`

主成果物。
実装判断に使う意味付き中間表現。

### 11.4 `report.md`

人間向け要約。
低信頼箇所、境界推定、再利用候補、確認点を示す。

## 12. 必須機能要件

### 12.1 接続

- runtime に接続できること
- 接続状態確認ができること
- 対象フレームを snapshot できること
- 公開 API は `attach`, `health`, `snapshot_frame` に限定すること

### 12.2 抽出

- `RawFrameSnapshot` から `FeatureGraph` を生成できること
- 少なくとも以下の特徴を取ること:
    - depth
    - child_count
    - sibling_index
    - subtree_size
    - layout direction
    - gap
    - padding
    - fill / radius / shadow
    - text content
    - repeated_subtree_count

### 12.3 正規化

- layout-only wrapper collapse を行うこと
- boundary detection を行うこと
- source node ids を保持したまま `UiNode` へ変換すること

### 12.4 分類

- `component_candidates` を返すこと
- `atomic_candidates` を返すこと
- `confidence` を返すこと
- `reusability` を返すこと
- 単一ラベル断定を避けること

### 12.5 説明可能性

- diagnostics を top-level と node-level の両方で保持すること
- 低信頼判定を report に露出すること

### 12.6 保存

- JSON ファイルとして保存できること
- 将来的な SQLite 保存に備えて保存層を分離すること

## 13. データ要件

この文書では exact schema を再定義しない。
正規の field 名と構造は `docs/design/data-models.md` を参照する。

### 13.1 `UiIr` の必須概念

- `version`
- `source`
- `tree`
- `diagnostics`
- `stats`

### 13.2 `UiNode` の必須概念

- node id
- kind
- source node ids
- children
- component candidates
- atomic candidates
- confidence
- layout
- semantics

### 13.3 削ってはいけない情報

- source node ids
- component candidates
- atomic candidates
- confidence
- diagnostics

これらは逆引き、推定理由、修正判断に必要である。

# 設計判断と原則

## 目的

このプロジェクトは、Figma からコードを直接生成する道具としてではなく、**実装判断に耐える UI IR を抽出する道具**として設計する。

## 固定した前提

- Figma REST API は v1 の必須経路にしない
- Figma plugin も v1 の必須経路にしない
- Desktop 実行環境との接続を優先する
- ただし transport 抽象は固定し、remote MCP の追加を妨げない
- 配布骨格は `figex-cli` を使う
- 出力の中心はコードではなく UI IR に置く
- コード生成は後段に分離する

## 外部前提の確認

2026-03-22 時点で Figma 公式ドキュメントを確認した。

- Figma MCP server には desktop と remote の両方がある
- desktop server は `http://127.0.0.1:3845/mcp`
- remote server は `https://mcp.figma.com/mcp`
- desktop server は Figma desktop app 上で有効化する
- desktop server の有効化位置は Dev Mode の inspect panel 内である
- MCP server の利用条件と rate limit は Figma plan / seat に依存する

参照:

- https://developers.figma.com/docs/figma-mcp-server/local-server-installation/
- https://developers.figma.com/docs/figma-mcp-server/remote-server-installation/
- https://developers.figma.com/docs/figma-mcp-server/plans-access-and-permissions/

将来の runtime 設計では、この公式 MCP 経路を transport 候補として残す。

## プロダクト定義

このプロジェクトは汎用の Figma CLI ではない。
**Figma Runtime Extractor** として定義する。

本体の責務は次の 4 つに固定する。

1. Figma runtime に接続する
2. 指定フレームを観測する
3. 実装向け UI IR に正規化する
4. JSON とレポートを出す

## 非目標

次のものは本体に入れない。

- 作成系の操作
- 更新系の操作
- 画像 export
- 任意 `eval` の公開 API
- JSX 生成
- Storybook 生成
- React 直接出力
- 汎用 Figma CLI 化
- LLM を中心にした全面分類
- design token 抽出を v1 の必須成果物にすること

## 設計原則

### 1. JSON を設計の中心に置かない

JSON は採用するが、設計の中心には置かない。
中心に置くのは **意味付きの中間表現としての UI IR** である。

### 2. 接続方式をプロダクトの中心に置かない

runtime 接続は必要条件だが、価値の中心は次の 3 点である。

- wrapper collapse
- boundary detection
- diagnostics

### 3. 解析パイプラインとして設計する

CLI の責務は広げず、次だけに限定する。

- Figma から取得する
- 対象フレームを切り出す
- UI 構造へ正規化する
- コンポーネント候補をグルーピングする
- 実装向け IR を出す
- 必要なら後段の生成器へ渡す

## 設計判断の総括

### 採用する判断

- 配布は `figex-cli` の Rust + npm launcher + platform package 構成を踏襲する
- ドメイン構造は `runtime -> raw -> features -> ui_ir -> report` の直列パイプラインとする
- runtime の公開 API は `attach`, `health`, `snapshot_frame` に限定する
- 取得データは `Raw / Features / UiIr` に三段分離する
- 正規化の中核は `wrapper collapse`, `boundary detection`, `diagnostics` とする
- classifier は断定器ではなく推定器とする
- 保存形式は当初 JSON のみとし、SQLite は後段とする

### 採用しない判断

- plugin 経由ブリッジを v1 の必須経路にすること
- daemon 常駐前提
- arbitrary eval の公開
- 書き込み操作を混ぜた万能 CLI
- コード生成まで一気通貫で抱える構成
- `tokens.json` を v1 の必須成果物にすること

### 2026-03 時点で見直した点

以前の検討では `CDP 直結のみ` に寄りすぎていたが、Figma 公式は desktop / remote の MCP server を公開している。
したがって、v1 では CDP 優先で入っても、runtime 境界は `transport` 抽象で切り、`mcp` を後から差し込める形にするのが妥当である。

## 価値が出る箇所

本プロジェクトで差が出るのは次の 3 点である。

### wrapper collapse

これが弱いと出力が冗長になる。

### boundary detection

これが弱いと component 分割が信用されない。

### diagnostics

これが弱いと、人が誤判定を追えない。

runtime が完全でなくても、この 3 点が強ければ道具として成立する。

## 現時点のリスク

### 最大の技術リスク

CDP 直結は Figma Desktop 側の変更で壊れやすい可能性がある。
対策として transport adapter を分離し、snapshot 取得の公開契約を trait で固定する。

### データ品質リスク

Auto Layout, text, bounds の欠落があると downstream が崩れる。
Raw で削らず保持する方針で緩和する。

### 誤分類リスク

classifier を推定器として扱い、confidence と diagnostics を常に同梱する。

### 配布リスク

ネイティブ配布はプラットフォーム差異で詰まりやすいため、`figex-cli` の vendor 配置規約に沿って CI で早期に検証する。

## 将来拡張

本体完成後に追加する crate / package は次を想定する。

- `runtime-mcp`
- `projector-react`
- `projector-html`
- `projector-storybook`
- `store-sqlite`

これらは **UiIr を入力に取る後段** とし、core pipeline を汚さない形で追加する。

## 最終判断

このプロジェクトは、Figma から直接コードを吐く道具として作るより、**Figma から実装判断に耐える IR を引き抜く抽出器**として作る方が構造的に健全である。

完成条件は次の 1 本である。

```text
runtime
  -> RawFrameSnapshot
  -> FeatureGraph
  -> UiIr
  -> report.md
```

配布は `figex-cli` の骨格に寄せる。
接続は runtime に寄せる。
plugin / daemon / codegen を本体から切る。
normalizer と diagnostics に工数を集中させる。

# 実装計画

## 第1段階: runtime 接続成立

- CDP endpoint discovery
- `Runtime.enable`
- `Runtime.evaluate`
- ping
- snapshot frame
- `doctor` の最小診断

## 第2段階: Raw を固める

- auto layout
- text raw content
- fills / effects / radius
- source metadata
- Raw schema 固定

## 第3段階: FeatureGraph を固める

- subtree fingerprint
- repeated counts
- layout / text / visual feature 抽出
- feature fixture の固定

## 第4段階: normalizer を固める

- wrapper collapse
- boundary detection
- tree reconstruction
- diagnostics

## 第5段階: classifier と reporter

- component naming
- atomic candidates
- reusability
- confidence
- report.md

## 第6段階: schema / packaging / 回帰試験

- schema versioning
- schema validation
- launcher / packaging 検証
- 実データ fixture の追加
- regression suite 固定

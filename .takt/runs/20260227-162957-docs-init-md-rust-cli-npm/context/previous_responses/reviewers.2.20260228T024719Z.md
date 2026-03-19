## arch-review
必要な情報が揃いました。レビューを実施します。

---

## アーキテクチャレビュー結果

### 前回 Open Findings の追跡

前回 arch-review（Iteration 1）はレート制限により空のレスポンスを返しており、**追跡すべき既存 finding は存在しない**。

---

### 新規検出: ARCH-001

**finding_id:** ARCH-001  
**状態:** new  
**分類:** BLOCKING（変更ファイル内の問題 / ボーイスカウトルール）

---

#### 問題

`packages/cli/tsup.config.ts` に `clean: true` が未設定。

```typescript
// 現状
export default defineConfig({
  entry: ['src/index.ts'],
  format: ['cjs'],
  outDir: 'dist',
  outExtension: () => ({ js: '.cjs' }),
  // ← clean: true がない
})
```

`clean: true` がないため、tsup はビルド前に `dist/` を削除しない。前回ビルド（`outExtension` 未設定時）が生成した `dist/index.js`（タイムスタンプ: 28 Feb 01:46）が現在も残存している。

```
dist/index.cjs  ← 今回ビルドの正規出力（28 Feb 06:37）
dist/index.js   ← 前回ビルドの残滓（28 Feb 01:46）← 問題
```

`package.json` の `"files": ["dist/**"]` により、`dist/` 配下の全ファイルが npm に publish される。開発者がローカルでビルド後に publish した場合、不要な `index.js` が同梱される。

**根拠（実ファイル確認）:**
- `packages/cli/dist/index.js` の存在を `ls -la` で確認済み（28 Feb 01:46）
- `packages/cli/tsup.config.ts` に `clean` プロパティが存在しないことを `Read` で確認済み
- `.gitignore` に `dist` が記載されているため、CI では問題が再現しないが、ローカル publish では確実に混入する

**修正案:**

```typescript
export default defineConfig({
  entry: ['src/index.ts'],
  format: ['cjs'],
  outDir: 'dist',
  outExtension: () => ({ js: '.cjs' }),
  clean: true,  // ← 追加
})
```

---

### その他確認項目（問題なし）

| 確認項目 | 結果 |
|---|---|
| `platform.ts` の設計（PLATFORM_ENTRIES による一元管理） | OK |
| `BINARY_NAME` 定数の1箇所管理 | OK |
| `resolveEntry` のプライベート化・公開 API の分離 | OK |
| `pnpm-workspace.yaml`（`packages/*` で全パッケージをカバー済み） | OK |
| `Cargo.toml` ワークスペース構成 | OK |
| CI マトリクス（5ターゲット）と publish 順序（プラットフォーム先、ランチャー後） | OK |
| `.changeset/config.json` の `linked` 設定（全パッケージ同一バージョン） | OK |
| `detect-libc` の `dependencies` 配置（ランタイム依存） | OK |
| プラットフォームパッケージの `os`/`cpu`/`files` フィールド | OK |
| エラーハンドリング（`result.status ?? 1`） | OK |
| テストカバレッジ（13ケース: 正常系・異常系・エッジケース） | OK |

---

### 判定: **REJECT**

ARCH-001（`tsup.config.ts` の `clean: true` 欠如）は変更ファイル内の問題（ボーイスカウトルール適用）であり、ブロッキングと判定する。

修正内容は1行追加のみ: `packages/cli/tsup.config.ts` に `clean: true` を追加する。

---

## qa-review
レポートファイルへの書き込みは後のフェーズで自動的に行われるため、ここではレビュー結果を回答します。

---

## QA Review: **APPROVE** ✅

### 前回 Open Findings の追跡

前回の `qa-review` は API 制限により出力なし → 追跡対象なし。

---

### 今回の変更スコープ

| ファイル | 変更種別 |
|---|---|
| `packages/cli/tsup.config.ts` | 新規作成 |
| `packages/cli/package.json` | `scripts.build` 変更（`"tsup --out-extension .js=.cjs"` → `"tsup"`） |

---

### `packages/cli/tsup.config.ts`（新規）

```typescript
import { defineConfig } from 'tsup'
export default defineConfig({
  entry: ['src/index.ts'],
  format: ['cjs'],
  outDir: 'dist',
  outExtension: () => ({ js: '.cjs' }),
})
```

- タスク仕様（CommonJS, `dist/index.cjs`）に完全合致 ✓
- `outExtension` は tsup の正式な設定方式 ✓（CLI フラグではなく設定ファイルで指定するのが正解）
- `any` 型なし、TODO なし、未使用コードなし ✓
- 設定ファイルのためテスト不要 ✓

### `packages/cli/package.json`（変更）

- `"build": "tsup"` → 設定ファイルへ委譲。クリーン ✓

---

### モジュール全体（ボーイスカウトルール確認）

**`platform.ts`:**
- `Readonly<Record<...>>` でイミュータブル定義 ✓
- 内部ヘルパー `resolveEntry`/`buildKey` は非エクスポート（内部実装の漏洩なし） ✓
- `any` 型なし、具体的エラーメッセージ ✓

**`index.ts`:**
- `BINARY_NAME` を定数として1箇所で管理（仕様準拠） ✓
- パッケージ解決失敗・バイナリ実行失敗の両方でエラーハンドリング ✓
- `process.exit(result.status ?? 1)` — シグナル kill 時に 1 で終了するのは合理的デフォルト（フォールバック乱用には該当しない） ✓
- 空 `catch` なし ✓

**`platform.test.ts`（13テスト全通過）:**

| テストグループ | カバレッジ |
|---|---|
| `resolvePackageName` (8件) | darwin/arm64, darwin/x64, win32/x64, linux+glibc, linux+musl, isGlibc フラグ無視、未対応 platform、未対応 arch |
| `resolveTargetTriple` (5件) | darwin/arm64, darwin/x64, win32/x64, linux+glibc, linux+musl |

タスク仕様に列挙されたすべてのテストケースをカバー ✓

---

### REJECT 基準チェックリスト

| 基準 | 結果 |
|---|---|
| テストなしの新規ビヘイビア | なし（設定ファイル） |
| `any` 型 | なし |
| フォールバック値の乱用 | なし |
| 未使用コード | なし |
| 空 `catch` | なし |
| TODO コメント | なし |
| DRY 違反 | なし |
| 内部実装のパブリックエクスポート | なし |

---

### 総評

根本原因（tsup v8 CLI に `--out-extension` フラグが存在しない）を正確に解決。`tsup.config.ts` による設定ファイル方式への移行は適切であり、ビルド成功（`dist/index.cjs` 生成）と全 13 テスト通過が確認されている。REJECT 基準に該当する問題は検出されなかった。

---

## testing-review
これで十分な情報が揃いました。レビューを実施します。

## レビュー結果

### テスト計画との突合

| タスク指示書のテストケース | 実装状況 |
|---|---|
| darwin/arm64 → `@taiga-tech/cli-darwin-arm64` | ✅ 実装済み（L7-11） |
| darwin/x64 → `@taiga-tech/cli-darwin-x64` | ✅ 実装済み（L13-17） |
| win32/x64 → `@taiga-tech/cli-win32-x64` | ✅ 実装済み（L21-25） |
| linux/x64 + glibc検出 → `@taiga-tech/cli-linux-x64-gnu` | ✅ 実装済み（L29-33） |
| linux/x64 + glibc未検出 → `@taiga-tech/cli-linux-x64-musl` | ✅ 実装済み（L35-39） |
| 未対応プラットフォーム → エラースロー | ✅ 実装済み（L51-55） |

計画外で追加されたケース（`test-decisions.md` に記録済み）:
- ✅ `darwin + isGlibc=true`（isGlibc フラグが linux 以外で無視されることを検証）
- ✅ `linux/arm64`（platform 存在・arch 未対応のエラーメッセージ形式を検証）
- ✅ `resolveTargetTriple` の5ケース（`platform.ts` が2関数を公開しているため対象として適切）

---

### テスト構造評価

**Given-When-Then**: 純粋関数のシンプルな1:1テストのため、1行インライン形式は許容範囲。Arrange-Act-Assert が暗黙に成立している。

**独立性・再現性**: `process.platform`/`process.arch` に依存せず、全引数を直接渡している。実行順序・時間・外部状態に依存なし。✅

**モック設計**: `platform.ts` は `detect-libc` を使わず、`isGlibc: boolean` を引数として受け取る設計のため、モック不要。✅

---

### 問題一覧

**ブロッキング問題: なし**

| # | Finding ID | 種別 | ファイル/行 | 内容 |
|---|---|---|---|---|
| 1 | W-01 | Warning | `platform.test.ts:64-84` | `resolveTargetTriple` に異常系テスト（未対応 platform/arch）が存在しない。`resolvePackageName` の異常系テスト（L51-61）は共有実装 `resolveEntry()` 経由で `resolveTargetTriple` も間接的に保護しているが、直接テストがない |
| 2 | W-02 | Warning | `platform.test.ts` 全体 | テスト名が `should {振る舞い} when {条件}` パターンでなく矢印表記（例: `darwin/arm64 → @taiga-tech/cli-darwin-arm64`）。意図は十分読み取れるため深刻ではない |
| 3 | W-03 | Warning | `platform.test.ts:64-84` | `resolveTargetTriple` に `darwin + isGlibc=true` のエッジケーステストがない。`resolvePackageName` 側（L43-47）にはある |

---

### 判定

```
✅ APPROVE
```

タスク指示書の全6テストケースが実装済み。追加された8件（エッジケース2件 + `resolveTargetTriple` 5件 + darwin+isGlibc=true 1件）は品質向上に貢献している。ブロッキング問題なし。

Warning は3件記録するが、いずれも共有実装 `resolveEntry()` による間接保護が成立しており、誤検知・検知漏れ・フレークの実害は確認されない。
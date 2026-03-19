---

## AI Antipattern Review — 第2回

### 前回指摘項目の確認

| finding_id | 状態 | 根拠 |
|---|---|---|
| F-001 | ✅ resolved | `packages/cli/package.json` に `"type": "module"` なし |
| F-002 | ✅ resolved | `platform.test.ts` に `// When:` / `// Then:` コメントなし |
| F-003 | ✅ resolved | `crates/cli/src/main.rs` — `use std::process;` も `process::exit(0)` もない |
| F-004 | ✅ resolved | `platform.test.ts:72-94` に `resolveTargetTriple` の describe ブロックが追加済み |
| F-005 | ✅ resolved | `@types/node: "^24.0.0"` |

---

### 新規検出問題

#### F-NEW-001 [BLOCKING] — tsup ビルド出力ファイル名と `bin` エントリが不一致

**ファイル:** `packages/cli/package.json:6, 10`

**問題:**

```json
// L6
"sample-cli": "./dist/index.cjs"   ← bin はこちらを参照

// L10
"build": "tsup src/index.ts --format cjs --outDir dist"
```

F-001 修正で `"type": "module"` を削除した結果、tsup の出力ファイル名が変わった。

tsup v8 の出力拡張子ルール:
- `"type": "module"` **あり** + `--format cjs` → **`.cjs`** を出力
- `"type": "module"` **なし** + `--format cjs` → **`.js`** を出力 ← 現在の状態

現在の `package.json` には `"type"` フィールドが存在しないため、tsup は `dist/index.js` を出力する。しかし `bin` エントリは `./dist/index.cjs` を指しており、**`pnpm --filter @taiga-tech/cli build` 後にバイナリが存在しない。** CLI コマンド実行時に `Error: Cannot find module './dist/index.cjs'` が発生する。

これは F-001 修正が導入した新たなバグ（「今回の変更により生じた問題」→ BLOCKING）。

**修正方法:** ビルドスクリプトに `--out-extension '.js=.cjs'` を追加して拡張子を強制する。

```json
"build": "tsup src/index.ts --format cjs --outDir dist --out-extension '.js=.cjs'"
```

---

### 判定

```
REJECT
```

**理由:** F-NEW-001（`bin` エントリが存在しないファイルを参照）— F-001 修正によって導入されたビルド破壊バグ。
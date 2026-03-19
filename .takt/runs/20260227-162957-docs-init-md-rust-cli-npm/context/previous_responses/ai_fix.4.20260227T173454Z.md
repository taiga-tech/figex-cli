3つのコメントがすべて削除されていることを確認しました（grep 結果なし）。

---

## 確認したファイル

- `packages/cli/src/platform.ts:14` — `// Linux requires libc variant suffix; other platforms do not` が存在 → 削除済み
- `packages/cli/src/index.ts:8` — `// Single source of truth for the binary name across TypeScript code` が存在 → 削除済み
- `packages/cli/src/platform.test.ts:44` — `// Given: darwin に isGlibc=true を誤って渡した場合 (musl 選択ロジックは linux 限定)` が存在 → 削除済み

## 実行した検索

- `grep -n "Linux requires\|Single source of truth\|Given: darwin"` → 出力なし（3コメント全て削除を確認）

## 修正内容

| finding_id | ファイル | 修正内容 |
|-----------|---------|---------|
| F-NEW-002 | `platform.ts:14` | What コメント行（`// Linux requires libc variant suffix; other platforms do not`）を削除 |
| F-NEW-003 | `index.ts:8` | What コメント行（`// Single source of truth for the binary name across TypeScript code`）を削除 |
| F-NEW-004 | `platform.test.ts:44` | What コメント行（`// Given: darwin に isGlibc=true を誤って渡した場合 (musl 選択ロジックは linux 限定)`）を削除 |

## テスト結果

```
pnpm --filter @taiga-tech/cli test
✓ src/platform.test.ts (13 tests) 6ms
Test Files  1 passed (1)
Tests       13 passed (13)
```

全13テスト通過。
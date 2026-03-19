全13テスト通過。

---

## 確認したファイル

- `packages/cli/src/platform.ts:3-4` — 旧行3 `// All supported platform/arch/libc combinations mapped to npm package name and Rust target triple.`（What）と旧行4 `// Key format: ...`（How）が実在を確認
- `packages/cli/src/index.ts:13` — `// familySync() returns 'glibc' on GNU/Linux, null on non-Linux or musl`（What）が実在を確認
- `packages/cli/src/platform.test.ts:8,15,24,33,40,58,65,86,91` — 9箇所の `// Given:` コメントが実在を確認（49行目の Why コメントは存置）

## 実行した検索

- `Read` ツールで3ファイルを直接読み込み、指摘箇所の実在を目視確認

## 修正内容

| ファイル | 修正 |
|---|---|
| `platform.ts:3-4` | 2行のWhat/Howコメントを削除 |
| `index.ts:13` | `familySync()`戻り値を説明するWhatコメントを削除 |
| `platform.test.ts` | 旧8,15,24,33,40,58,65,86,91行の `// Given:` コメント9箇所を削除。旧49行（`// Given: darwin に isGlibc=true を誤って渡した場合...`）はWhyコメントのため存置 |

## テスト結果

```
pnpm --filter @taiga-tech/cli test
✓ src/platform.test.ts (13 tests) 6ms
Test Files  1 passed (1)
Tests       13 passed (13)
```

全13テスト通過。
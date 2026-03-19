import { describe, expect, it } from 'vitest'

import { resolvePackageName, resolveTargetTriple } from './platform'

describe('resolvePackageName', () => {
    // ── 正常系: darwin ────────────────────────────────────────────────────────

    it('darwin/arm64 → @taiga-tech/cli-darwin-arm64', () => {
        expect(resolvePackageName('darwin', 'arm64', false)).toBe(
            '@taiga-tech/cli-darwin-arm64'
        )
    })

    it('darwin/x64 → @taiga-tech/cli-darwin-x64', () => {
        expect(resolvePackageName('darwin', 'x64', false)).toBe(
            '@taiga-tech/cli-darwin-x64'
        )
    })

    // ── 正常系: win32 ─────────────────────────────────────────────────────────

    it('win32/x64 → @taiga-tech/cli-win32-x64', () => {
        expect(resolvePackageName('win32', 'x64', false)).toBe(
            '@taiga-tech/cli-win32-x64'
        )
    })

    // ── 正常系: linux ─────────────────────────────────────────────────────────

    it('linux/x64 + glibc 検出 → @taiga-tech/cli-linux-x64-gnu', () => {
        expect(resolvePackageName('linux', 'x64', true)).toBe(
            '@taiga-tech/cli-linux-x64-gnu'
        )
    })

    it('linux/x64 + glibc 未検出 → @taiga-tech/cli-linux-x64-musl', () => {
        expect(resolvePackageName('linux', 'x64', false)).toBe(
            '@taiga-tech/cli-linux-x64-musl'
        )
    })

    // ── エッジケース ─────────────────────────────────────────────────────────

    it('darwin では isGlibc フラグを無視して arm64 パッケージを返す', () => {
        expect(resolvePackageName('darwin', 'arm64', true)).toBe(
            '@taiga-tech/cli-darwin-arm64'
        )
    })

    // ── 異常系 ────────────────────────────────────────────────────────────────

    it('未対応 platform → "Unsupported platform: ..." をスロー', () => {
        expect(() => resolvePackageName('freebsd', 'x64', false)).toThrow(
            'Unsupported platform: freebsd'
        )
    })

    it('未対応 arch → "Unsupported arch: ..." をスロー', () => {
        expect(() => resolvePackageName('linux', 'arm64', false)).toThrow(
            'Unsupported arch: linux/arm64'
        )
    })
})

describe('resolveTargetTriple', () => {
    it('darwin/arm64 → aarch64-apple-darwin', () => {
        expect(resolveTargetTriple('darwin', 'arm64', false)).toBe(
            'aarch64-apple-darwin'
        )
    })

    it('darwin/x64 → x86_64-apple-darwin', () => {
        expect(resolveTargetTriple('darwin', 'x64', false)).toBe(
            'x86_64-apple-darwin'
        )
    })

    it('win32/x64 → x86_64-pc-windows-msvc', () => {
        expect(resolveTargetTriple('win32', 'x64', false)).toBe(
            'x86_64-pc-windows-msvc'
        )
    })

    it('linux/x64 + glibc 検出 → x86_64-unknown-linux-gnu', () => {
        expect(resolveTargetTriple('linux', 'x64', true)).toBe(
            'x86_64-unknown-linux-gnu'
        )
    })

    it('linux/x64 + glibc 未検出 → x86_64-unknown-linux-musl', () => {
        expect(resolveTargetTriple('linux', 'x64', false)).toBe(
            'x86_64-unknown-linux-musl'
        )
    })

    it('未対応 platform → "Unsupported platform: ..." をスロー', () => {
        expect(() => resolveTargetTriple('freebsd', 'x64', false)).toThrow(
            'Unsupported platform: freebsd'
        )
    })

    it('未対応 arch → "Unsupported arch: ..." をスロー', () => {
        expect(() => resolveTargetTriple('linux', 'arm64', false)).toThrow(
            'Unsupported arch: linux/arm64'
        )
    })

    it('darwin では isGlibc フラグを無視して arm64 triple を返す', () => {
        expect(resolveTargetTriple('darwin', 'arm64', true)).toBe(
            'aarch64-apple-darwin'
        )
    })
})

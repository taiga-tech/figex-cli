import path from 'node:path'
import { afterEach, describe, expect, it, vi } from 'vitest'

const familySyncMock = vi.fn<() => string | null>()
const spawnSyncMock = vi.fn()
const resolvePackageNameMock =
    vi.fn<(platform: string, arch: string, isGlibc: boolean) => string>()
const resolveTargetTripleMock =
    vi.fn<(platform: string, arch: string, isGlibc: boolean) => string>()

vi.mock('detect-libc', () => ({
    familySync: familySyncMock,
}))

vi.mock('node:child_process', () => ({
    spawnSync: spawnSyncMock,
}))

vi.mock('./platform', () => ({
    resolvePackageName: resolvePackageNameMock,
    resolveTargetTriple: resolveTargetTripleMock,
}))

const originalPlatformDescriptor = Object.getOwnPropertyDescriptor(
    process,
    'platform'
)
const originalArchDescriptor = Object.getOwnPropertyDescriptor(process, 'arch')
const originalArgvDescriptor = Object.getOwnPropertyDescriptor(process, 'argv')

type SpawnResult = {
    error?: Error
    status?: number | null
}

type SetupOptions = {
    arch?: string
    argv?: string[]
    libcFamily?: string | null
    packageName?: string
    platform?: string
    spawnResult?: SpawnResult
    targetTriple?: string
}

function setProcessProperty(
    key: 'platform' | 'arch' | 'argv',
    value: string | string[]
): void {
    Object.defineProperty(process, key, {
        configurable: true,
        value,
    })
}

function restoreProcessProperties(): void {
    if (originalPlatformDescriptor !== undefined) {
        Object.defineProperty(process, 'platform', originalPlatformDescriptor)
    }

    if (originalArchDescriptor !== undefined) {
        Object.defineProperty(process, 'arch', originalArchDescriptor)
    }

    if (originalArgvDescriptor !== undefined) {
        Object.defineProperty(process, 'argv', originalArgvDescriptor)
    }
}

function setupLauncher(options: SetupOptions = {}) {
    setProcessProperty('platform', options.platform ?? 'linux')
    setProcessProperty('arch', options.arch ?? 'x64')
    setProcessProperty(
        'argv',
        options.argv ?? ['node', 'sample-cli', '--flag', 'value']
    )

    familySyncMock.mockReturnValue(
        options.libcFamily === undefined ? 'glibc' : options.libcFamily
    )
    resolvePackageNameMock.mockReturnValue(
        options.packageName ?? '@taiga-tech/cli-linux-x64-gnu'
    )
    resolveTargetTripleMock.mockReturnValue(
        options.targetTriple ?? 'x86_64-unknown-linux-gnu'
    )

    spawnSyncMock.mockReturnValue(options.spawnResult ?? { status: 0 })

    const stderrWriteSpy = vi
        .spyOn(process.stderr, 'write')
        .mockImplementation(() => true)

    const exitSpy = vi
        .spyOn(process, 'exit')
        .mockImplementation((code?: string | number | null): never => {
            throw new Error(`process.exit:${String(code)}`)
        })

    return { exitSpy, stderrWriteSpy }
}

async function runLauncher(): Promise<void> {
    vi.resetModules()
    await import('./index')
}

afterEach(() => {
    vi.restoreAllMocks()
    vi.resetModules()

    familySyncMock.mockReset()
    spawnSyncMock.mockReset()
    resolvePackageNameMock.mockReset()
    resolveTargetTripleMock.mockReset()

    restoreProcessProperties()
})

describe('index launcher', () => {
    it('glibc 環境では isGlibc=true で解決し status を process.exit に伝播する', async () => {
        // Given
        const { exitSpy } = setupLauncher({
            libcFamily: 'glibc',
            spawnResult: { status: 23 },
        })

        // When
        await expect(runLauncher()).rejects.toThrow('process.exit:23')

        // Then
        expect(resolvePackageNameMock).toHaveBeenCalledWith(
            'linux',
            'x64',
            true
        )
        expect(resolveTargetTripleMock).toHaveBeenCalledWith(
            'linux',
            'x64',
            true
        )

        const pkgDir = path.dirname(
            require.resolve('@taiga-tech/cli-linux-x64-gnu/package.json')
        )
        const expectedBinaryPath = path.join(
            pkgDir,
            'vendor',
            'x86_64-unknown-linux-gnu',
            'sample-cli',
            'sample-cli'
        )

        expect(spawnSyncMock).toHaveBeenCalledWith(
            expectedBinaryPath,
            ['--flag', 'value'],
            { stdio: 'inherit' }
        )
        expect(exitSpy).toHaveBeenCalledWith(23)
    })

    it('win32 では実行ファイル名に .exe を付与して実行する', async () => {
        // Given
        setupLauncher({
            libcFamily: null,
            packageName: '@taiga-tech/cli-win32-x64',
            platform: 'win32',
            spawnResult: { status: 0 },
            targetTriple: 'x86_64-pc-windows-msvc',
        })

        // When
        await expect(runLauncher()).rejects.toThrow('process.exit:0')

        // Then
        const pkgDir = path.dirname(
            require.resolve('@taiga-tech/cli-win32-x64/package.json')
        )
        const expectedBinaryPath = path.join(
            pkgDir,
            'vendor',
            'x86_64-pc-windows-msvc',
            'sample-cli',
            'sample-cli.exe'
        )

        expect(spawnSyncMock).toHaveBeenCalledWith(
            expectedBinaryPath,
            ['--flag', 'value'],
            { stdio: 'inherit' }
        )
    })

    it('resolvePackageName が例外を投げた場合は spawnSync を呼ばず失敗する', async () => {
        // Given
        const { exitSpy } = setupLauncher()
        resolvePackageNameMock.mockImplementationOnce(() => {
            throw new Error('unsupported platform/arch')
        })

        // When
        await expect(runLauncher()).rejects.toThrow('unsupported platform/arch')

        // Then
        expect(spawnSyncMock).not.toHaveBeenCalled()
        expect(exitSpy).not.toHaveBeenCalled()
    })

    it('resolveTargetTriple が例外を投げた場合は spawnSync を呼ばず失敗する', async () => {
        // Given
        const { exitSpy } = setupLauncher()
        resolveTargetTripleMock.mockImplementationOnce(() => {
            throw new Error('unsupported target triple')
        })

        // When
        await expect(runLauncher()).rejects.toThrow('unsupported target triple')

        // Then
        expect(spawnSyncMock).not.toHaveBeenCalled()
        expect(exitSpy).not.toHaveBeenCalled()
    })

    it('package 解決に失敗した場合はエラーメッセージを出して exit(1) する', async () => {
        // Given
        const { exitSpy, stderrWriteSpy } = setupLauncher({
            packageName: '@taiga-tech/cli-package-not-installed',
        })

        // When
        await expect(runLauncher()).rejects.toThrow('process.exit:1')

        // Then
        expect(stderrWriteSpy).toHaveBeenCalledWith(
            expect.stringContaining(
                'Failed to resolve package @taiga-tech/cli-package-not-installed. Is the platform package installed?'
            )
        )
        expect(stderrWriteSpy).toHaveBeenCalledWith(
            expect.stringContaining('Error:')
        )
        expect(spawnSyncMock).not.toHaveBeenCalled()
        expect(exitSpy).toHaveBeenCalledWith(1)
    })

    it('spawnSync が error を返した場合はメッセージを出して exit(1) する', async () => {
        // Given
        const { exitSpy, stderrWriteSpy } = setupLauncher({
            packageName: '@taiga-tech/cli-linux-x64-gnu',
            spawnResult: {
                error: new Error('spawn failed'),
                status: null,
            },
        })

        // When
        await expect(runLauncher()).rejects.toThrow('process.exit:1')

        // Then
        const pkgDir = path.dirname(
            require.resolve('@taiga-tech/cli-linux-x64-gnu/package.json')
        )
        const expectedBinaryPath = path.join(
            pkgDir,
            'vendor',
            'x86_64-unknown-linux-gnu',
            'sample-cli',
            'sample-cli'
        )
        expect(stderrWriteSpy).toHaveBeenCalledWith(
            expect.stringContaining(
                `Failed to execute ${expectedBinaryPath}: spawn failed`
            )
        )
        expect(stderrWriteSpy).toHaveBeenCalledWith(
            expect.stringContaining('spawn failed')
        )
        expect(exitSpy).toHaveBeenCalledWith(1)
    })

    it('spawnSync が error と status=0 を返しても exit(1) を優先する', async () => {
        // Given
        const { exitSpy } = setupLauncher({
            spawnResult: {
                error: new Error('spawn failed with zero status'),
                status: 0,
            },
        })

        // When
        await expect(runLauncher()).rejects.toThrow('process.exit:1')

        // Then
        expect(exitSpy).toHaveBeenCalledTimes(1)
        expect(exitSpy).toHaveBeenCalledWith(1)
        expect(exitSpy).not.toHaveBeenCalledWith(0)
    })

    it('spawnSync.status が null の場合は exit(1) する', async () => {
        // Given
        const { exitSpy } = setupLauncher({
            spawnResult: { status: null },
        })

        // When
        await expect(runLauncher()).rejects.toThrow('process.exit:1')

        // Then
        expect(exitSpy).toHaveBeenCalledWith(1)
    })

    it('spawnSync.status が undefined の場合は exit(1) する', async () => {
        // Given
        const { exitSpy } = setupLauncher({
            spawnResult: {},
        })

        // When
        await expect(runLauncher()).rejects.toThrow('process.exit:1')

        // Then
        expect(exitSpy).toHaveBeenCalledWith(1)
    })

    it('glibc 以外では isGlibc=false を resolver に渡す', async () => {
        // Given
        setupLauncher({
            libcFamily: 'musl',
            packageName: '@taiga-tech/cli-linux-x64-musl',
            spawnResult: { status: 0 },
            targetTriple: 'x86_64-unknown-linux-musl',
        })

        // When
        await expect(runLauncher()).rejects.toThrow('process.exit:0')

        // Then
        expect(resolvePackageNameMock).toHaveBeenCalledWith(
            'linux',
            'x64',
            false
        )
        expect(resolveTargetTripleMock).toHaveBeenCalledWith(
            'linux',
            'x64',
            false
        )
    })

    it('familySync が null の場合は isGlibc=false を resolver に渡す', async () => {
        // Given
        setupLauncher({
            libcFamily: null,
            packageName: '@taiga-tech/cli-linux-x64-musl',
            spawnResult: { status: 0 },
            targetTriple: 'x86_64-unknown-linux-musl',
        })

        // When
        await expect(runLauncher()).rejects.toThrow('process.exit:0')

        // Then
        expect(resolvePackageNameMock).toHaveBeenCalledWith(
            'linux',
            'x64',
            false
        )
        expect(resolveTargetTripleMock).toHaveBeenCalledWith(
            'linux',
            'x64',
            false
        )
    })

    it('追加CLI引数がない場合は spawnSync に空配列を渡す', async () => {
        // Given
        setupLauncher({
            argv: ['node', 'sample-cli'],
            spawnResult: { status: 0 },
        })

        // When
        await expect(runLauncher()).rejects.toThrow('process.exit:0')

        // Then
        const pkgDir = path.dirname(
            require.resolve('@taiga-tech/cli-linux-x64-gnu/package.json')
        )
        const expectedBinaryPath = path.join(
            pkgDir,
            'vendor',
            'x86_64-unknown-linux-gnu',
            'sample-cli',
            'sample-cli'
        )
        expect(spawnSyncMock).toHaveBeenCalledWith(expectedBinaryPath, [], {
            stdio: 'inherit',
        })
    })

    it('正常終了時は process.stderr.write を呼び出さない', async () => {
        // Given
        const { stderrWriteSpy } = setupLauncher({
            spawnResult: { status: 0 },
        })

        // When
        await expect(runLauncher()).rejects.toThrow('process.exit:0')

        // Then
        expect(stderrWriteSpy).not.toHaveBeenCalled()
    })
})

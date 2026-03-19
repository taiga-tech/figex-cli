const SCOPE = '@taiga-tech' as const

const PLATFORM_ENTRIES: Readonly<
    Record<string, { pkg: string; triple: string }>
> = {
    'darwin/arm64': {
        pkg: `${SCOPE}/cli-darwin-arm64`,
        triple: 'aarch64-apple-darwin',
    },
    'darwin/x64': {
        pkg: `${SCOPE}/cli-darwin-x64`,
        triple: 'x86_64-apple-darwin',
    },
    'win32/x64': {
        pkg: `${SCOPE}/cli-win32-x64`,
        triple: 'x86_64-pc-windows-msvc',
    },
    'linux/x64/gnu': {
        pkg: `${SCOPE}/cli-linux-x64-gnu`,
        triple: 'x86_64-unknown-linux-gnu',
    },
    'linux/x64/musl': {
        pkg: `${SCOPE}/cli-linux-x64-musl`,
        triple: 'x86_64-unknown-linux-musl',
    },
}

const SUPPORTED_PLATFORMS = new Set(['darwin', 'win32', 'linux'])

function buildKey(platform: string, arch: string, isGlibc: boolean): string {
    const libcSuffix =
        platform === 'linux' ? `/${isGlibc ? 'gnu' : 'musl'}` : ''
    return `${platform}/${arch}${libcSuffix}`
}

function resolveEntry(
    platform: string,
    arch: string,
    isGlibc: boolean
): { pkg: string; triple: string } {
    if (!SUPPORTED_PLATFORMS.has(platform)) {
        throw new Error(`Unsupported platform: ${platform}`)
    }

    const key = buildKey(platform, arch, isGlibc)
    const entry = PLATFORM_ENTRIES[key]

    if (entry === undefined) {
        throw new Error(`Unsupported arch: ${platform}/${arch}`)
    }

    return entry
}

export function resolvePackageName(
    platform: string,
    arch: string,
    isGlibc: boolean
): string {
    return resolveEntry(platform, arch, isGlibc).pkg
}

export function resolveTargetTriple(
    platform: string,
    arch: string,
    isGlibc: boolean
): string {
    return resolveEntry(platform, arch, isGlibc).triple
}

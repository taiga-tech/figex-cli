import { familySync } from 'detect-libc'
import { spawnSync } from 'node:child_process'
import path from 'node:path'

import { resolvePackageName, resolveTargetTriple } from './platform'

const BINARY_NAME = 'sample-cli' as const

function main(): void {
    const { platform, arch } = process
    const isGlibc = familySync() === 'glibc'

    const pkg = resolvePackageName(platform, arch, isGlibc)
    const triple = resolveTargetTriple(platform, arch, isGlibc)

    let pkgDir: string
    try {
        const pkgJsonPath = require.resolve(`${pkg}/package.json`)
        pkgDir = path.dirname(pkgJsonPath)
    } catch (err) {
        process.stderr.write(
            `Failed to resolve package ${pkg}. Is the platform package installed?\n${err}\n`
        )
        process.exit(1)
    }

    const binaryFilename =
        platform === 'win32' ? `${BINARY_NAME}.exe` : BINARY_NAME
    const binaryPath = path.join(
        pkgDir,
        'vendor',
        triple,
        BINARY_NAME,
        binaryFilename
    )

    const result = spawnSync(binaryPath, process.argv.slice(2), {
        stdio: 'inherit',
    })

    if (result.error) {
        process.stderr.write(
            `Failed to execute ${binaryPath}: ${result.error.message}\n`
        )
        process.exit(1)
    }

    process.exit(result.status ?? 1)
}

main()

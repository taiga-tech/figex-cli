import path from 'node:path'

type PackageJsonResolver = (request: string) => string

export function resolvePackageDir(
    packageName: string,
    resolvePackageJsonPath: PackageJsonResolver = require.resolve
): string {
    const packageJsonPath = resolvePackageJsonPath(
        `${packageName}/package.json`
    )
    return path.dirname(packageJsonPath)
}

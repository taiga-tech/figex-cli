import path from 'node:path'
import { describe, expect, it } from 'vitest'

import { resolvePackageDir } from './package'

describe('resolvePackageDir', () => {
    it('package.json への解決結果から親ディレクトリを返す', () => {
        const packageName = '@taiga-tech/figex-cli-win32-x64'
        const packageJsonPath = path.join(
            '/virtual',
            '@taiga-tech',
            'figex-cli-win32-x64',
            'package.json'
        )
        const resolver = (request: string) => {
            expect(request).toBe(`${packageName}/package.json`)
            return packageJsonPath
        }

        expect(resolvePackageDir(packageName, resolver)).toBe(
            path.dirname(packageJsonPath)
        )
    })

    it('package 解決エラーをそのまま再送出する', () => {
        const resolver = () => {
            throw new Error('Cannot find module')
        }

        expect(() =>
            resolvePackageDir(
                '@taiga-tech/figex-cli-package-not-installed',
                resolver
            )
        ).toThrow('Cannot find module')
    })
})

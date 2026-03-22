import { defineConfig } from 'vitest/config'

export default defineConfig({
    test: {
        coverage: {
            all: true,
            include: ['src/**/*.ts'],
            exclude: ['src/**/*.test.ts'],
            provider: 'v8',
            reporter: ['text', 'json-summary'],
            thresholds: {
                branches: 100,
                functions: 100,
                lines: 100,
                statements: 100,
            },
        },
    },
})

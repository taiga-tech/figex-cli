#!/usr/bin/env node

import fs from 'node:fs'

const ROOT_HINTS = new Set(['packages', 'crates', 'apps', 'docs', 'scripts'])

function usage() {
    console.error(
        'Usage: node compare_coverage_json.mjs <baseline.json> <candidate.json>'
    )
}

function fail(message, exitCode = 2) {
    console.error(message)
    process.exit(exitCode)
}

function readJson(filePath) {
    try {
        return JSON.parse(fs.readFileSync(filePath, 'utf8'))
    } catch (error) {
        fail(
            `Failed to read ${filePath}: ${
                error instanceof Error ? error.message : String(error)
            }`
        )
    }
}

function toNumber(value) {
    return typeof value === 'number' && Number.isFinite(value) ? value : null
}

function normalizePath(filePath) {
    const normalized = String(filePath).replaceAll('\\', '/')
    const parts = normalized.split('/').filter(Boolean)
    const startIndex = parts.findIndex((part) => ROOT_HINTS.has(part))

    if (startIndex >= 0) {
        return parts.slice(startIndex).join('/')
    }

    return normalized
}

function extractMetrics(container, metricNames, pctKey) {
    const metrics = {}

    for (const metricName of metricNames) {
        const value = toNumber(container?.[metricName]?.[pctKey])
        if (value !== null) {
            metrics[metricName] = value
        }
    }

    return metrics
}

function extractVitestSummary(data) {
    const totals = extractMetrics(
        data.total,
        ['lines', 'statements', 'functions', 'branches'],
        'pct'
    )
    const files = new Map()

    for (const [rawPath, fileSummary] of Object.entries(data)) {
        if (rawPath === 'total') {
            continue
        }

        const metrics = extractMetrics(
            fileSummary,
            ['lines', 'statements', 'functions', 'branches'],
            'pct'
        )

        if (Object.keys(metrics).length > 0) {
            files.set(normalizePath(rawPath), metrics)
        }
    }

    return {
        format: 'vitest-json-summary',
        totals,
        files,
    }
}

function extractRustSummary(data) {
    const report = Array.isArray(data.data) ? data.data[0] : undefined
    const totals = extractMetrics(
        report?.totals,
        ['lines', 'functions', 'regions'],
        'percent'
    )
    const files = new Map()

    for (const fileEntry of report?.files ?? []) {
        const rawPath =
            fileEntry.filename ?? fileEntry.path ?? fileEntry.name ?? null
        const summary = fileEntry.summary ?? fileEntry.totals ?? {}
        const metrics = extractMetrics(
            summary,
            ['lines', 'functions', 'regions'],
            'percent'
        )

        if (rawPath !== null && Object.keys(metrics).length > 0) {
            files.set(normalizePath(rawPath), metrics)
        }
    }

    return {
        format: 'cargo-llvm-cov-summary',
        totals,
        files,
    }
}

function parseSummary(filePath) {
    const data = readJson(filePath)

    if (data?.total?.lines?.pct !== undefined) {
        return extractVitestSummary(data)
    }

    if (
        Array.isArray(data?.data) &&
        data.data[0]?.totals?.lines?.percent !== undefined
    ) {
        return extractRustSummary(data)
    }

    fail(`Unsupported coverage format: ${filePath}`)
}

function formatPct(value) {
    return `${value.toFixed(1)}%`
}

function compareMetrics(scope, baselineMetrics, candidateMetrics) {
    const regressions = []
    const gaps = []
    const metricNames = new Set([
        ...Object.keys(baselineMetrics),
        ...Object.keys(candidateMetrics),
    ])

    for (const metricName of metricNames) {
        const baselineValue = baselineMetrics[metricName]
        const candidateValue = candidateMetrics[metricName]

        if (candidateValue === undefined) {
            regressions.push(
                `${scope} ${metricName}: missing in candidate (baseline ${formatPct(
                    baselineValue
                )})`
            )
            continue
        }

        if (
            baselineValue !== undefined &&
            candidateValue + Number.EPSILON < baselineValue
        ) {
            regressions.push(
                `${scope} ${metricName}: ${formatPct(
                    baselineValue
                )} -> ${formatPct(candidateValue)}`
            )
        }

        if (candidateValue + Number.EPSILON < 100) {
            gaps.push(`${scope} ${metricName}: ${formatPct(candidateValue)}`)
        }
    }

    return { gaps, regressions }
}

function main() {
    const [, , baselinePath, candidatePath] = process.argv

    if (!baselinePath || !candidatePath) {
        usage()
        process.exit(2)
    }

    const baseline = parseSummary(baselinePath)
    const candidate = parseSummary(candidatePath)

    if (baseline.format !== candidate.format) {
        fail(
            `Coverage format mismatch: ${baseline.format} vs ${candidate.format}`
        )
    }

    const overall = compareMetrics('total', baseline.totals, candidate.totals)
    const fileNotes = []
    const allFiles = new Set([
        ...baseline.files.keys(),
        ...candidate.files.keys(),
    ])

    for (const filePath of [...allFiles].sort()) {
        const baselineMetrics = baseline.files.get(filePath) ?? {}
        const candidateMetrics = candidate.files.get(filePath) ?? {}
        const result = compareMetrics(
            filePath,
            baselineMetrics,
            candidateMetrics
        )

        if (result.regressions.length > 0 || result.gaps.length > 0) {
            fileNotes.push(...result.regressions, ...result.gaps)
        }
    }

    console.log(`Format: ${baseline.format}`)
    console.log(`Baseline: ${baselinePath}`)
    console.log(`Candidate: ${candidatePath}`)
    console.log('')

    if (
        overall.regressions.length === 0 &&
        overall.gaps.length === 0 &&
        fileNotes.length === 0
    ) {
        console.log(
            'No regressions found. Candidate coverage matches or exceeds the baseline, and all reported metrics are 100.0%.'
        )
        process.exit(0)
    }

    if (overall.regressions.length > 0) {
        console.log('Overall regressions:')
        for (const line of overall.regressions) {
            console.log(`- ${line}`)
        }
        console.log('')
    }

    const uniqueGaps = [...new Set([...overall.gaps, ...fileNotes])]

    console.log('Coverage gaps to fix:')
    for (const line of uniqueGaps) {
        console.log(`- ${line}`)
    }

    process.exit(1)
}

main()

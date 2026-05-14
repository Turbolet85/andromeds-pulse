# Chunk #54 perf SLO regression detector — PowerShell variant.
# Tails agent-latest.jsonl for metric.webgpu.frame_duration_ms events;
# asserts p99 .fields.duration_ms ≤33ms. Sibling check:
# metric.buffer.memory_bytes max .fields.value ≤512_000_000.
# Empty event streams map к NEUTRAL (exit 0).

$ErrorActionPreference = 'Stop'

if ($args.Count -lt 1) {
    Write-Error "perf-slo-check: missing log file argument"
    exit 1
}
$logPath = $args[0]
if (-not (Test-Path -LiteralPath $logPath)) {
    Write-Host "perf-slo-check: log file $logPath not found; treating as NEUTRAL"
    exit 0
}

$frameSamples = New-Object System.Collections.Generic.List[double]
$memSamples = New-Object System.Collections.Generic.List[double]

Get-Content -LiteralPath $logPath -Encoding UTF8 | ForEach-Object {
    $line = $_.Trim()
    if ([string]::IsNullOrEmpty($line)) { return }
    try {
        $rec = $line | ConvertFrom-Json -ErrorAction Stop
    } catch {
        return
    }
    if ($rec.target -eq "metric.webgpu.frame_duration_ms") {
        $val = $rec.fields.duration_ms
        if ($null -ne $val) {
            [void]$frameSamples.Add([double]$val)
        }
    } elseif ($rec.target -eq "metric.buffer.memory_bytes") {
        $val = $rec.fields.value
        if ($null -ne $val) {
            [void]$memSamples.Add([double]$val)
        }
    }
}

if ($frameSamples.Count -eq 0) {
    Write-Host "perf-slo-check: zero frame_duration_ms events; NEUTRAL (webview not booted during load OR observability not subscribed)"
} else {
    $sorted = $frameSamples | Sort-Object
    $count = $sorted.Count
    $idx = [int]([math]::Ceiling($count * 0.99)) - 1
    if ($idx -lt 0) { $idx = 0 }
    if ($idx -ge $count) { $idx = $count - 1 }
    $p99 = $sorted[$idx]
    if ($p99 -le 33.0) {
        Write-Host "perf-slo-check: frame_duration_ms p99 = ${p99}ms ≤ 33ms (PASS; n=$count)"
    } else {
        Write-Error "perf-slo-check: frame_duration_ms p99 = ${p99}ms > 33ms (FAIL; n=$count)"
        exit 1
    }
}

if ($memSamples.Count -eq 0) {
    Write-Host "perf-slo-check: zero buffer.memory_bytes events; NEUTRAL (heartbeat not running)"
} else {
    $memMax = ($memSamples | Measure-Object -Maximum).Maximum
    if ($memMax -le 512000000) {
        Write-Host "perf-slo-check: buffer.memory_bytes max = $memMax ≤ 512_000_000 (PASS)"
    } else {
        Write-Error "perf-slo-check: buffer.memory_bytes max = $memMax > 512_000_000 (FAIL)"
        exit 1
    }
}

exit 0

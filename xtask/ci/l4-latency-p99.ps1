# xtask/ci/l4-latency-p99.ps1 — obs-plan §10 SLO gate (L4 inference latency)
#
# Parses metric.pipeline.l4.inference_latency_p99_milliseconds records from
# the JSON log, groups by hardware_profile label, computes p99 per profile,
# and asserts each is at or below the dist-arch v3 §L4 hardware profile
# matrix budget.
#
# INACTIVE state (chunks #82-#83 substrate): when no L4 inference latency
# records are present, exits 0. Activates organically when the first real
# mistralrs binding chunk lands.
#
# Usage: l4-latency-p99.ps1 [<log-path>]

$ErrorActionPreference = 'Stop'

$logArg = if ($args.Count -gt 0) { $args[0] } else { $null }
$targetMetric = 'metric.pipeline.l4.inference_latency_p99_milliseconds'

$budgets = @{
    'gpu_primary'  = if ($env:L4_GPU_PRIMARY_BUDGET_MS)  { [int]$env:L4_GPU_PRIMARY_BUDGET_MS }  else { 5000 }
    'gpu_fallback' = if ($env:L4_GPU_FALLBACK_BUDGET_MS) { [int]$env:L4_GPU_FALLBACK_BUDGET_MS } else { 3000 }
    'cpu_primary'  = if ($env:L4_CPU_PRIMARY_BUDGET_MS)  { [int]$env:L4_CPU_PRIMARY_BUDGET_MS }  else { 30000 }
    'cpu_fallback' = if ($env:L4_CPU_FALLBACK_BUDGET_MS) { [int]$env:L4_CPU_FALLBACK_BUDGET_MS } else { 15000 }
}

$logDir = if ($env:ANDROMEDA_PULSE_DATA_DIR) {
    Join-Path $env:ANDROMEDA_PULSE_DATA_DIR 'logs'
} else {
    $null
}

$logFiles = @()
if ($logArg -and (Test-Path -LiteralPath $logArg -PathType Leaf)) {
    $logFiles = @($logArg)
} elseif ($logDir -and (Test-Path -LiteralPath $logDir -PathType Container)) {
    $logFiles = @(Get-ChildItem -LiteralPath $logDir -Filter 'agent-latest.jsonl*' -File -ErrorAction SilentlyContinue | Sort-Object Name | ForEach-Object { $_.FullName })
}

if ($logFiles.Count -eq 0) {
    Write-Output 'l4-latency-p99: no log file present (INACTIVE state — pre-mistralrs-binding chunk; gate trivially passes)'
    exit 0
}

$samples = @{}
foreach ($file in $logFiles) {
    $reader = [System.IO.StreamReader]::new($file)
    try {
        while (-not $reader.EndOfStream) {
            $line = $reader.ReadLine()
            if ([string]::IsNullOrWhiteSpace($line)) { continue }
            $obj = $null
            try {
                $obj = $line | ConvertFrom-Json -ErrorAction Stop
            } catch {
                continue
            }
            if (-not $obj.target) { continue }
            if ($obj.target -ne $targetMetric) { continue }
            $profile = $obj.fields.hardware_profile
            $ms = $obj.fields.duration_ms
            if (-not $profile -or $null -eq $ms) { continue }
            if (-not $samples.ContainsKey($profile)) { $samples[$profile] = @() }
            $samples[$profile] += [int]$ms
        }
    } finally {
        $reader.Dispose()
    }
}

if ($samples.Count -eq 0) {
    Write-Output "l4-latency-p99: no $targetMetric records found (INACTIVE state — pre-mistralrs-binding chunk; gate trivially passes)"
    exit 0
}

$exitCode = 0
foreach ($profile in $samples.Keys) {
    $arr = $samples[$profile] | Sort-Object
    $n = $arr.Count
    $idx = [int][Math]::Floor($n * 0.99)
    if ($idx -lt 1) { $idx = 1 }
    if ($idx -gt $n) { $idx = $n }
    $p99 = $arr[$idx - 1]
    $budget = $budgets[$profile]
    if (-not $budget) {
        Write-Output "l4-latency-p99: unknown profile label '$profile' (sample_count=$n) — skipping"
        continue
    }
    if ($p99 -gt $budget) {
        Write-Error "::error::l4-latency-p99: profile=$profile p99=${p99}ms exceeds budget ${budget}ms (sample_count=$n)"
        $exitCode = 1
    } else {
        Write-Output "l4-latency-p99: profile=$profile p99=${p99}ms ≤ ${budget}ms (sample_count=$n) PASS"
    }
}

exit $exitCode

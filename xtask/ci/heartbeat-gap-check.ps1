# xtask/ci/heartbeat-gap-check.ps1 — obs-plan §10 SLO gate (heartbeat stall detection)
#
# Parses {ingest|buffer|viz|plugins}.tick records from the JSON log, computes
# consecutive timestamp deltas, and asserts max ≤45000ms.
#
# INACTIVE state (chunk #5/#6): when no tick records are present (Foundation
# epoch — no subsystems exist yet to emit ticks), exits 0. Activates organically
# when first subsystem ships in Epoch 2 (route#16+).
#
# Usage: heartbeat-gap-check.ps1 [<log-path>]

$ErrorActionPreference = 'Stop'

$logArg = if ($args.Count -gt 0) { $args[0] } else { $null }
$thresholdMs = if ($env:HEARTBEAT_GAP_THRESHOLD_MS) { [int]$env:HEARTBEAT_GAP_THRESHOLD_MS } else { 45000 }
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
    Write-Output 'heartbeat-gap-check: no log file present (INACTIVE state — pre-subsystem chunk; gate trivially passes)'
    exit 0
}

$tickTargets = @('ingest.tick', 'buffer.tick', 'viz.tick', 'plugins.tick')
$tickLines = @()
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
            if ($tickTargets -notcontains $obj.target) { continue }
            $ts = $obj.timestamp
            if (-not $ts) { continue }
            $dt = [datetime]::Parse($ts, [System.Globalization.CultureInfo]::InvariantCulture, [System.Globalization.DateTimeStyles]::AdjustToUniversal -bor [System.Globalization.DateTimeStyles]::AssumeUniversal)
            $epochMs = [int64](($dt.ToUniversalTime() - [datetime]::new(1970, 1, 1, 0, 0, 0, [System.DateTimeKind]::Utc)).TotalMilliseconds)
            $tickLines += [pscustomobject]@{ Target = $obj.target; Ms = $epochMs }
        }
    } finally {
        $reader.Dispose()
    }
}

if ($tickLines.Count -eq 0) {
    Write-Output 'heartbeat-gap-check: no {ingest|buffer|viz|plugins}.tick records found (INACTIVE state — pre-subsystem chunk; gate trivially passes)'
    exit 0
}

$grouped = $tickLines | Group-Object -Property Target
$overallMax = 0
$overallTarget = ''
foreach ($group in $grouped) {
    $sorted = $group.Group | Sort-Object Ms
    $prev = $null
    $maxGap = 0
    foreach ($entry in $sorted) {
        if ($null -ne $prev) {
            $gap = $entry.Ms - $prev
            if ($gap -gt $maxGap) { $maxGap = $gap }
        }
        $prev = $entry.Ms
    }
    if ($maxGap -gt $overallMax) {
        $overallMax = $maxGap
        $overallTarget = $group.Name
    }
}

if ($overallMax -gt $thresholdMs) {
    Write-Error "::error::heartbeat-gap-check: max gap $overallMax ms in $overallTarget exceeds $thresholdMs ms threshold"
    exit 1
}

Write-Output "heartbeat-gap-check: max gap $overallMax ms in $overallTarget (threshold $thresholdMs ms) PASS"
exit 0

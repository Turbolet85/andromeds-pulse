# Chunk #56 criterion bench regression detector — PowerShell variant.
# Compares current target/criterion/<bench>/new/estimates.json against
# baseline; uses .mean.point_estimate; fails if mean regresses by more
# than +10% (default; CRITERION_REGRESSION_THRESHOLD_PCT override).
# NEUTRAL when baseline missing.

$ErrorActionPreference = 'Stop'

$currentDir = if ($args.Count -ge 1) { $args[0] } else { "target/criterion" }
$baselineDir = if ($args.Count -ge 2) { $args[1] } else { "" }
$thresholdPct = if ($env:CRITERION_REGRESSION_THRESHOLD_PCT) { [double]$env:CRITERION_REGRESSION_THRESHOLD_PCT } else { 10.0 }

if (-not (Test-Path -LiteralPath $currentDir -PathType Container)) {
    Write-Host "criterion-regression-check: current dir $currentDir not found; NEUTRAL (no criterion bench output yet)"
    exit 0
}

if ([string]::IsNullOrEmpty($baselineDir) -or -not (Test-Path -LiteralPath $baselineDir -PathType Container)) {
    $shown = if ([string]::IsNullOrEmpty($baselineDir)) { "(unset)" } else { $baselineDir }
    Write-Host "criterion-regression-check: baseline $shown not found; NEUTRAL (first PR / new branch / local dev)"
    exit 0
}

$currentRoot = (Resolve-Path -LiteralPath $currentDir).Path
$baselineRoot = (Resolve-Path -LiteralPath $baselineDir).Path

$currentEstimates = Get-ChildItem -Path $currentRoot -Recurse -Filter 'estimates.json' -ErrorAction SilentlyContinue |
    Where-Object { $_.FullName -match '[/\\]new[/\\]' } |
    Sort-Object FullName

if ($currentEstimates.Count -eq 0) {
    Write-Host "criterion-regression-check: no estimates.json found under $currentDir; NEUTRAL (no benches present)"
    exit 0
}

$fail = $false
$checked = 0

foreach ($est in $currentEstimates) {
    $curPath = $est.FullName
    $rel = $curPath.Substring($currentRoot.Length).TrimStart('\','/')
    $basePath = Join-Path $baselineRoot $rel
    if (-not (Test-Path -LiteralPath $basePath)) {
        Write-Host "criterion-regression-check: skipping $rel (no baseline equivalent at $basePath)"
        continue
    }
    try {
        $cur = (Get-Content -LiteralPath $curPath -Raw | ConvertFrom-Json).mean.point_estimate
        $base = (Get-Content -LiteralPath $basePath -Raw | ConvertFrom-Json).mean.point_estimate
    } catch {
        Write-Host "criterion-regression-check: skipping $rel (mean.point_estimate missing or malformed)"
        continue
    }
    if ($null -eq $cur -or $null -eq $base) {
        Write-Host "criterion-regression-check: skipping $rel (mean.point_estimate is null)"
        continue
    }
    $benchName = $rel -replace '[/\\]new[/\\]estimates\.json$', ''
    if ($base -eq 0) {
        $pctChange = 0.0
    } else {
        $pctChange = [math]::Round((($cur - $base) / $base) * 100.0, 2)
    }
    $checked++
    if ($pctChange -gt $thresholdPct) {
        Write-Error "criterion-regression-check: $benchName regressed by ${pctChange}% (current mean ${cur}ns vs baseline ${base}ns; threshold ${thresholdPct}%)"
        $fail = $true
    } else {
        Write-Host "  ${benchName}: mean ${cur}ns vs baseline ${base}ns (${pctChange}% change; threshold ${thresholdPct}%) PASS"
    }
}

if ($checked -eq 0) {
    Write-Host "criterion-regression-check: NEUTRAL (no benches with matching baseline)"
    exit 0
}

if ($fail) { exit 1 }
Write-Host "criterion-regression-check: PASS ($checked bench(es) within ${thresholdPct}% threshold)"
exit 0

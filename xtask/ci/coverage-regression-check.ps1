# Chunk #55 coverage regression detector — PowerShell variant.
# Compares current lcov.info against base-branch lcov-baseline.info;
# computes line/branch/function delta. Fails if any metric regresses by
# more than the threshold (default +0.0pp). NEUTRAL when baseline missing.

$ErrorActionPreference = 'Stop'

$current = if ($args.Count -ge 1) { $args[0] } else { "lcov.info" }
$baseline = if ($args.Count -ge 2) { $args[1] } else { "" }
$thresholdPp = if ($env:COVERAGE_REGRESSION_THRESHOLD_PP) { [double]$env:COVERAGE_REGRESSION_THRESHOLD_PP } else { 0.0 }

if (-not (Test-Path -LiteralPath $current)) {
    Write-Host "coverage-regression-check: current lcov.info $current not found; treating as NEUTRAL"
    exit 0
}

if ([string]::IsNullOrEmpty($baseline) -or -not (Test-Path -LiteralPath $baseline)) {
    $shown = if ([string]::IsNullOrEmpty($baseline)) { "(unset)" } else { $baseline }
    Write-Host "coverage-regression-check: baseline $shown not found; NEUTRAL (first PR / new branch / local dev)"
    exit 0
}

function Sum-Counters {
    param([string]$path)
    $lf = 0; $lh = 0; $brf = 0; $brh = 0; $fnf = 0; $fnh = 0
    Get-Content -LiteralPath $path -Encoding UTF8 | ForEach-Object {
        if ($_ -match '^LF:(\d+)') { $lf += [int]$Matches[1] }
        elseif ($_ -match '^LH:(\d+)') { $lh += [int]$Matches[1] }
        elseif ($_ -match '^BRF:(\d+)') { $brf += [int]$Matches[1] }
        elseif ($_ -match '^BRH:(\d+)') { $brh += [int]$Matches[1] }
        elseif ($_ -match '^FNF:(\d+)') { $fnf += [int]$Matches[1] }
        elseif ($_ -match '^FNH:(\d+)') { $fnh += [int]$Matches[1] }
    }
    [PSCustomObject]@{
        LineTotal = $lf; LineHit = $lh
        BranchTotal = $brf; BranchHit = $brh
        FnTotal = $fnf; FnHit = $fnh
    }
}

function Get-Pct {
    param([int]$hit, [int]$total)
    if ($total -eq 0) { return 100.0 }
    return [math]::Round(($hit / $total) * 100.0, 2)
}

$cur = Sum-Counters $current
$base = Sum-Counters $baseline

$curLinePct = Get-Pct $cur.LineHit $cur.LineTotal
$curBranchPct = Get-Pct $cur.BranchHit $cur.BranchTotal
$curFnPct = Get-Pct $cur.FnHit $cur.FnTotal
$baseLinePct = Get-Pct $base.LineHit $base.LineTotal
$baseBranchPct = Get-Pct $base.BranchHit $base.BranchTotal
$baseFnPct = Get-Pct $base.FnHit $base.FnTotal

Write-Host "coverage-regression-check: threshold=${thresholdPp}pp"
Write-Host "  Line:     current=${curLinePct}% baseline=${baseLinePct}%"
Write-Host "  Branch:   current=${curBranchPct}% baseline=${baseBranchPct}%"
Write-Host "  Function: current=${curFnPct}% baseline=${baseFnPct}%"

$fail = $false
function Test-Regression {
    param([string]$metric, [double]$cur, [double]$base, [double]$threshold)
    $regression = $base - $cur
    if ($regression -gt $threshold) {
        Write-Error "coverage-regression-check: ${metric} regressed by $($regression.ToString('F2'))pp (current ${cur}% vs baseline ${base}%; threshold ${threshold}pp)"
        return $true
    }
    return $false
}

if (Test-Regression "line" $curLinePct $baseLinePct $thresholdPp) { $fail = $true }
if (Test-Regression "branch" $curBranchPct $baseBranchPct $thresholdPp) { $fail = $true }
if (Test-Regression "function" $curFnPct $baseFnPct $thresholdPp) { $fail = $true }

if (-not $fail) {
    Write-Host "coverage-regression-check: PASS (no metric regressed by more than ${thresholdPp}pp)"
    exit 0
}
exit 1

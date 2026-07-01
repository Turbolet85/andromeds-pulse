# scripts/free-disk.ps1 - reclaim disk from regenerable Rust/build/log artifacts (dev env)
#
# SAFE BY DEFAULT: no args = measure + dry-run only (deletes nothing). Pass -Execute to delete.
#   .\scripts\free-disk.ps1            # preview what would be freed
#   .\scripts\free-disk.ps1 -Execute   # actually reclaim
#
# Removes (all regenerable): target\debug\incremental, <cargo-home>\registry\{cache,src},
#   target\doc, experiments\llamacpp_spike\target, and %APPDATA%\andromeda-pulse\logs\
#   agent-latest.jsonl.* older than 2 days (keeps the last 2 days).
# NEVER touches: target\debug\deps (warm current-toolchain artifacts), the corpus/DB,
#   .andromeda\ state, pulse-app\ui\dist, registry\index, source, the last-2-days logs.
#
# Deeper reclaim from target\debug\deps after toolchain churn (safe, keeps current warm):
#   cargo install cargo-sweep ; cargo sweep --installed
#
# NOTE: an agent cannot run -Execute (PowerShell / recursive-rm deny rules). The routine is:
#   agent runs the default dry-run to MEASURE, hands the -Execute line to the user, the user
#   reclaims, the agent re-runs the dry-run to verify freed space + preserve-list.

param([switch]$Execute)

$ErrorActionPreference = 'Continue'  # resilience: skip a locked file, never halt mid-clean

$Repo      = Split-Path $PSScriptRoot -Parent
$CargoHome = if ($env:CARGO_HOME) { $env:CARGO_HOME } elseif (Test-Path 'D:\dev\rust\cargo') { 'D:\dev\rust\cargo' } else { Join-Path $env:USERPROFILE '.cargo' }
$LogDir    = Join-Path $env:APPDATA 'andromeda-pulse\logs'

$DirTargets = @(
    (Join-Path $Repo      'target\debug\incremental'),
    (Join-Path $CargoHome 'registry\cache'),
    (Join-Path $CargoHome 'registry\src'),
    (Join-Path $Repo      'target\doc'),
    (Join-Path $Repo      'experiments\llamacpp_spike\target')
)
$Preserve = @(
    (Join-Path $Repo      'target\debug\deps'),
    (Join-Path $Repo      'pulse-app\ui\dist'),
    (Join-Path $Repo      '.andromeda'),
    (Join-Path $CargoHome 'registry\index'),
    (Join-Path $env:APPDATA 'andromeda-pulse\corpus')
)

function Get-DirGB($p) {
    if (Test-Path $p) { (Get-ChildItem $p -Recurse -Force -ErrorAction SilentlyContinue | Measure-Object Length -Sum).Sum / 1GB }
    else { -1 }
}
function Show-Free($label) {
    Write-Output "  [$label] free space:"
    Get-PSDrive C,D | Select-Object Name, @{n='FreeGB';e={[math]::Round($_.Free/1GB,1)}} | Format-Table -AutoSize
}

$Mode = 'DRY-RUN (pass -Execute to delete)'
if ($Execute) { $Mode = 'EXECUTE' }
Write-Output "=== free-disk.ps1  [$Mode] ==="
Show-Free 'before'

Write-Output '--- Regenerable dir targets ---'
foreach ($t in $DirTargets) {
    $gb = Get-DirGB $t
    if ($gb -lt 0) { Write-Output ("  absent           {0}" -f $t); continue }
    if ($Execute) {
        Remove-Item -Recurse -Force $t -ErrorAction SilentlyContinue
        $state = 'DELETED'
        if (Test-Path $t) { $state = 'LOCKED/PARTIAL' }
        Write-Output ("  {0,-14} {1,7:N2} GB  {2}" -f $state, $gb, $t)
    } else {
        Write-Output ("  would free     {0,7:N2} GB  {1}" -f $gb, $t)
    }
}

Write-Output '--- Old rotated logs (older than 2 days) ---'
$OldLogs = @(Get-ChildItem "$LogDir\agent-latest.jsonl.*" -ErrorAction SilentlyContinue | Where-Object { $_.LastWriteTime -lt (Get-Date).AddDays(-2) })
if ($OldLogs.Count -eq 0) { Write-Output '  (none older than 2 days)' }
foreach ($l in $OldLogs) {
    if ($Execute) {
        Remove-Item $l.FullName -Force -ErrorAction SilentlyContinue
        Write-Output ("  DELETED   {0,8:N1} MB  {1}" -f ($l.Length/1MB), $l.Name)
    } else {
        Write-Output ("  would delete {0,8:N1} MB  {1}" -f ($l.Length/1MB), $l.Name)
    }
}

Write-Output '--- Preserve-list (must all exist) ---'
foreach ($p in $Preserve) {
    $state = 'MISSING'
    if (Test-Path $p) { $state = 'OK     ' }
    Write-Output "  $state $p"
}

if ($Execute) { Show-Free 'after' }

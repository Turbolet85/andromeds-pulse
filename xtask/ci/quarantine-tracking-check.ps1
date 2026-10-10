# Quarantine convention enforcement — PowerShell variant.
# Greps for #[ignore] in Rust source; asserts each match has a GitHub
# issue URL in the surrounding 5-line window. A missing search dir fails,
# and so does a scan of zero .rs files; the PASS line says how many files
# were scanned.

$ErrorActionPreference = 'Stop'

$workspaceRoot = if ($args.Count -ge 1) { $args[0] } else { (Get-Location).Path }
$searchDirs = @(
    "crates",
    "pulse-app/src",
    "pulse-app/tests",
    "xtask/src"
)

$missing = $false
foreach ($dir in $searchDirs) {
    if (-not (Test-Path -LiteralPath (Join-Path $workspaceRoot $dir) -PathType Container)) {
        [Console]::Error.WriteLine("::error::quarantine-tracking-check: search dir $dir is missing")
        $missing = $true
    }
}
if ($missing) { exit 1 }

$fileCount = 0
$found = New-Object System.Collections.Generic.List[object]
foreach ($dir in $searchDirs) {
    $files = Get-ChildItem -Path (Join-Path $workspaceRoot $dir) -Recurse -File -Filter '*.rs' -ErrorAction SilentlyContinue
    foreach ($file in $files) {
        $fileCount++
        $lines = Get-Content -LiteralPath $file.FullName -Encoding UTF8
        for ($i = 0; $i -lt $lines.Count; $i++) {
            if ($lines[$i] -match '^\s*#\[ignore') {
                [void]$found.Add([PSCustomObject]@{
                    File = $file.FullName
                    Line = $i + 1
                })
            }
        }
    }
}

if ($fileCount -eq 0) {
    [Console]::Error.WriteLine("::error::quarantine-tracking-check: no .rs file under $($searchDirs -join ' ')")
    exit 1
}

if ($found.Count -eq 0) {
    Write-Host "quarantine-tracking-check: PASS (0 quarantine(s) across $fileCount file(s))"
    exit 0
}

$fail = $false
$pattern = 'https://github\.com/[^/]+/[^/]+/issues/[0-9]+'
foreach ($m in $found) {
    $lines = Get-Content -LiteralPath $m.File -Encoding UTF8
    $startIdx = [math]::Max(0, $m.Line - 3)
    $endIdx = [math]::Min($lines.Count - 1, $m.Line + 4)
    $window = $lines[$startIdx..$endIdx] -join "`n"
    if ($window -notmatch $pattern) {
        Write-Error "quarantine-tracking-check: $($m.File):$($m.Line) — #[ignore] lacks GitHub issue URL in surrounding 5-line window"
        $fail = $true
    }
}

if (-not $fail) {
    Write-Host "quarantine-tracking-check: PASS ($($found.Count) quarantine(s) tracked via GitHub issue URL across $fileCount file(s))"
    exit 0
}
exit 1

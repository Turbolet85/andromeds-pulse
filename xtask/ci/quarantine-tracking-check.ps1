# Chunk #55 quarantine convention enforcement — PowerShell variant.
# Greps for #[ignore] in Rust source; asserts each match has a GitHub
# issue URL in the surrounding 5-line window. NEUTRAL on zero matches.

$ErrorActionPreference = 'Stop'

$workspaceRoot = if ($args.Count -ge 1) { $args[0] } else { (Get-Location).Path }
$searchDirs = @(
    (Join-Path $workspaceRoot "crates"),
    (Join-Path $workspaceRoot "pulse-app/src"),
    (Join-Path $workspaceRoot "pulse-app/tests"),
    (Join-Path $workspaceRoot "xtask/src")
)

$found = New-Object System.Collections.Generic.List[object]
foreach ($dir in $searchDirs) {
    if (-not (Test-Path -LiteralPath $dir)) { continue }
    $files = Get-ChildItem -Path $dir -Recurse -Filter '*.rs' -ErrorAction SilentlyContinue
    foreach ($file in $files) {
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

if ($found.Count -eq 0) {
    Write-Host "quarantine-tracking-check: NEUTRAL (zero #[ignore] in source; gate establishes convention for future quarantines)"
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
    Write-Host "quarantine-tracking-check: PASS ($($found.Count) quarantine(s) tracked via GitHub issue URL)"
    exit 0
}
exit 1

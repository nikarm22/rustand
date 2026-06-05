$ErrorActionPreference = "Stop"

$runtimes = @("single-threaded", "multi-threaded", "tokio", "async-std")
$fuzz_targets = @("single_threaded", "multi_threaded", "tokio", "async_std")

Write-Host "Running tests for all runtimes..." -ForegroundColor Cyan

foreach ($runtime in $runtimes) {
    Write-Host "`n>>> Testing runtime: $runtime" -ForegroundColor Yellow
    cargo test --features $runtime --no-default-features
}

Write-Host "`nChecking for fuzzing tools..." -ForegroundColor Cyan
$has_fuzz = (Get-Command cargo-fuzz -ErrorAction SilentlyContinue) -and (rustup toolchain list | Select-String "nightly")

if ($has_fuzz) {
    Write-Host "Running fuzzers (smoke test)..." -ForegroundColor Cyan
    foreach ($target in $fuzz_targets) {
        $runtime = $target.Replace("_", "-")
        Write-Host "`n>>> Fuzzing target: $target (runtime: $runtime)" -ForegroundColor Yellow
        cargo +nightly fuzz run $target --features $runtime -- -runs=1000
    }
} else {
    Write-Host "`nSkipping fuzzers: cargo-fuzz or nightly toolchain not found." -ForegroundColor Gray
}

Write-Host "`nAll tests completed!" -ForegroundColor Green

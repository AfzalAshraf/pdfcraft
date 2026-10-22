# Keep failing native Cargo gates failing, while making their diagnostics readable via the
# checks API even when GitHub's external log-download host is inaccessible.
param(
  [Parameter(Mandatory)] [string] $Label,
  [Parameter(Mandatory)] [string[]] $CargoArguments
)
$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false
$env:CARGO_TERM_COLOR = 'never'
& cargo @CargoArguments 2>&1 | Tee-Object -Variable diagnostics
$code = $LASTEXITCODE
if ($code -ne 0) {
  $message = ($diagnostics | Select-Object -Last 300 | Out-String)
  # Each annotation is truncated at 4096 characters, so split before escaping.
  for ($offset = 0; $offset -lt $message.Length; $offset += 3000) {
    $chunk = $message.Substring($offset, [Math]::Min(3000, $message.Length - $offset))
    $chunk = $chunk.Replace('%', '%25').Replace("`r", '%0D').Replace("`n", '%0A')
    Write-Output "::error title=$Label diagnostics ${offset}::$chunk"
  }
}
exit $code

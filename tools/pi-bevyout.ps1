[CmdletBinding()]
param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $PiArguments
)

$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
$contextPath = Join-Path $projectRoot '.pi\PI.md'

Push-Location $projectRoot
try {
    $arguments = @(
        '--no-context-files',
        '--approve',
        '--append-system-prompt',
        $contextPath
    )
    $arguments += @($PiArguments)
    & pi @arguments
    exit $LASTEXITCODE
}
finally {
    Pop-Location
}

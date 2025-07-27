# Get the current script's directory
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition

# Define relative paths from the script directory
$indexPath = Join-Path $scriptDir "target\dx\port\release\web\public\index.html"
$assetsPath = Join-Path $scriptDir "target\dx\port\release\web\public\assets"

# Find the correct tailwind CSS file (matches tailwind-*.css)
$tailwindFile = Get-ChildItem -Path $assetsPath -Filter "tailwind-*.css" | Select-Object -First 1

if (-not $tailwindFile) {
    Write-Error "No tailwind-*.css file found in $assetsPath"
    exit
}

# Extract filename
$tailwindFileName = $tailwindFile.Name
$tailwindHref = "/assets/$tailwindFileName"

# Load index.html content
$htmlContent = Get-Content $indexPath -Raw

# Replace old tailwind hrefs with the new one (works with or without 'assets/' prefix)
$htmlContent = $htmlContent -replace '<link rel="stylesheet" href="[^"]*tailwind[^"]*\.css">', "<link rel=`"stylesheet`" href=`"$tailwindHref`">"
$htmlContent = $htmlContent -replace '<link rel="preload" as="style" href="[^"]*tailwind[^"]*\.css">', "<link rel=`"preload`" as=`"style`" href=`"$tailwindHref`">"

# If preload doesn't exist, add it manually after the stylesheet link
if ($htmlContent -notmatch '<link rel="preload" as="style" href="[^"]*tailwind[^"]*\.css">') {
    $htmlContent = $htmlContent -replace "($tailwindHref`">\s*)", "`$1<link rel=`"preload`" as=`"style`" href=`"$tailwindHref`">`n"
}

# Save updated content back
Set-Content -Path $indexPath -Value $htmlContent -Encoding UTF8

Write-Host "✅ index.html updated with correct Tailwind CSS: $tailwindFileName"

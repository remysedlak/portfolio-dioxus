# add_lang.ps1 - Adds lang="en" to index.html after dx build

# Path to your release index.html
$IndexFile = ".\target\dx\port\release\web\public\index.html"

# Check if the file exists
if (-Not (Test-Path $IndexFile)) {
    Write-Host "Error: $IndexFile does not exist. Run 'dx build release' first."
    exit 1
}

# Read the file content
$content = Get-Content $IndexFile

# Check if lang="en" is already present
if ($content -match '<html[^>]*lang="en"') {
    Write-Host 'lang="en" already exists in index.html'
} else {
    # Add lang="en" to the <html> tag
    $newContent = $content -replace '<html', '<html lang="en"'
    # Write back to file
    Set-Content -Path $IndexFile -Value $newContent -Encoding UTF8
    Write-Host 'Added lang="en" to index.html'
}

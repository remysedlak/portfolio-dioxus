# add_style.ps1 - Adds custom <style> to <head> in index.html after dx build

# Path to your release index.html
$IndexFile = ".\target\dx\port\release\web\public\index.html"

# Check if the file exists
if (-Not (Test-Path $IndexFile)) {
    Write-Host "Error: $IndexFile does not exist. Run 'dx build release' first."
    exit 1
}

# Read the file content
$content = Get-Content $IndexFile -Raw

# Check if our style is already present
if ($content -match '<style>\s*html, body\s*{') {
    Write-Host "Custom <style> already exists in index.html"
} else {
    # Insert the <style> block right after <head>
    $styleBlock = @"
<style>
  html, body {
    margin: 0;
    background-color: #111827;
    min-height: 100%;
  }
</style>
"@

    # Replace <head> with <head> + style block
    $newContent = $content -replace '<head>', "<head>`r`n$styleBlock"

    # Write back to file
    Set-Content -Path $IndexFile -Value $newContent -Encoding UTF8
    Write-Host "Added custom <style> to index.html"
}

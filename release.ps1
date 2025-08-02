# release.ps1

# --- Configuration ---
$CargoTomlPath = "./Cargo.toml"
$LicensePath = "./LICENSE"
$ReadmePath = "./README.md"
$ReadmeJaPath = "./README_ja.md"
$ReleaseDir = "./target/release"
$OutputArchiveDir = "./" # Output to the project root

# --- Script ---

# 1. Get package info from Cargo.toml
Write-Host "Reading package info from $CargoTomlPath..."
$cargoTomlContent = Get-Content $CargoTomlPath -Raw

# Get Version
$versionMatch = $cargoTomlContent | Select-String -Pattern 'version = "([0-9]+\.[0-9]+\.[0-9]+)"'
if (-not $versionMatch) {
    Write-Error "Version not found in $CargoTomlPath"
    exit 1
}
$Version = $versionMatch.Matches[0].Groups[1].Value

# Get Package Name
$nameMatch = $cargoTomlContent | Select-String -Pattern 'name = "(.*)"'
if (-not $nameMatch) {
    Write-Error "Package name not found in $CargoTomlPath"
    exit 1
}
$PackageName = $nameMatch.Matches[0].Groups[1].Value
$ProjectName = (Get-Item -Path ".").Name # Keep the original project name for the zip file

Write-Host "Package: $PackageName, Version: $Version"

# 2. Build the project
Write-Host "Running cargo build --release..."
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Cargo build failed."
    exit 1
}

# 3. Prepare files for packaging
$ExeName = "$PackageName.exe" # Use package name for the exe
$ExePath = Join-Path -Path $ReleaseDir -ChildPath $ExeName
$OutputArchiveName = "$($ProjectName)_v$($Version).zip" # Use folder name for the zip
$OutputArchivePath = Join-Path -Path $OutputArchiveDir -ChildPath $OutputArchiveName

# Check if files exist
if (-not (Test-Path $ExePath)) {
    Write-Error "Executable not found at $ExePath"
    exit 1
}
if (-not (Test-Path $LicensePath)) {
    Write-Warning "LICENSE file not found at $LicensePath"
}
if (-not (Test-Path $ReadmePath)) {
    Write-Warning "README.md file not found at $ReadmePath"
}
if (-not (Test-Path $ReadmeJaPath)) { # Corrected variable name
    Write-Warning "README_ja.md file not found at $ReadmeJaPath"
}

# 4. Create the zip archive
Write-Host "Creating archive: $OutputArchivePath"
# Remove old archive if it exists
if (Test-Path $OutputArchivePath) {
    Remove-Item $OutputArchivePath
}

# Create a temporary directory to stage files
$stagingDir = "./staging_for_release"
if (Test-Path $stagingDir) {
    Remove-Item $stagingDir -Recurse -Force
}
New-Item -ItemType Directory -Path $stagingDir | Out-Null

# Copy files to staging directory
Copy-Item -Path $ExePath -Destination $stagingDir
Copy-Item -Path $LicensePath -Destination $stagingDir
Copy-Item -Path $ReadmePath -Destination $stagingDir
Copy-Item -Path $ReadmeJaPath -Destination $stagingDir

# Create the archive from the staging directory
Compress-Archive -Path "$stagingDir\*" -DestinationPath $OutputArchivePath

# Clean up the temporary staging directory
Remove-Item $stagingDir -Recurse -Force

Write-Host "Release package created successfully: $OutputArchivePath"
# setup.ps1
#
# Prepares a fresh copy of training-rust for the lesson runner (QUICKSTART.md, steps 1 to 6):
#   1. Checks that Rust 1.85 or later is available (rustup, rustc, cargo).
#   2. Checks for the C++ build tools that Rust uses for linking on Windows.
#   3. Checks for Node.js (npm).
#   4. Installs the lesson runner's npm packages (lesson-runner\node_modules).
#   5. Copies marked and viz into lesson-runner\src\vendor, where the runner loads them.
#   6. Downloads the runner's Rust crates (cargo fetch), so the first start is quicker.
#   7. Installs the rust-analyzer extension in VS Code, if the code command is available.
#
# With -InstallMissing, the script uses winget to install Rust, the C++ build tools, and Node.js when they're missing.
# Without it, the script stops and shows the winget commands to run.
#
# Safe to run again: each step checks what's already in place. Nothing here starts the lesson runner or changes
# machine-wide settings.
#
# Run from the repository root with:
#   powershell -ExecutionPolicy Bypass -File .\setup.ps1
#   powershell -ExecutionPolicy Bypass -File .\setup.ps1 -InstallMissing

param(
    [switch]$InstallMissing
)

$ErrorActionPreference = "Stop"
$root = $PSScriptRoot
Set-Location $root

$script:warnings = @()
$script:results = [ordered]@{}

function Write-Step([string]$title) {
    Write-Host ""
    Write-Host "== $title ==" -ForegroundColor Cyan
}

function Write-Note([string]$message) {
    Write-Host "   $message"
}

function Add-Warning([string]$message) {
    $script:warnings += $message
    Write-Host "   WARNING: $message" -ForegroundColor Yellow
}

function Stop-Setup([string]$message) {
    Write-Host ""
    Write-Host "SETUP STOPPED: $message" -ForegroundColor Red
    Write-Host "Fix the problem above, then run this script again. Steps already done will be skipped."
    exit 1
}

# Runs a program and stops the script if it returns a non-zero exit code. PowerShell doesn't do this for native programs by itself.
function Invoke-Checked([string]$program, [string[]]$arguments) {
    & $program @arguments
    if ($LASTEXITCODE -ne 0) {
        throw "Command failed: $program $($arguments -join ' ')"
    }
}

# winget installs update the registry's PATH, but this PowerShell window still has the old one. Reload it, and add Rust's folder, so new programs are found.
function Update-SessionPath {
    $machinePath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
    $env:Path = "$machinePath;$userPath;$cargoBin"
}

# Installs a package with winget. Returns $true when it works, and $false otherwise, so the caller decides whether that's fatal.
function Install-WithWinget([string]$id, [string]$label, [string]$override = "") {
    if (-not (Get-Command winget -ErrorAction SilentlyContinue)) {
        Write-Note "winget isn't available on this computer, so $label can't be installed automatically."
        return $false
    }
    Write-Note "Installing $label with winget (package id: $id). This can take several minutes..."
    if ($override) {
        & winget install --id $id -e --accept-package-agreements --accept-source-agreements --override $override
    } else {
        & winget install --id $id -e --accept-package-agreements --accept-source-agreements
    }
    if ($LASTEXITCODE -ne 0) {
        Write-Note "winget didn't install $label (exit code $LASTEXITCODE). Company policy can block winget installs. Ask IT, or install it by hand."
        return $false
    }
    Update-SessionPath
    return $true
}

# Returns the installed Rust version as a [version], or $null when rustc isn't available.
function Get-RustVersion {
    try {
        $output = (& rustc --version 2>$null)
        if ($LASTEXITCODE -eq 0 -and $output -match '(\d+\.\d+\.\d+)') {
            return [version]$Matches[1]
        }
    } catch {
        return $null
    }
    return $null
}

# Returns the install path of Visual Studio Build Tools with the C++ workload, or an empty string.
function Get-CppBuildToolsPath {
    $vswhere = Join-Path ${env:ProgramFiles(x86)} "Microsoft Visual Studio\Installer\vswhere.exe"
    if (-not (Test-Path $vswhere)) {
        return ""
    }
    $path = & $vswhere -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -latest -property installationPath 2>$null
    if ($path) {
        return ($path | Select-Object -First 1).Trim()
    }
    return ""
}

# 1. Rust ------------------------------------------------------------------------------------------

Write-Step "1. Rust 1.85 or later (rustup, rustc, cargo)"
$rustVersion = Get-RustVersion
if (-not $rustVersion) {
    if ($InstallMissing) {
        Write-Note "Rust isn't on PATH, so it will be installed with rustup."
        if (Install-WithWinget "Rustlang.Rustup" "Rust (rustup)") {
            $rustVersion = Get-RustVersion
        }
    }
    if (-not $rustVersion) {
        Stop-Setup ("Rust isn't installed, or isn't on PATH. Either:`n" +
            "   - run this script with -InstallMissing, which uses winget, or`n" +
            "   - install it yourself: winget install Rustlang.Rustup`n" +
            "   Then open a new PowerShell window and run this script again.")
    }
}
if ($rustVersion -lt [version]"1.85") {
    Stop-Setup "Rust $rustVersion was found. Rust 1.85 or later is needed, because the course uses the 2024 edition. Run: rustup update"
}
Write-Note "Found Rust $rustVersion."
$script:results["Rust $rustVersion"] = "ok"

# 2. C++ build tools -------------------------------------------------------------------------------

Write-Step "2. C++ build tools (the linker Rust uses on Windows)"
$cppPath = Get-CppBuildToolsPath
if ($cppPath) {
    Write-Note "Found the C++ workload (Visual Studio or Build Tools) at $cppPath."
    $script:results["C++ workload"] = "ok"
} elseif ($InstallMissing) {
    Write-Note "The C++ build tools aren't installed, so they will be installed. This is a large download."
    $override = "--wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
    if (Install-WithWinget "Microsoft.VisualStudio.2022.BuildTools" "Visual Studio Build Tools" $override) {
        $cppPath = Get-CppBuildToolsPath
    }
    if ($cppPath) {
        Write-Note "Installed the C++ build tools."
        $script:results["C++ workload"] = "ok"
    } else {
        Add-Warning "The C++ build tools couldn't be installed. Install them by hand: winget install Microsoft.VisualStudio.2022.BuildTools, then choose 'Desktop development with C++'. Rust can't link programs without them."
        $script:results["C++ workload"] = "missing"
    }
} else {
    Add-Warning "The C++ workload isn't installed, in Visual Studio or the Build Tools. Rust needs it to link programs. Install Visual Studio with 'Desktop development with C++', or install the Build Tools with: winget install Microsoft.VisualStudio.2022.BuildTools --override ""--wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"" (or run this script with -InstallMissing, which installs only the Build Tools)."
    $script:results["C++ workload"] = "missing"
}

# 3. Node.js -----------------------------------------------------------------------------------------

Write-Step "3. Node.js (npm)"
$npm = Get-Command npm -ErrorAction SilentlyContinue
if (-not $npm -and $InstallMissing) {
    Write-Note "Node.js isn't on PATH, so it will be installed."
    if (Install-WithWinget "OpenJS.NodeJS.LTS" "Node.js LTS") {
        $npm = Get-Command npm -ErrorAction SilentlyContinue
    }
}
if (-not $npm) {
    Stop-Setup ("Node.js (npm) isn't installed, or isn't on PATH. Either:`n" +
        "   - run this script with -InstallMissing, which uses winget, or`n" +
        "   - install it yourself: winget install OpenJS.NodeJS.LTS`n" +
        "   Then open a new PowerShell window and run this script again.")
}
$npmVersion = (& npm --version 2>$null).Trim()
Write-Note "Found npm $npmVersion."
$script:results["Node.js (npm $npmVersion)"] = "ok"

# 4. Lesson runner packages --------------------------------------------------------------------------

Write-Step "4. Lesson runner packages (npm install)"
$runnerDir = Join-Path $root "lesson-runner"
Push-Location $runnerDir
try {
    Write-Note "Installing the packages in lesson-runner\package.json. This takes a minute or two the first time..."
    Invoke-Checked "npm" @("install")
} finally {
    Pop-Location
}

$markedSource = Join-Path $runnerDir "node_modules\marked\lib\marked.esm.js"
$vizSource = Join-Path $runnerDir "node_modules\@viz-js\viz\dist\viz.js"
if ((Test-Path $markedSource) -and (Test-Path $vizSource)) {
    Write-Note "The marked and viz packages are installed."
    $script:results["Runner packages"] = "ok"
} else {
    Stop-Setup "npm install finished, but marked or @viz-js/viz wasn't found under lesson-runner\node_modules. Check the npm output above for errors."
}

# 5. Browser libraries -----------------------------------------------------------------------------

Write-Step "5. Browser libraries for the lesson runner (src\vendor)"
$vendorDir = Join-Path $runnerDir "src\vendor"
New-Item -ItemType Directory -Force $vendorDir | Out-Null
$markedTarget = Join-Path $vendorDir "marked.js"
$vizTarget = Join-Path $vendorDir "viz.js"
Copy-Item $markedSource $markedTarget -Force
Copy-Item $vizSource $vizTarget -Force
if ((Test-Path $markedTarget) -and (Test-Path $vizTarget)) {
    Write-Note "Copied marked.js and viz.js into lesson-runner\src\vendor."
    $script:results["Browser libraries"] = "ok"
} else {
    Stop-Setup "The browser libraries couldn't be copied into lesson-runner\src\vendor."
}

# 6. Rust crates for the runner ----------------------------------------------------------------------

Write-Step "6. Rust crates for the lesson runner (cargo fetch)"
$tauriDir = Join-Path $runnerDir "src-tauri"
Push-Location $tauriDir
try {
    Write-Note "Downloading the crates listed in lesson-runner\src-tauri\Cargo.toml..."
    Invoke-Checked "cargo" @("fetch")
    $script:results["Runner crates"] = "ok"
} catch {
    Add-Warning "cargo fetch failed. The first start of the runner will download the crates instead, which needs an internet connection. Error: $($_.Exception.Message)"
    $script:results["Runner crates"] = "not fetched"
} finally {
    Pop-Location
}

# 7. VS Code extension ----------------------------------------------------------------------------

Write-Step "7. VS Code and the rust-analyzer extension (optional)"
$code = Get-Command code -ErrorAction SilentlyContinue
if (-not $code) {
    Write-Note "VS Code's code command isn't on PATH, so the extension wasn't installed. This is optional."
    Write-Note "Install VS Code with: winget install Microsoft.VisualStudioCode, then add rust-analyzer from the Extensions view."
    $script:results["rust-analyzer"] = "skipped (VS Code not on PATH)"
} else {
    $installed = (& code --list-extensions 2>$null) -contains "rust-lang.rust-analyzer"
    if ($installed) {
        Write-Note "The rust-analyzer extension is already installed."
    } else {
        Invoke-Checked "code" @("--install-extension", "rust-lang.rust-analyzer")
        Write-Note "Installed the rust-analyzer extension."
    }
    $script:results["rust-analyzer"] = "ok"
}

# Summary ----------------------------------------------------------------------------------------------

Write-Step "Summary"
foreach ($item in $script:results.GetEnumerator()) {
    Write-Host ("   {0,-30} {1}" -f $item.Key, $item.Value)
}

if ($script:warnings.Count -gt 0) {
    Write-Host ""
    Write-Host "Things to do:" -ForegroundColor Yellow
    foreach ($warning in $script:warnings) {
        Write-Host "   - $warning"
    }
}

Write-Host ""
Write-Host "Setup finished. To start the lesson runner, run these from the repository root:" -ForegroundColor Green
Write-Host "   cd lesson-runner"
Write-Host "   npm run tauri dev"
Write-Host ""
Write-Host "The first start takes several minutes, because Rust compiles the runner. Later starts are much faster."

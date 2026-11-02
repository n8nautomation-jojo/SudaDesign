# SudaDesign Android Installer & Launcher
$ErrorActionPreference = "Stop"
$platformTools = "C:\Users\HP\AppData\Local\Android\Sdk\platform-tools"
$env:PATH = "$platformTools;$env:PATH"

$apkPath = "apps\photocraft-android\android\app\build\outputs\apk\debug\app-debug.apk"
if (-not (Test-Path $apkPath)) {
    Write-Error "APK not found at $apkPath. Please build it first."
    exit 1
}

Write-Host "Searching for connected Android devices via ADB..." -ForegroundColor Cyan
$devices = adb devices | Select-String -Pattern "\b(device|unauthorized)\b"

if (-not $devices) {
    Write-Warning "No Android devices detected. Please ensure your device is connected via USB with USB debugging enabled, or connected over wireless ADB."
    exit 1
}

Write-Host "Installing SudaDesign APK ($apkPath)..." -ForegroundColor Green
adb install -r $apkPath

Write-Host "Launching SudaDesign..." -ForegroundColor Green
adb shell am start -n ai.storyteller.photocraft/ai.storyteller.photocraft.MainActivity

Write-Host "SudaDesign launched successfully!" -ForegroundColor Green


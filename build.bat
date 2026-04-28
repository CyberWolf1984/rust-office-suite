@echo off
REM =========================================================================
REM  Rust-Office Build & Package Script (Windows)
REM =========================================================================
REM  Prerequisites:
REM    1. Rust toolchain (rustup)      — https://rustup.rs
REM    2. WiX Toolset v4 (dotnet CLI)  — dotnet tool install -g wix
REM    3. WiX UI extension             — wix extension add WixToolset.UI.wixext
REM
REM  Usage:
REM    build.bat           — build debug binaries only
REM    build.bat release   — build release binaries + GUI MSI installer
REM =========================================================================

set MODE=%1
if "%MODE%"=="" set MODE=debug

echo.
echo ========================================
echo  Rust-Office Build System
echo ========================================
echo.

REM -----------------------------------------------------------------
REM  Step 1: Compile
REM -----------------------------------------------------------------
echo [1/3] Compiling Rust-Office (%MODE%) ...

if "%MODE%"=="release" (
    cargo build --release
    set BIN_DIR=target\release
) else (
    cargo build
    set BIN_DIR=target\debug
)

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo  *** BUILD FAILED ***
    exit /b 1
)

echo       Binaries written to %BIN_DIR%\
echo.

REM -----------------------------------------------------------------
REM  Step 2: List built artefacts
REM -----------------------------------------------------------------
echo [2/3] Built artefacts:
echo       - %BIN_DIR%\rust-office.exe   (Launcher)
echo       - %BIN_DIR%\ro-writer.exe     (Writer)
echo       - %BIN_DIR%\ro-calc.exe       (Calc)
echo.

REM -----------------------------------------------------------------
REM  Step 3: Package MSI (release only)
REM -----------------------------------------------------------------
if "%MODE%"=="release" (
    echo [3/3] Packaging GUI installer (.msi) ...
    echo.

    REM Ensure the WiX UI extension is available
    wix extension add WixToolset.UI.wixext 2>nul

    wix build installer\wix\main.wxs ^
        -ext WixToolset.UI.wixext ^
        -d BinDir=%BIN_DIR% ^
        -o target\rust-office.msi

    if %ERRORLEVEL% EQU 0 (
        echo.
        echo  *** MSI installer created: target\rust-office.msi ***
        echo.
        echo  The installer provides:
        echo    - Welcome screen
        echo    - License agreement
        echo    - Custom install (select Writer, Calc, etc.)
        echo    - Install directory picker
        echo    - Start Menu + Desktop shortcuts
    ) else (
        echo.
        echo  *** MSI packaging failed ***
        echo  Make sure WiX Toolset v4 is installed:
        echo    dotnet tool install -g wix
        echo    wix extension add WixToolset.UI.wixext
    )
) else (
    echo [3/3] Skipping MSI packaging (debug build).
    echo       Run "build.bat release" to create the installer.
)

echo.
echo Done.

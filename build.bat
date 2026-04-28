@echo off
REM =========================================================================
REM  Rust-Office Build & Package Script (Windows)
REM =========================================================================
REM  Prerequisites:
REM    1. Rust toolchain (rustup)     — https://rustup.rs
REM    2. WiX Toolset v4 (dotnet)     — dotnet tool install -g wix
REM
REM  Usage:
REM    build.bat           — build debug binaries
REM    build.bat release   — build release binaries + MSI installer
REM =========================================================================

set MODE=%1
if "%MODE%"=="" set MODE=debug

echo [1/3] Building Rust-Office (%MODE%) ...

if "%MODE%"=="release" (
    cargo build --release
    set BIN_DIR=target\release
) else (
    cargo build
    set BIN_DIR=target\debug
)

if %ERRORLEVEL% NEQ 0 (
    echo BUILD FAILED
    exit /b 1
)

echo [2/3] Build complete.  Binaries in %BIN_DIR%

if "%MODE%"=="release" (
    echo [3/3] Packaging MSI installer ...
    wix build installer\wix\main.wxs -d BinDir=%BIN_DIR% -o target\rust-office.msi
    if %ERRORLEVEL% EQU 0 (
        echo MSI created: target\rust-office.msi
    ) else (
        echo MSI packaging failed — is WiX Toolset installed?
    )
) else (
    echo [3/3] Skipping MSI packaging (debug build).
)

echo Done.

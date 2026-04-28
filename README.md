# Rust-Office

A ground-up, high-performance rewrite of the LibreOffice productivity suite in Rust. 

## Project Goals

This project aims to modernize and optimize the desktop productivity experience for contemporary hardware while maintaining absolute software agility. 

- **Extreme Memory Efficiency**: Target an average idle RAM footprint of **< 20MB** per process by stripping away heavy dynamic dependencies, removing garbage-collected runtime hooks, and utilizing zero-copy, memory-mapped (`mmap`) storage abstractions.
- **Hardware Acceleration**: Built from the ground up for modern parallel instruction execution.
  - **AMD CPU SIMD Mapping**: Highly-threaded data execution utilizing parallel iterators.
  - **AMD iGPU Offload**: Pushing heavy geometry and formatting computation pipelines straight onto the GPU.

## Modern Rendering Architecture

Instead of the archaic C++ visual layouts, Rust-Office utilizes **WGPU** as its low-level hardware interface. This ensures lightning-fast user interactions while keeping the code loosely coupled enough to target Vulkan, Metal, or DirectX 12 interchangeably down the line.

## Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| [Rust](https://rustup.rs) | 1.80+ (stable) | Compiler toolchain |
| [Git](https://git-scm.com) | 2.x | Source control |
| [WiX Toolset v4](https://wixtoolset.org) | 4.x | Windows MSI packaging (optional) |
| Visual Studio Build Tools | 2022+ | MSVC linker & Windows SDK |

> **Note:** WiX is only needed if you want to build the `.msi` installer. You can compile and run the applications without it.

Install the WiX CLI via .NET (if needed):
```
dotnet tool install -g wix
```

## Compiling from Source

### Clone the Repository

```bash
git clone https://github.com/CyberWolf1984/rust-office-suite.git
cd rust-office-suite
```

### Debug Build (fast compile, unoptimized)

```bash
cargo build
```

Binaries will be placed in `target/debug/`:
- `rust-office.exe` — Launcher / Start Centre
- `ro-writer.exe` — Writer (word processor)
- `ro-calc.exe` — Calc (spreadsheet)

### Release Build (slow compile, fully optimized)

```bash
cargo build --release
```

Binaries will be placed in `target/release/`. This build enables all optimizations including LTO and is what you should use for real-world usage.

### Build Individual Applications

```bash
cargo build -p writer            # Writer only
cargo build -p calc              # Calc only
cargo build -p launcher          # Launcher only
```

### Run Directly (without installing)

```bash
cargo run -p launcher             # Start Centre
cargo run -p writer               # Writer
cargo run -p calc                 # Calc
```

Or with the launcher's app selector:
```bash
cargo run -p launcher -- --writer
cargo run -p launcher -- --calc
```

## Installing on Windows

### Option A: Build and Package the MSI Installer

The included `build.bat` script compiles a release build and packages it into a Windows `.msi` installer:

```cmd
build.bat release
```

This will:
1. Compile all binaries in release mode.
2. Run `wix build` to produce `target/rust-office.msi`.

Then double-click `target/rust-office.msi` to install. The installer will:
- Copy all binaries to `C:\Program Files\RustOffice\bin\`.
- Create **Start Menu shortcuts** for Rust-Office, Writer, and Calc.

To uninstall, use **Settings → Apps → Rust-Office → Uninstall**.

### Option B: Manual Install (no MSI)

1. Build the release binaries:
   ```bash
   cargo build --release
   ```
2. Copy the three executables from `target/release/` to a directory of your choice (e.g. `C:\RustOffice\`):
   - `rust-office.exe`
   - `ro-writer.exe`
   - `ro-calc.exe`
3. (Optional) Add that directory to your system `PATH`.

## Ported LibreOffice Software Licensing

In compliance with open-source reuse configurations, this repository preserves the original legal structure:
- [COPYING](./COPYING)
- [COPYING.LGPL](./COPYING.LGPL)
- [COPYING.MPL](./COPYING.MPL)

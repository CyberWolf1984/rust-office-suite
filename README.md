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

Binaries will be placed in `target/debug/` (without the .exe extension on Linux):
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

### Option A: GUI Installer (recommended)

The included `build.bat` script compiles a release build and packages it into a Windows `.msi` installer with a full graphical setup wizard:

```cmd
build.bat release
```

Then double-click `target/rust-office.msi` to launch the installer. The setup wizard walks you through:

1. **Welcome** — introduction screen.
2. **License Agreement** — MPL-2.0 / LGPL / GPL terms (must accept to continue).
3. **Custom Setup** — a feature tree where you can check/uncheck individual applications:
   - ✅ **Writer** — Word processor (.odt, .docx)
   - ✅ **Calc** — Spreadsheet (.ods, .xlsx)
   - *(Future: Impress, Draw, Base, Math)*
4. **Install Directory** — defaults to `C:\Program Files\RustOffice\`.
5. **Installation Progress** — copies selected files.
6. **Finish** — installation complete.

The installer creates:
- **Start Menu shortcuts** under `Rust-Office` for each selected app.
- **Desktop shortcuts** for Writer and Calc.

To uninstall, use **Settings → Apps → Rust-Office → Uninstall**, or re-run the `.msi`.

### Option B: Manual Install (no MSI)

1. Build the release binaries:
   ```bash
   cargo build --release
   ```
2. Copy the executables from `target/release/` to a directory of your choice (e.g. `C:\RustOffice\`):
   - `rust-office.exe`
   - `ro-writer.exe`
   - `ro-calc.exe`
3. (Optional) Add that directory to your system `PATH`.

## Ported LibreOffice Software Licensing

In compliance with open-source reuse configurations, this repository preserves the original legal structure:
- [COPYING](./COPYING)
- [COPYING.LGPL](./COPYING.LGPL)
- [COPYING.MPL](./COPYING.MPL)

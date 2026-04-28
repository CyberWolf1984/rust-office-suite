# LibreOffice Rust Migration & Optimization Plan

This plan provides a comprehensive map of the LibreOffice repository, sectioned by functionality. It outlines a strategy to rewrite the suite in Rust, with a specific focus on **extreme memory efficiency** (averaging < 20MB RAM, max 1GB) and **hardware optimization** for AMD CPUs (multi-threading) and AMD iGPUs (compute/rendering acceleration).

## 1. Foundational Abstractions & System Layer
*These libraries form the absolute bedrock of the application, managing memory, threading, and system calls.*

- **`sal/` (System Abstraction Layer)**
  - **Core Features**: OS-level abstractions, threading, file I/O, IPC, string management.
  - **Rust Optimization**: Replace with `std`, `tokio` (for asynchronous I/O), and `memmap2`. Memory-mapped files will be crucial to keeping RAM usage below 20MB by letting the OS page data in and out.
- **`o3tl/` & `tools/`**
  - **Core Features**: Custom template libraries, fundamental types (Dates, Fractions, Strings).
  - **Rust Optimization**: Port to pure, `#![no_std]` compatible Rust crates to guarantee zero hidden allocations.
- **`comphelper/`, `cppu/`, `cppuhelper/`, `unoil/`**
  - **Core Features**: The UNO (Universal Network Objects) component model.
  - **Rust Optimization**: This is a major source of memory overhead. We will replace this entirely with static Rust traits and generic constraints (`impl Trait`) where possible, avoiding dynamic dispatch (`dyn Trait`) to prevent heap fragmentation.

## 2. Graphics, Rendering, and GPU Compute
*Critical for UI responsiveness and offloading heavy lifting to the AMD iGPU.*

- **`basegfx/` & `drawinglayer/`**
  - **Core Features**: 2D/3D math, geometry algorithms, and visual primitives mapping.
  - **Rust Optimization**: Use `glam` for SIMD-accelerated math (optimized for AMD CPUs). Move primitive generation into compute shaders where possible.
- **`vcl/` (Visual Class Library)**
  - **Core Features**: The core widget toolkit, OS windowing layer, and legacy rendering.
  - **Rust Optimization**: Completely replace the rendering backend with **WGPU**. WGPU provides the most modern rendering technology (WebGPU standard) and maps to Vulkan on AMD/Windows, but crucially leaves the door open to easily add native DX12 (for Intel/Nvidia) or Metal (for Apple) backends later without changing our code. The UI will be rendered directly on the iGPU in a single pass.
- **`canvas/` & `cppcanvas/`**
  - **Core Features**: UNO canvas rendering model.
  - **Rust Optimization**: Obsolete. Replaced by the new Vulkan/WGPU VCL backend.
- **`opencl/`**
  - **Core Features**: OpenCL integration for math processing.
  - **Rust Optimization**: Migrate compute tasks (especially for Calc) from OpenCL to Vulkan Compute Shaders (`vkCmdDispatch`) for lower latency and better integration with the AMD graphics pipeline.

## 3. Core Application Framework
*The shared logic that glues the applications together and handles document states.*

- **`sfx2/`**
  - **Core Features**: Document load/save pipelines, action dispatching, properties, document models.
  - **Rust Optimization**: Implement a zero-copy state machine. Documents will not be fully loaded into memory; instead, only the currently visible data (the viewport) will be deserialized, keeping baseline RAM near ~20MB.
- **`svx/` & `svtools/`**
  - **Core Features**: Shared drawing models, chart tools, and common UI controls across all apps.
  - **Rust Optimization**: Centralize these into a single highly-optimized `libre-core` crate to reduce the final binary size and maximize instruction cache efficiency on the CPU.
- **`framework/`**
  - **Core Features**: UI chrome (toolbars, menus, status bars).
  - **Rust Optimization**: Implement a data-driven, retained-mode UI built on top of the new GPU backend.

## 4. Specific Applications
*The user-facing programs. Each will receive specific threading/compute optimizations.*

- **`sc/` (Calc - Spreadsheet)**
  - **Core Features**: Formula parser, dependency graph, grid rendering.
  - **Rust Optimization**: Calc is perfectly suited for AMD architectures. We will use `rayon` to evaluate independent formula branches across all available AMD CPU cores. Matrix operations and large-scale recalculations will be sent to the AMD iGPU via Vulkan Compute.
- **`sw/` (Writer - Word Processor)**
  - **Core Features**: Text layout, pagination, spellchecking.
  - **Rust Optimization**: Text shaping and layout are CPU-intensive. We will use `rustybuzz` and implement a highly concurrent layout engine that paginates in the background without blocking the UI thread.
- **`sd/` (Impress/Draw - Presentations)**
  - **Core Features**: Slide layouts, transitions, vector graphics.
  - **Rust Optimization**: Render all slides and transitions natively on the iGPU, minimizing CPU-GPU memory transfers.
- **`starmath/` (Math), `dbaccess/` (Base)**
  - **Core Features**: Equation editing, database integration.
  - **Rust Optimization**: De-prioritize during early phases, eventually porting as lightweight modular plugins.

## 5. Document Formats, Parsers & Filters
*Loading and saving files without exploding memory limits.*

- **`oox/` (OOXML), `xmloff/`, `filter/`, `svl/`**
  - **Core Features**: Parsing and writing `.docx`, `.xlsx`, `.odt`, `.ods`, etc.
  - **Rust Optimization**: This is the highest risk area for memory spikes. We will use `quick-xml` combined with zero-copy deserialization (e.g., `serde` with `&'a str` borrows from a memory-mapped file). We will NEVER buffer an entire XML file into RAM; we will use event-driven parsing (SAX-style) to construct only the necessary parts of the document tree.

## 6. Scripting & Macros
- **`basic/`, `scripting/`, `vbahelper/`**
  - **Core Features**: StarBasic and VBA macro execution.
  - **Rust Optimization**: To hit the 20MB memory target, embedding heavy scripting runtimes (like Java or legacy Basic) at startup is impossible. Macros will be strictly lazy-loaded on-demand, potentially compiling them to lightweight WebAssembly (`wasm32-unknown-unknown`) using `wasmtime` for secure, low-overhead execution.

---

## Verification Plan

### Idle Memory Estimation (Example: Draw viewing a PDF)
By completely removing UNO, utilizing memory-mapped files, and pushing rendering state to the GPU via WGPU, the estimated idle RAM footprint for Draw viewing a standard PDF will be roughly **12-15 MB RSS**:
- **Executable Pages**: ~3-5 MB (only code currently executing is paged into RAM).
- **UI & Application State**: ~2 MB (using static structs instead of heap-allocated UNO objects).
- **PDF Data**: ~0 MB RSS (PDF file is mapped directly to disk via `mmap`; the OS only pages in the specific bytes being read).
- **WGPU / GPU Driver Overhead**: ~5-8 MB (maintaining the swapchain and command buffers).

### Memory Profiling Target (< 20MB)
- Use `valgrind` (massif), `heaptrack`, and OS-level memory tooling to ensure that opening a blank Writer document consumes no more than 20MB of resident set size (RSS).
- Assert zero-copy parsing by measuring memory spikes during the loading of a 100MB `.docx` file (target spike: < 5MB).

### AMD Optimization Verification
- **CPU**: Use AMD uProf (Microprocessor Profiler) to verify high core utilization across all threads during Calc recalculations.
- **iGPU**: Use Vulkan profiling tools (like RenderDoc or AMD Radeon Developer Tool Suite) to ensure rendering passes and compute shaders are executing efficiently on the integrated graphics without CPU bottlenecking.

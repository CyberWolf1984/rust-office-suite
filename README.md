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

## Ported LibreOffice Software Licensing

In compliance with open-source reuse configurations, this repository preserves the original legal structure:
- [COPYING](./COPYING)
- [COPYING.LGPL](./COPYING.LGPL)
- [COPYING.MPL](./COPYING.MPL)

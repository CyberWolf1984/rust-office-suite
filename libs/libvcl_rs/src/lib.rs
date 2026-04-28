//! # libvcl_rs — Visual Class Library
//!
//! GPU-accelerated UI toolkit built on WGPU + winit.  All rendering flows
//! through a single abstraction layer so backends can be swapped between
//! Vulkan (AMD), DX12 (Intel/Nvidia), and Metal (Apple) at runtime.

pub mod event;
pub mod renderer;
pub mod widget;
pub mod window;

pub use event::UiEvent;
pub use renderer::Renderer;
pub use window::AppWindow;

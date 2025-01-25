//! Application
//!
//! A module used to handle input and windowing

use winit::{
	application::ApplicationHandler,
	event::WindowEvent,
	event_loop::ActiveEventLoop,
	platform::windows::WindowAttributesExtWindows,
	raw_window_handle::{HasWindowHandle, RawWindowHandle},
	window::{Window, WindowAttributes, WindowId},
};

use crate::gi::context::Context;

/// DirectX 11 Test
///
/// Encapsulates the components for testing D3D11 rendering
pub(super) struct TestD3D11 {
	/// A graphics adapter used for rendering.
	context: Context,
	/// An optional window for rendering.
	window: Option<Window>,
}

impl Default for TestD3D11 {
	fn default() -> Self {
		Self {
			context: Context::default(),
			window: None,
		}
	}
}

impl ApplicationHandler for TestD3D11 {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let attributes: WindowAttributes = WindowAttributes::default()
			.with_active(true)
			.with_resizable(true)
			.with_window_icon(None)
			.with_taskbar_icon(None)
			.with_class_name("hello")
			.with_title("Hello, Direct3D 11!");

		let window: Window = event_loop
			.create_window(attributes)
			.expect("failed to create a window");

		if let Some(monitor) = window.current_monitor() {
			let sf: winit::dpi::LogicalSize<u32> =
				monitor.size().to_logical::<u32>(monitor.scale_factor());
			let size: (u32, u32) = (window.inner_size().width, window.inner_size().height);

			window.set_outer_position(winit::dpi::LogicalPosition::new(
				(sf.width - size.0) / 2,
				(sf.height - size.1) / 2,
			));
		}

		let RawWindowHandle::Win32(handle) = window.window_handle().unwrap().as_raw() else {
			unreachable!()
		};

		self.context
			.set_swap_chain(
				window.inner_size().width,
				window.inner_size().height,
				&windows::Win32::Foundation::HWND(handle.hwnd.get() as *mut std::ffi::c_void),
			)
			.expect("failed to create swapchain");

		self.window = Some(window);
	}

	fn window_event(
		&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent,
	) {
		match event {
			| WindowEvent::CloseRequested => event_loop.exit(),
			| WindowEvent::Resized(size) => {
				if let Some(window) = &self.window {
					let lsize: winit::dpi::LogicalSize<u32> =
						size.to_logical::<u32>(window.scale_factor());

					if lsize.width != 0 && lsize.height != 0 {
						self.context
							.resize_buffers(lsize.width, lsize.height)
							.expect("failed to resize swap chain buffers");
					}
					
					window.request_redraw();
				}
			}
			| WindowEvent::RedrawRequested => {
				if let Some(window) = &self.window {
					self.context.present().expect("failed to present frames");

					window.request_redraw();
				}
			}
			| _ => (),
		}
	}
}

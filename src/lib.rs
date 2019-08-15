use ash::{extensions::khr, prelude::*, vk};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::{ffi::CStr, ptr};

/// Create a surface from a raw surface handle.
///
/// `instance` must have created with platform specific surface extensions enabled.
pub unsafe fn create_surface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    window_handle: &impl HasRawWindowHandle,
) -> VkResult<vk::SurfaceKHR> {
    match window_handle.raw_window_handle() {
        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(handle) => {
            use winapi::um::libloaderapi::GetModuleHandleW;

            let hwnd = handle.hwnd;
            let hinstance = GetModuleHandleW(ptr::null());
            let win32_surface_desc = vk::Win32SurfaceCreateInfoKHR::builder()
                .hinstance(hinstance as *const _)
                .hwnd(hwnd);
            let win32_surface_fn = khr::Win32Surface::new(entry, instance);
            win32_surface_fn.create_win32_surface(&win32_surface_desc, None)
        }
        _ => unimplemented!(),
    }
}

/// Query the required instance extension for creating a surface from a window handle.
pub fn enumerate_required_extension(window_handle: &impl HasRawWindowHandle) -> &'static CStr {
    match window_handle.raw_window_handle() {
        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(_) => khr::Win32Surface::name(),

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(_) => khr::WaylandSurface::name(),

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::X11(_) => khr::XlibSurface::name(),

        _ => unimplemented!(),
    }
}

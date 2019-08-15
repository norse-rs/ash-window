use ash::{extensions::khr, prelude::*, vk};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::ptr;
use winapi::um::libloaderapi::GetModuleHandleW;

/// Create a surface from a raw surface handle.
///
/// `instance` must have created with platform specific surface extensions enabled.
pub unsafe fn create_surface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    window_handle: &impl HasRawWindowHandle,
) -> VkResult<vk::SurfaceKHR> {
    match window_handle.raw_window_handle() {
        RawWindowHandle::Windows(handle) => {
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

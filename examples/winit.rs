//! Demonstrate interop with winit windows.
//!
//! Sample creates a surface from a winit window through the
//! platform agnostic window handle trait.
//!
//! On instance extensions platform specific extensions need to be enabled.

use ash::{extensions::khr, version::EntryV1_0, vk};
use std::error::Error;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;

    unsafe {
        let entry = ash::Entry::new()?;

        let instance_extensions = vec![
            khr::Surface::name().as_ptr(),
            // Platform specific surface extensions
            #[cfg(windows)]
            khr::Win32Surface::name().as_ptr(),
        ];
        let app_desc = vk::ApplicationInfo::builder().api_version(ash::vk_make_version!(1, 0, 0));
        let instance_desc = vk::InstanceCreateInfo::builder()
            .application_info(&app_desc)
            .enabled_extension_names(&instance_extensions);
        let instance = entry.create_instance(&instance_desc, None)?;

        // Create a surface from winit window.
        let surface = ash_window::create_surface(&entry, &instance, &window, None)?;
        println!("surface: {:?}", surface);

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => *control_flow = ControlFlow::Wait,
        });
    }
}

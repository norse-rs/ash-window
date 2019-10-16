//! Demonstrate interop with beryllium/SDL windows.
//!
//! Sample creates a surface from a window through the
//! platform agnostic window handle trait.
//!
//! On instance extensions platform specific extensions need to be enabled.

use ash::{extensions::khr, version::EntryV1_0, vk};
use beryllium::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl = beryllium::init()?;

    let window = sdl.create_window(
        "ash-window x beryllium",
        WINDOW_POSITION_CENTERED,
        WINDOW_POSITION_CENTERED,
        800,
        600,
        WindowFlags::default(),
    )?;

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
        let surface_fn = ash::extensions::khr::Surface::new(&entry, &instance);
        println!("surface: {:?}", surface);

        'main: loop {
            while let Some(event) = sdl.poll_event() {
                match event {
                    Event::Quit { .. } => break 'main,
                    _ => (),
                }
            }
        }

        surface_fn.destroy_surface(surface, None);
    }

    Ok(())
}

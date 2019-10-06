
<h1 align="center">ash-window</h1>
<p align="center">
    <a href="https://github.com/norse-rs">
       <img src="https://img.shields.io/badge/project-norse-9cf.svg?style=flat-square" alt="NORSE">
    </a>
    <a href="LICENSE-MIT">
      <img src="https://img.shields.io/badge/license-MIT-green.svg?style=flat-square" alt="License - MIT">
    </a>
    <a href="LICENSE-APACHE">
      <img src="https://img.shields.io/badge/license-APACHE2-green.svg?style=flat-square" alt="License - Apache2">
    </a>
    <br>
    <a href="https://dev.azure.com/msiglreith/norse/_build/latest?definitionId=3&branchName=master">
    <img src="https://dev.azure.com/msiglreith/norse/_apis/build/status/norse-rs.ash-window?branchName=master" alt="Build Status">
    </a>
</p>

Interoperability between [`ash`](https://github.com/MaikKlein/ash) and [`raw-window-handle`](https://github.com/rust-windowing/raw-window-handle) for surface creation.

## Usage

The library exposes two functions:

- `enumerate_required_extension` returns the required instance extension needed for surface creation from a specific window handle.

- `create_surface` allows to create a surface from a type implementing `HasRawWindowHandle`

```rust
ash_window::create_surface(&entry, &instance, &window, None)?;
```

## Support

- [x] Windows (`VK_KHR_win32_surface`)
- [x] Unix (`VK_KHR_xlib_surface`/`VK_KHR_xcb_surface`/`VK_KHR_wayland_surface`)
- [x] MacOS/IOS (`VK_MVK_macos_surface`/VK_MVK_ios_surface`, `VK_EXT_metal_surface` not exposed yet)
- [x] Android (`VK_KHR_android_surface`)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any Contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

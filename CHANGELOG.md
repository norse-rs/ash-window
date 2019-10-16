# Unreleased
## Changes
- `enumerate_required_extension` renamed to `enumerate_required_extensions`
- `enumerate_required_extensions` will return an error if the window handle is not supported instead of panic.
- `enumerate_required_extensions` may return multiple extension names. Includes all dependent extensions.
- `create_surface` will return an error if the window handle is not supported instead of panic.

# Version 0.1.0
Initial release for `raw-window-handle = "0.3"` with Windows, Linux, Android, MacOS/iOS support.
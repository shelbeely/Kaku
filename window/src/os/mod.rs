// Linux support: Wayland and X11
#[cfg(all(unix, not(target_os = "macos")))]
pub mod x11;
#[cfg(all(unix, not(target_os = "macos")))]
pub mod x_and_wayland;
#[cfg(all(unix, not(target_os = "macos")))]
pub mod xdg_desktop_portal;
#[cfg(all(unix, not(target_os = "macos")))]
pub mod xkeysyms;

#[cfg(all(unix, not(target_os = "macos"), feature = "wayland"))]
pub mod wayland;

// Export Linux implementations
#[cfg(all(unix, not(target_os = "macos")))]
pub use x_and_wayland::*;

pub mod parameters;

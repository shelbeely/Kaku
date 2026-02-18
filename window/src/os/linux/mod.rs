// Stub Linux/Wayland/X11 connection implementation
// This is a minimal stub to allow compilation on Linux/Ubuntu
// Full windowing support would require implementing the complete Connection trait

use crate::connection::ConnectionOps;
use crate::screen::Screens;
use crate::Appearance;
use anyhow::{bail, Result as Fallible};

pub struct Connection {}

impl Connection {
    pub fn create_new() -> Fallible<Self> {
        log::warn!("Linux GUI support is not yet fully implemented");
        log::warn!("This is a stub implementation for compilation purposes");
        bail!("Linux GUI support not yet available. Please use the CLI tools instead.");
    }
}

impl ConnectionOps for Connection {
    fn name(&self) -> String {
        "Linux (Stub)".to_string()
    }

    fn default_dpi(&self) -> f64 {
        96.0 // Standard Linux DPI
    }

    fn terminate_message_loop(&self) {
        // No-op
    }

    fn run_message_loop(&self) -> Fallible<()> {
        bail!("Message loop not implemented for Linux yet")
    }

    fn get_appearance(&self) -> Appearance {
        // For KDE Plasma, we could detect the theme here in the future
        // For now, return Light as default
        Appearance::Light
    }

    fn screens(&self) -> Fallible<Screens> {
        // Return empty screens for now
        bail!("Screen enumeration not yet implemented for Linux")
    }
}

// Stub Window type for Linux
pub struct Window {}

// Re-export for compatibility
pub use Connection as LinuxConnection;

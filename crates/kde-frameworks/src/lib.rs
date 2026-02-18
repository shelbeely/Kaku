//! KDE Frameworks Integration for Kurrent Terminal
//!
//! This crate provides deep integration with KDE Plasma desktop environment.
//!
//! ## Modules
//!
//! - `color_scheme`: Detect and apply KDE Plasma color schemes
//! - `notifications`: KNotifications integration for native KDE notifications
//! - `wallet`: KWallet integration for secure credential storage
//! - `activities`: Plasma Activities support for context-based configurations
//! - `dbus`: DBus service communication with KDE/Plasma

pub mod activities;
pub mod color_scheme;
pub mod dbus;
pub mod notifications;
pub mod wallet;

use anyhow::Result;

/// Main KDE Integration interface
pub struct KdeIntegration {
    plasma_version: Option<String>,
}

impl KdeIntegration {
    /// Create a new KDE integration instance.
    ///
    /// Detects if running in KDE Plasma and reads the Plasma version.
    pub fn new() -> Result<Self> {
        let plasma_version = dbus::detect_plasma_version();
        Ok(Self { plasma_version })
    }

    /// Check if running in KDE Plasma environment
    pub fn is_kde_plasma() -> bool {
        dbus::is_kde_plasma()
    }

    /// Get Plasma version (e.g. "5" or "6")
    pub fn plasma_version(&self) -> Option<&str> {
        self.plasma_version.as_deref()
    }

    /// Get the current KDE color scheme, if available
    pub fn get_color_scheme(&self) -> Result<Option<color_scheme::KdeColorScheme>> {
        color_scheme::get_current_color_scheme()
    }

    /// Show a desktop notification via the freedesktop Notifications interface
    pub fn notify(&self, title: &str, message: &str) -> Result<u32> {
        let notif = notifications::Notification::new(title, message);
        notifications::show_notification(&notif)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kde_detection_does_not_panic() {
        let _ = KdeIntegration::is_kde_plasma();
    }

    #[test]
    fn test_new_does_not_panic() {
        let kde = KdeIntegration::new().unwrap();
        // plasma_version may or may not be set depending on environment
        let _ = kde.plasma_version();
    }

    #[test]
    fn test_get_color_scheme_no_kde() {
        let kde = KdeIntegration::new().unwrap();
        // In CI without KDE, returns None gracefully
        let scheme = kde.get_color_scheme().unwrap();
        assert!(scheme.is_none());
    }
}

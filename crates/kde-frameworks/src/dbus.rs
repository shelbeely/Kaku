//! DBus Integration for KDE/Plasma Services
//!
//! This module provides low-level DBus communication with KDE services.
//! It's used by other modules (notifications, wallet, activities, etc.)
//!
//! ## Common KDE DBus Services
//!
//! - org.kde.KWin - Window manager
//! - org.kde.kwalletd5/6 - Credential storage
//! - org.kde.ActivityManager - Activities management
//! - org.freedesktop.Notifications - Desktop notifications
//! - org.kde.plasmashell - Plasma shell
//!
//! ## Usage
//!
//! This module provides utilities for connecting to and communicating
//! with KDE services via DBus.

use anyhow::{Context, Result};

/// Get a blocking DBus connection to the session bus
pub fn session_connection() -> Result<zbus::blocking::Connection> {
    zbus::blocking::Connection::session()
        .context("Failed to connect to DBus session bus")
}

/// Check if a DBus service is available on the session bus
pub fn is_service_available(service_name: &str) -> bool {
    let conn = match session_connection() {
        Ok(c) => c,
        Err(_) => return false,
    };
    let proxy = match zbus::blocking::fdo::DBusProxy::new(&conn) {
        Ok(p) => p,
        Err(_) => return false,
    };
    match service_name.try_into() {
        Ok(name) => proxy.name_has_owner(name).unwrap_or(false),
        Err(_) => false,
    }
}

/// KDE DBus service names
pub mod services {
    pub const KWIN: &str = "org.kde.KWin";
    pub const KWALLET5: &str = "org.kde.kwalletd5";
    pub const KWALLET6: &str = "org.kde.kwalletd6";
    pub const ACTIVITY_MANAGER: &str = "org.kde.ActivityManager";
    pub const NOTIFICATIONS: &str = "org.freedesktop.Notifications";
    pub const PLASMA_SHELL: &str = "org.kde.plasmashell";
}

/// Application identity constants following KDE reverse-domain naming
pub mod app {
    /// Reverse-domain application ID (used in .desktop file, metainfo, icon name)
    pub const APP_ID: &str = "org.kde.kurrent";
    /// Human-readable application name
    pub const APP_NAME: &str = "Kurrent Terminal";
    /// Desktop file name (matches org.kde.kurrent.desktop)
    pub const DESKTOP_FILE_NAME: &str = "org.kde.kurrent";
}

/// Common DBus paths
pub mod paths {
    pub const KWIN: &str = "/KWin";
    pub const KWALLET: &str = "/modules/kwalletd5";
    pub const ACTIVITY_MANAGER: &str = "/ActivityManager/Activities";
    pub const NOTIFICATIONS: &str = "/org/freedesktop/Notifications";
}

/// Common DBus interfaces
pub mod interfaces {
    pub const KWALLET: &str = "org.kde.KWallet";
    pub const ACTIVITIES: &str = "org.kde.ActivityManager.Activities";
    pub const NOTIFICATIONS: &str = "org.freedesktop.Notifications";
}

/// Detect KDE Plasma version from environment
pub fn detect_plasma_version() -> Option<String> {
    std::env::var("KDE_SESSION_VERSION").ok()
}

/// Check if running in KDE Plasma
pub fn is_kde_plasma() -> bool {
    if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
        desktop.to_uppercase().contains("KDE")
    } else {
        false
    }
}

/// Check if running on Wayland
pub fn is_wayland() -> bool {
    std::env::var("WAYLAND_DISPLAY").is_ok()
}

/// Check if running on X11
pub fn is_x11() -> bool {
    std::env::var("DISPLAY").is_ok() && !is_wayland()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_constants() {
        assert_eq!(services::KWIN, "org.kde.KWin");
        assert_eq!(services::NOTIFICATIONS, "org.freedesktop.Notifications");
    }

    #[test]
    fn test_path_constants() {
        assert_eq!(paths::NOTIFICATIONS, "/org/freedesktop/Notifications");
        assert_eq!(paths::ACTIVITY_MANAGER, "/ActivityManager/Activities");
    }

    #[test]
    fn test_interface_constants() {
        assert_eq!(interfaces::NOTIFICATIONS, "org.freedesktop.Notifications");
        assert_eq!(interfaces::KWALLET, "org.kde.KWallet");
    }

    #[test]
    fn test_desktop_detection() {
        let _ = is_kde_plasma();
        let _ = is_wayland();
        let _ = is_x11();
    }

    #[test]
    fn test_detect_plasma_version() {
        // Just ensure it doesn't panic
        let _ = detect_plasma_version();
    }

    #[test]
    fn test_app_identity_constants() {
        assert_eq!(app::APP_ID, "org.kde.kurrent");
        assert_eq!(app::DESKTOP_FILE_NAME, "org.kde.kurrent");
        assert!(app::APP_NAME.contains("Kurrent"));
    }

    #[test]
    fn test_is_service_available_no_panic() {
        // Will return false in CI (no DBus session), but should not panic
        let _ = is_service_available("org.freedesktop.Notifications");
    }
}

//! KNotifications Integration
//!
//! This module provides native desktop notifications using the
//! freedesktop Notifications DBus interface, with KDE-specific hints.
//!
//! ## DBus Interface
//!
//! Service: org.freedesktop.Notifications
//! Path: /org/freedesktop/Notifications
//! Interface: org.freedesktop.Notifications

use anyhow::{Context, Result};
use std::collections::HashMap;

use crate::dbus as kde_dbus;

/// Notification urgency levels (matching org.freedesktop.Notifications)
#[derive(Debug, Clone, Copy)]
pub enum Urgency {
    Low = 0,
    Normal = 1,
    Critical = 2,
}

/// A notification to display
#[derive(Debug, Clone)]
pub struct Notification {
    pub title: String,
    pub body: String,
    pub icon: Option<String>,
    pub urgency: Urgency,
    /// Timeout in milliseconds. -1 for server default, 0 for no timeout.
    pub timeout: Option<i32>,
    pub actions: Vec<NotificationAction>,
}

#[derive(Debug, Clone)]
pub struct NotificationAction {
    pub id: String,
    pub label: String,
}

impl Notification {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            icon: Some("utilities-terminal".to_string()),
            urgency: Urgency::Normal,
            timeout: None,
            actions: Vec::new(),
        }
    }

    pub fn with_urgency(mut self, urgency: Urgency) -> Self {
        self.urgency = urgency;
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_timeout(mut self, timeout_ms: i32) -> Self {
        self.timeout = Some(timeout_ms);
        self
    }

    pub fn with_action(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.actions.push(NotificationAction {
            id: id.into(),
            label: label.into(),
        });
        self
    }
}

/// Show a notification via the freedesktop Notifications DBus interface.
///
/// Returns the notification ID assigned by the server, or 0 if the DBus
/// call could not be made (e.g. no session bus available).
pub fn show_notification(notification: &Notification) -> Result<u32> {
    let conn = match kde_dbus::session_connection() {
        Ok(c) => c,
        Err(_) => {
            log::warn!(
                "No DBus session bus; notification not shown: {}",
                notification.title
            );
            return Ok(0);
        }
    };

    let proxy = zbus::blocking::Proxy::new(
        &conn,
        kde_dbus::services::NOTIFICATIONS,
        kde_dbus::paths::NOTIFICATIONS,
        kde_dbus::interfaces::NOTIFICATIONS,
    )
    .context("Failed to create Notifications proxy")?;

    // Build actions list: [id1, label1, id2, label2, ...]
    let actions: Vec<&str> = notification
        .actions
        .iter()
        .flat_map(|a| vec![a.id.as_str(), a.label.as_str()])
        .collect();

    // Build hints with urgency
    let mut hints: HashMap<&str, zbus::zvariant::Value> = HashMap::new();
    hints.insert(
        "urgency",
        zbus::zvariant::Value::U8(notification.urgency as u8),
    );

    let icon = notification.icon.as_deref().unwrap_or("");
    let timeout = notification.timeout.unwrap_or(-1);

    let reply: u32 = proxy
        .call(
            "Notify",
            &(
                "Kurrent",           // app_name
                0u32,                // replaces_id
                icon,                // app_icon
                &*notification.title, // summary
                &*notification.body,  // body
                &actions[..],        // actions
                &hints,              // hints
                timeout,             // expire_timeout
            ),
        )
        .unwrap_or(0);

    Ok(reply)
}

/// Close a notification by ID
pub fn close_notification(id: u32) -> Result<()> {
    let conn = kde_dbus::session_connection()?;
    let proxy = zbus::blocking::Proxy::new(
        &conn,
        kde_dbus::services::NOTIFICATIONS,
        kde_dbus::paths::NOTIFICATIONS,
        kde_dbus::interfaces::NOTIFICATIONS,
    )
    .context("Failed to create Notifications proxy")?;

    let _: () = proxy
        .call("CloseNotification", &(id,))
        .context("Failed to close notification")?;
    Ok(())
}

/// Get notification capabilities from the notification server
pub fn get_capabilities() -> Result<Vec<String>> {
    let conn = kde_dbus::session_connection()?;
    let proxy = zbus::blocking::Proxy::new(
        &conn,
        kde_dbus::services::NOTIFICATIONS,
        kde_dbus::paths::NOTIFICATIONS,
        kde_dbus::interfaces::NOTIFICATIONS,
    )
    .context("Failed to create Notifications proxy")?;

    let caps: Vec<String> = proxy
        .call("GetCapabilities", &())
        .context("Failed to get capabilities")?;
    Ok(caps)
}

/// Check if Do Not Disturb mode is active via the Plasma shell interface.
///
/// Returns `false` if the Plasma shell service is unreachable (e.g. non-KDE).
pub fn is_dnd_active() -> Result<bool> {
    let conn = match kde_dbus::session_connection() {
        Ok(c) => c,
        Err(_) => return Ok(false),
    };
    let proxy = match zbus::blocking::Proxy::new(
        &conn,
        "org.kde.plasmashell",
        "/org/freedesktop/Notifications",
        "org.freedesktop.Notifications",
    ) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };

    // Plasma exposes inhibition state; if it fails, assume DND is off
    let inhibited: bool = proxy
        .get_property("Inhibited")
        .unwrap_or_else(|e| {
            log::debug!("Could not read DND inhibition state: {}", e);
            false
        });
    Ok(inhibited)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_builder() {
        let notif = Notification::new("Test", "Body")
            .with_urgency(Urgency::Normal)
            .with_action("open", "Open Terminal");

        assert_eq!(notif.title, "Test");
        assert_eq!(notif.body, "Body");
        assert_eq!(notif.actions.len(), 1);
        assert_eq!(notif.actions[0].id, "open");
        assert_eq!(notif.actions[0].label, "Open Terminal");
    }

    #[test]
    fn test_notification_with_icon_and_timeout() {
        let notif = Notification::new("T", "B")
            .with_icon("custom-icon")
            .with_timeout(5000);

        assert_eq!(notif.icon.as_deref(), Some("custom-icon"));
        assert_eq!(notif.timeout, Some(5000));
    }

    #[test]
    fn test_show_notification_no_dbus() {
        // In CI without a DBus session, this should return Ok(0) gracefully
        let notif = Notification::new("Test", "Body");
        let result = show_notification(&notif);
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_dnd_active_no_plasma() {
        // Without Plasma, DND should be reported as inactive
        let active = is_dnd_active().unwrap();
        assert!(!active);
    }
}

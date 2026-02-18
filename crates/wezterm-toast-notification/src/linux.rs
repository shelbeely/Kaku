// Linux toast notification implementation using freedesktop notifications.
//
// Sends notifications via `notify-send` (provided by libnotify-bin on most
// Linux distributions). Falls back to a log message if the command is not
// available.

use crate::ToastNotification;
use std::process::Command;

pub fn show_notif(notif: ToastNotification) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("notify-send");
    cmd.arg("--app-name=Kurrent");

    if let Some(ref timeout) = notif.timeout {
        let millis = timeout.as_millis();
        cmd.arg(format!("--expire-time={}", millis));
    }

    cmd.arg(&notif.title);
    cmd.arg(&notif.message);

    match cmd.output() {
        Ok(output) if output.status.success() => {
            if let Some(url) = notif.url.as_deref() {
                // Open the URL in the default browser after showing the notification
                wezterm_open_url::open_url(url);
            }
            Ok(())
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::warn!("notify-send failed: {}", stderr);
            Ok(())
        }
        Err(_) => {
            // notify-send not installed; just log the notification
            log::info!("Notification: {} - {}", notif.title, notif.message);
            Ok(())
        }
    }
}

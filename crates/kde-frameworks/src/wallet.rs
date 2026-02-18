//! KWallet Integration
//!
//! Secure credential storage using KDE's KWallet system.
//! Supports both KWallet5 (Plasma 5) and KWallet6 (Plasma 6) via DBus.
//!
//! ## DBus Interface
//!
//! Service: org.kde.kwalletd5 or org.kde.kwalletd6
//! Path: /modules/kwalletd5 or /modules/kwalletd6
//! Interface: org.kde.KWallet

use anyhow::{bail, Context, Result};

use crate::dbus as kde_dbus;

const KURRENT_FOLDER: &str = "Kurrent";
const APP_ID: &str = "kurrent";

/// Which KWallet daemon version is available
enum WalletService {
    V6,
    V5,
}

impl WalletService {
    fn service_name(&self) -> &'static str {
        match self {
            Self::V6 => kde_dbus::services::KWALLET6,
            Self::V5 => kde_dbus::services::KWALLET5,
        }
    }

    fn object_path(&self) -> &'static str {
        match self {
            Self::V6 => "/modules/kwalletd6",
            Self::V5 => "/modules/kwalletd5",
        }
    }
}

/// Detect which KWallet daemon is running, preferring V6.
fn detect_wallet_service() -> Option<WalletService> {
    if kde_dbus::is_service_available(kde_dbus::services::KWALLET6) {
        Some(WalletService::V6)
    } else if kde_dbus::is_service_available(kde_dbus::services::KWALLET5) {
        Some(WalletService::V5)
    } else {
        None
    }
}

/// KWallet connection handle
pub struct KWallet {
    handle: i32,
    proxy: zbus::blocking::Proxy<'static>,
}

impl KWallet {
    /// Open connection to KWallet via DBus.
    ///
    /// Returns an error if no KWallet daemon is available.
    pub fn open() -> Result<Self> {
        let service = detect_wallet_service()
            .context("No KWallet daemon (kwalletd5 or kwalletd6) is available")?;

        let conn = kde_dbus::session_connection()?;
        let proxy = zbus::blocking::Proxy::new(
            &conn,
            service.service_name(),
            service.object_path(),
            kde_dbus::interfaces::KWALLET,
        )
        .context("Failed to create KWallet proxy")?;

        // Open the default wallet (usually "kdewallet")
        let handle: i32 = proxy
            .call("open", &("kdewallet", 0i64, APP_ID))
            .context("Failed to open KWallet")?;

        if handle < 0 {
            bail!("KWallet returned invalid handle {}", handle);
        }

        // Ensure the Kurrent folder exists
        let has_folder: bool = proxy
            .call("hasFolder", &(handle, KURRENT_FOLDER, APP_ID))
            .unwrap_or_else(|e| {
                log::warn!("Failed to check KWallet folder existence: {}", e);
                false
            });
        if !has_folder {
            let created: bool = proxy
                .call("createFolder", &(handle, KURRENT_FOLDER, APP_ID))
                .unwrap_or_else(|e| {
                    log::warn!("Failed to create KWallet folder '{}': {}", KURRENT_FOLDER, e);
                    false
                });
            if !created {
                log::warn!("KWallet folder '{}' could not be created", KURRENT_FOLDER);
            }
        }

        Ok(Self { handle, proxy })
    }

    /// Check if any KWallet daemon is available on DBus
    pub fn is_available() -> bool {
        detect_wallet_service().is_some()
    }

    /// Store a password/secret in the Kurrent folder
    pub fn store_password(&self, key: &str, value: &str) -> Result<()> {
        let result: i32 = self
            .proxy
            .call(
                "writePassword",
                &(self.handle, KURRENT_FOLDER, key, value, APP_ID),
            )
            .context("Failed to write password to KWallet")?;
        if result != 0 {
            bail!("KWallet writePassword returned error code {}", result);
        }
        Ok(())
    }

    /// Retrieve a password/secret from the Kurrent folder
    pub fn get_password(&self, key: &str) -> Result<Option<String>> {
        let value: String = self
            .proxy
            .call(
                "readPassword",
                &(self.handle, KURRENT_FOLDER, key, APP_ID),
            )
            .context("Failed to read password from KWallet")?;
        if value.is_empty() {
            Ok(None)
        } else {
            Ok(Some(value))
        }
    }

    /// Delete a password/secret from the Kurrent folder
    pub fn delete_password(&self, key: &str) -> Result<()> {
        let result: i32 = self
            .proxy
            .call(
                "removeEntry",
                &(self.handle, KURRENT_FOLDER, key, APP_ID),
            )
            .context("Failed to delete entry from KWallet")?;
        if result != 0 {
            bail!("KWallet removeEntry returned error code {}", result);
        }
        Ok(())
    }

    /// List all stored keys in the Kurrent folder
    pub fn list_keys(&self) -> Result<Vec<String>> {
        let keys: Vec<String> = self
            .proxy
            .call("entryList", &(self.handle, KURRENT_FOLDER, APP_ID))
            .context("Failed to list entries in KWallet")?;
        Ok(keys)
    }
}

impl Drop for KWallet {
    fn drop(&mut self) {
        // Close the wallet handle; ignore errors during cleanup
        let _: Result<i32, _> = self
            .proxy
            .call("close", &(self.handle, false, APP_ID));
    }
}

/// Credential types that can be stored in KWallet
#[derive(Debug)]
pub enum CredentialType {
    /// OpenAI API key
    OpenAiApiKey,
    /// Anthropic (Claude) API key
    AnthropicApiKey,
    /// GitHub token
    GitHubToken,
    /// Custom credential
    Custom(String),
}

impl CredentialType {
    pub fn key(&self) -> String {
        match self {
            Self::OpenAiApiKey => "openai_api_key".to_string(),
            Self::AnthropicApiKey => "anthropic_api_key".to_string(),
            Self::GitHubToken => "github_token".to_string(),
            Self::Custom(name) => name.clone(),
        }
    }
}

/// High-level API for storing AI coding assistant credentials
pub fn store_ai_credential(cred_type: CredentialType, value: &str) -> Result<()> {
    let wallet = KWallet::open()?;
    wallet.store_password(&cred_type.key(), value)
}

/// High-level API for retrieving AI coding assistant credentials
pub fn get_ai_credential(cred_type: CredentialType) -> Result<Option<String>> {
    let wallet = KWallet::open()?;
    wallet.get_password(&cred_type.key())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_type_keys() {
        assert_eq!(CredentialType::OpenAiApiKey.key(), "openai_api_key");
        assert_eq!(CredentialType::AnthropicApiKey.key(), "anthropic_api_key");
        assert_eq!(CredentialType::GitHubToken.key(), "github_token");
        assert_eq!(CredentialType::Custom("test".to_string()).key(), "test");
    }

    #[test]
    fn test_kwallet_is_available_no_panic() {
        // In CI without KWallet, returns false
        let available = KWallet::is_available();
        assert!(!available);
    }

    #[test]
    fn test_kwallet_open_no_daemon() {
        // Without a running KWallet daemon, open() should return a clear error
        let result = KWallet::open();
        assert!(result.is_err());
    }
}

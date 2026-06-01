//! Wi-Fi network connectivity handling via NetworkManager.
//!
//! This module provides both asynchronous and synchronous interfaces to interact
//! with the system's NetworkManager daemon. It automates the discovery of wireless 
//! interfaces, handles configuration generation for WPA2-PSK networks, and triggers 
//! network activation.
/**
* apply_wifi.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use networkmanager::NetworkManager;
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during the Wi-Fi interface discovery or activation process.
#[derive(Error, Debug)]
pub enum WifiError {
    /// Returned when the system does not possess an active or visible wireless network interface.
    #[error("No Wi-Fi device found on the system")]
    NoWifiDevice,

    /// Wraps errors returned directly by the NetworkManager D-Bus API.
    #[error("NetworkManager internal error: {0}")]
    NMError(String),

    /// Returned when the underlying asynchronous execution environment fails to initialize.
    #[error("Failed to initialize local async runtime: {0}")]
    RuntimeError(String),
}

/// Connects to a Wi-Fi network asynchronously.
///
/// This function queries NetworkManager for an available wireless interface,
/// constructs a runtime connection profile mapping the target SSID and WPA2-PSK pre-shared key,
/// and sequentially registers and activates the connection profile.
///
/// # Arguments
///
/// * `ssid` - A string slice representing the network's Service Set Identifier.
/// * `password` - A string slice containing the WPA2 passphrase (Pre-Shared Key).
///
/// # Errors
///
/// Returns a [`WifiError::NoWifiDevice`] if no compliant wireless hardware is discovered.
/// Returns a [`WifiError::NMError`] if the NetworkManager daemon rejects the generated 
/// profile configuration or network authentication fails.
///
/// # Example
///
/// ```rust
/// # tokio_test::block_on(async {
/// #     // Mock execution wrapper
/// #     let result = amos_wifi::connect_to_wifi_async("AtlantisNet", "secure_pass123").await;
/// # });
/// ```
pub async fn connect_to_wifi_async(ssid: &str, password: &str) -> Result<(), WifiError> {
    let nm = NetworkManager::new();

    // locate the first available Wi-Fi device interface
    let devices = nm.get_devices().await
        .map_err(|e| WifiError::NMError(e.to_string()))?;

    let wifi_device = devices
        .into_iter()
        .find(|d| d.device_type() == networkmanager::devices::DeviceType::Wifi)
        .ok_or(WifiError::NoWifiDevice)?;

    // build the nested NetworkManager connection setting maps
    let mut connection_settings = HashMap::new();

    // wireless infrastructure configuration
    let mut wifi_section = HashMap::new();
    wifi_section.insert("ssid".to_string(), ssid.as_bytes().to_vec().into());
    wifi_section.insert("mode".to_string(), "infrastructure".into());
    connection_settings.insert("802-11-wireless".to_string(), wifi_section);

    // wireless Security configuration (WPA2-PSK)
    let mut security_section = HashMap::new();
    security_section.insert("key-mgmt".to_string(), "wpa-psk".into());
    security_section.insert("auth-alg".to_string(), "open".into());
    security_section.insert("psk".to_string(), password.into());
    connection_settings.insert("802-11-wireless-security".to_string(), security_section);

    // base Connection settings
    let mut conn_section = HashMap::new();
    conn_section.insert("id".to_string(), format!("WiFi-{}", ssid).into());
    conn_section.insert("type".to_string(), "802-11-wireless".into());
    connection_settings.insert("connection".to_string(), conn_section);

    // register configuration changes and request activation from daemon
    nm.add_and_activate_connection(&connection_settings, &wifi_device)
        .await
        .map_err(|e| WifiError::NMError(e.to_string()))?;

    Ok(())
}

/// Connects to a Wi-Fi network synchronously.
///
/// A blocking wrapper around [`connect_to_wifi_async`]. This function initializes
/// a single-threaded local Tokio runtime instance on the calling thread to execute 
/// the underlying D-Bus network routines to completion.
///
/// # Arguments
///
/// * `ssid` - A string slice representing the network's Service Set Identifier.
/// * `password` - A string slice containing the WPA2 passphrase (Pre-Shared Key).
///
/// # Errors
///
/// In addition to the errors inherited from [`connect_to_wifi_async`], this function 
/// can return a [`WifiError::RuntimeError`] if the thread context is unable to host
/// a new Tokio reactor instance.
///
/// # Example
///
/// ```rust
/// fn main() {
///     println!("Initiating Wi-Fi handshake...");
///     match connect_to_wifi("AtlantisNet", "secure_pass123") {
///         Ok(_) => println!("Successfully authenticated and connected!"),
///         Err(e) =>決 epintln!("Network connection error: {}", e),
///     }
/// }
/// ```
pub fn connect_to_wifi(ssid: &str, password: &str) -> Result<(), WifiError> {
    // instantiate an isolated single-threaded runtime wrapper for synchronous contexts
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| WifiError::RuntimeError(e.to_string()))?;

    // block the caller thread until the async driver signals completion
    rt.block_on(connect_to_wifi_async(ssid, password))
}

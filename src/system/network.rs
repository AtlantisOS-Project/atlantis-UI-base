//! Wi-Fi network connectivity handling via NetworkManager.
//!
//! This module provides both asynchronous and synchronous interfaces to interact
//! with the system's NetworkManager daemon. It automates the discovery of wireless 
//! interfaces, handles configuration generation for WPA2-PSK networks, and triggers 
//! network activation.
pub mod apply_wifi;

// Re-Export
pub use apply_wifi::*;

/**
* toml_config.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* fn main() {
*    if let Err(e) = set_config_val("atl_test", "theme", "dark") {
*        eprintln!("Error at saving: {}", e);
*    }
*    match get_config_val("atl_test", "theme") {
*        Some(val) => println!("Not found: {}", val),
*        None => println!("Key not found!"),
*    }
*    set_config_val("atl_test", "port", "8080").unwrap();
*    let port: u16 = get_config_val("port")
*        .and_then(|v| v.parse().ok())
*        .unwrap_or(3000); // default value
*	 println!("Port: {}", port);
* }
*/

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// structure the strings for saving configurations
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AtlConfigs {
    data: HashMap<String, String>,
}

/**
* @brief Function that read a value to a key from the configuration file
*/
pub fn get_config(app_name: &str, key: &str) -> Option<String> {
    let cfg: AtlConfigs = confy::load(app_name, None).unwrap_or_default();
    cfg.data.get(key).cloned()
}

/**
* @brief Function that set the key-value pair and save the file
*/
pub fn set_config(app_name: &str, key: &str, value: &str) -> Result<(), confy::ConfyError> {
    // load existing configuration or create new 
    let mut cfg: AtlConfigs = confy::load(app_name, None).unwrap_or_default();
    
    // insert the values and update the file
    cfg.data.insert(key.to_string(), value.to_string());
    
    // write the file
    confy::store(app_name, None, cfg)
}

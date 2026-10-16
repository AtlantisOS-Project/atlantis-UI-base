// helper_tools.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A command line program that uses the helper functions (`atlbase::prelude`).
//!
//! It needs no display and no GTK window, and only touches a temporary directory:
//!
//! ```text
//! cargo run --example helper_tools
//! ```

use atlbase::prelude::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = tempfile::tempdir()?;
    let root = workspace.path().to_path_buf();
    println!("Working in {}\n", root.display());

    // ── files and directories ──────────────────────────────────────────
    println!("== File system ==");
    create_directory(root.join("logs/sys"));
    make_path_dirname(root.join("config/app.conf"))?;
    println!(
        "directory exists: {}",
        directory_exists(root.join("logs/sys"))
    );
    println!("expanded path:    {}", expand_path("~/Documents").display());

    // ── writing, appending and reading values ──────────────────────────
    println!("\n== Files ==");
    write_file(
        "app.conf",
        "# settings\nMODE=\"fast\"\n",
        Some(root.clone()),
        &["config"],
    )?;
    let conf = root.join("config/app.conf");
    let conf = conf.to_str().ok_or("path is not UTF-8")?;

    add_line_file(&true, conf, "THEME=dark")?;
    add_line_file(&true, conf, "THEME=dark")?; // already there, nothing happens
    println!("MODE  = {:?}", get_config_value(conf, "MODE"));
    println!("THEME = {:?}", get_config_value(conf, "THEME"));
    println!("clean = {:?}", trim_and_clean("  \"quoted text\"  "));

    // ── the temporary file of the application ─────────────────────────
    println!("\n== Temporary file ==");
    write_to_temp("first line\nsecond line\nthird line")?;
    remove_line_from_temp(1)?;
    println!("{}", read_from_temp()?.trim_end());
    clear_temp_file()?;

    // ── searching and cleaning up ─────────────────────────────────────
    println!("\n== Search and cleanup ==");
    println!(
        "a .conf file in config/: {}",
        search_file_directory(root.join("config"), ".conf")?
    );

    let cache: PathBuf = root.join("projects/old/cache");
    make_path(&cache)?;
    write_file("tmp.log", "log", Some(cache.clone()), &[])?;
    delete_files_with_parent(cache.to_str().ok_or("path is not UTF-8")?);
    println!(
        "cache removed, with its empty parents: {}",
        !root.join("projects").exists()
    );

    // ── settings of the application (in a temporary XDG directory) ────
    println!("\n== Settings ==");
    // SAFETY: no other thread is running yet.
    unsafe { std::env::set_var("XDG_CONFIG_HOME", root.join("xdg-config")) };
    set_config("helper-tools-demo", "theme", "dark")?;
    println!("theme = {:?}", get_config("helper-tools-demo", "theme"));

    // ── commands and the environment ──────────────────────────────────
    println!("\n== Commands ==");
    println!("`sh` installed: {}", command_exists_native("sh"));
    let out = capture_command_output(&["echo", "captured output"])?;
    println!("echo said: {}", out.trim());
    let status = run_command(&["true"])?;
    println!("`true` succeeded: {}", status.success());

    println!("\n== Environment ==");
    println!("execution environment: {}", get_execution_environment());
    let environment = check_application_environment("my-app", "/etc/apt/sources.list.d");
    println!("installation source:   {:?}", environment);
    if environment == AppEnvironment::Flatpak {
        println!("(inside a Flatpak sandbox pkexec and host terminals are not available)");
    }

    Ok(())
}

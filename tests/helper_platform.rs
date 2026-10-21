// helper_platform.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The platform layer as a program sees it: only the public names of the prelude.

use atlbase::prelude::*;

#[test]
fn the_target_is_the_one_the_test_was_built_for() {
    let target = Target::current();
    assert_eq!(target.os == Os::Linux, cfg!(target_os = "linux"));
    assert_eq!(target.arch == Arch::X86_64, cfg!(target_arch = "x86_64"));
    assert!(target.to_string().contains('-'));
}

#[test]
fn a_program_is_found_and_started_through_the_layer() {
    let sh = if cfg!(windows) { "cmd" } else { "sh" };
    assert!(find_program(sh).is_some());
    assert!(command_exists_native(sh));
    assert!(find_program("this-program-does-not-exist").is_none());

    let line = command_line("echo", ["via", "the", "layer"]);
    let output = host_command_line(&line).output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "via the layer"
    );
    // the commands of the library go through the same door
    assert_eq!(capture_command_output(&["echo", "x"]).unwrap().trim(), "x");
}

#[test]
fn a_bundled_tool_is_found_before_path() {
    let dir = tempfile::tempdir().unwrap();
    let name = format!("atl-bundled-tool{}", Os::current().exe_suffix());
    let tool = dir.path().join(&name);
    std::fs::write(&tool, "#!/bin/sh\necho bundled\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&tool, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    register_tool_dir(dir.path());
    assert_eq!(find_program("atl-bundled-tool"), Some(tool));
    #[cfg(unix)]
    assert_eq!(
        String::from_utf8_lossy(&host_command("atl-bundled-tool").output().unwrap().stdout).trim(),
        "bundled"
    );
}

#[test]
fn the_folders_of_an_application_follow_the_system() {
    let dirs = AppDirs::new("atlbase-platform-test").unwrap();
    assert!(dirs.config_file().starts_with(dirs.config_dir()));
    assert!(dirs.log_file().starts_with(dirs.state_dir()));
    assert!(home_dir().is_some());
    assert!(
        DataDirs::current()
            .data_dirs()
            .iter()
            .all(|dir| dir.is_absolute())
    );
}

#[test]
fn administrator_commands_are_built_for_every_system() {
    let command = command_line("id", ["-u"]);
    for os in [Os::Linux, Os::Windows, Os::MacOs] {
        let elevated = elevation_command_for(os, &command).unwrap();
        assert!(elevated.iter().any(|word| word.contains("id")), "{os}");
    }
    assert!(
        elevation_command(&command)
            .unwrap()
            .iter()
            .any(|word| word.contains("id"))
    );
}

#[test]
fn the_features_agree_with_the_system() {
    assert_eq!(supports(Feature::PosixShell), !cfg!(windows));
    // a feature that needs administrator rights needs the way to get them
    if supports(Feature::PackageInstall) {
        assert!(supports(Feature::Elevation));
        assert!(PackageManager::detect().is_some());
    }
    let install = PackageManager::Apt.install_command(&["adb"]);
    assert_eq!(install, ["apt-get", "install", "-y", "adb"]);
}

#[test]
fn logging_goes_where_the_system_wants_it() {
    assert_eq!(
        journal_follow_command("app").is_some(),
        cfg!(target_os = "linux")
    );
}

// program.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! External programs of the host: finding them and starting them.
//!
//! A program asks for `adb` and gets the file that runs it: `/usr/bin/adb`, or
//! `C:\tools\platform-tools\adb.exe`. Every command of the library is started through
//! [`host_command`], so the same rules apply everywhere:
//!
//! * a program is looked for in the folders registered with [`register_tool_dir`] (a bundled
//!   copy of the tools comes first), then in the folders of the package format (`/app/bin` of a
//!   Flatpak, `$SNAP/usr/bin`, `$APPDIR/usr/bin` of an AppImage: the tools that are bundled in the
//!   package are found even where `PATH` does not name them), then, on Windows, next to the
//!   program itself (as Windows does), then in `PATH`;
//! * the ending of a program file is added where the system needs it (`adb` → `adb.exe`);
//! * on Windows a command line tool started by a window program does not flash a console
//!   window (`CREATE_NO_WINDOW`).
//!
//! A program that cannot be found is not an error here: the plain name is used, and starting
//! it fails with `NotFound` as it always did.

use super::dirs::DataDirs;
use super::os::{os_profile, os_profile_for};
use super::target::Os;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

/// Folders with bundled programs, searched before `PATH`.
static TOOL_DIRS: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());

/// Adds a folder to the places a program is looked for, before `PATH`.
///
/// Use it for programs that are shipped with the application (a `platform-tools` folder next to
/// the executable of a Windows bundle). Adding the same folder twice does nothing. The folder
/// does not have to exist: it is only searched if it does.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{find_program, register_tool_dir};
///
/// let dir = tempfile::tempdir().unwrap();
/// let tool = dir.path().join(if cfg!(windows) { "my-tool.exe" } else { "my-tool" });
/// std::fs::write(&tool, "#!/bin/sh\n").unwrap();
/// # #[cfg(unix)]
/// # std::fs::set_permissions(&tool, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
///
/// register_tool_dir(dir.path());
/// assert_eq!(find_program("my-tool"), Some(tool));
/// ```
pub fn register_tool_dir(dir: impl Into<PathBuf>) {
    let dir = dir.into();
    let mut dirs = TOOL_DIRS.lock().unwrap_or_else(|p| p.into_inner());
    if !dirs.contains(&dir) {
        dirs.push(dir);
    }
}

/// The file names to try for a program on `os`: the name itself, and on Windows the name with
/// every executable ending unless it has one. `None` if `os` is not compiled in.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{Os, program_file_names};
///
/// assert!(program_file_names(Os::current(), "adb", None).unwrap()[0].starts_with("adb"));
/// ```
pub fn program_file_names(os: Os, name: &str, pathext: Option<&str>) -> Option<Vec<String>> {
    Some((os_profile_for(os)?.program_names)(name, pathext))
}

/// Whether `path` is a file that can be started.
///
/// On Unix that means a regular file with an execute bit; on Windows any file (the ending
/// decides, see [`program_file_names`]).
pub fn is_executable(path: &Path) -> bool {
    let Ok(meta) = path.metadata() else {
        return false;
    };
    if !meta.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        meta.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        true
    }
}

/// The folders with programs that belong to the package format the program runs in: `/app/bin` in
/// a Flatpak, `$SNAP/usr/bin` and `$SNAP/bin` in a Snap, `$APPDIR/usr/bin` in an AppImage.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{DataDirs, package_bin_dirs};
///
/// // outside a package there are none
/// assert!(package_bin_dirs(&DataDirs::default()).is_empty());
/// ```
pub fn package_bin_dirs(places: &DataDirs) -> Vec<PathBuf> {
    places
        .roots()
        .iter()
        .flat_map(|(format, root)| format.bin_paths(root))
        .collect()
}

/// The folders a program is looked for in: registered folders, the folders of the package format,
/// the folder of the executable (Windows only), then `PATH`.
fn search_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = TOOL_DIRS.lock().unwrap_or_else(|p| p.into_inner()).clone();
    dirs.extend(package_bin_dirs(&DataDirs::current()));
    // Windows starts a program from the folder of the running program first
    #[cfg(windows)]
    if let Some(dir) = env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf))
    {
        dirs.push(dir);
    }
    if let Some(paths) = env::var_os("PATH") {
        dirs.extend(env::split_paths(&paths));
    }
    dirs
}

/// Finds a program.
///
/// `program` is an absolute path (the file must exist and be executable) or a name that is
/// looked for in the folders described in the [module documentation](self).
///
/// # Returns
///
/// The path of the file, or `None` if there is no such program.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::find_program;
///
/// assert!(find_program("sh").is_some() || cfg!(windows));
/// assert!(find_program("this-program-does-not-exist").is_none());
/// ```
pub fn find_program(program: &str) -> Option<PathBuf> {
    let pathext = env::var("PATHEXT").ok();
    let names = (os_profile().program_names)(program, pathext.as_deref());

    if Path::new(program).is_absolute() {
        return names
            .into_iter()
            .map(PathBuf::from)
            .find(|candidate| is_executable(candidate));
    }
    search_dirs()
        .iter()
        .flat_map(|dir| names.iter().map(move |name| dir.join(name)))
        .find(|candidate| is_executable(candidate))
}

/// The file to start for a program: the one [`find_program`] finds, otherwise the name as it
/// was given.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::program_path;
/// use std::path::PathBuf;
///
/// assert_eq!(program_path("this-program-does-not-exist"), PathBuf::from("this-program-does-not-exist"));
/// ```
pub fn program_path(program: &str) -> PathBuf {
    find_program(program).unwrap_or_else(|| PathBuf::from(program))
}

/// Prepares a command for the platform: no console window on Windows.
fn apply_platform_flags(command: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // CREATE_NO_WINDOW
        command.creation_flags(0x0800_0000);
    }
    #[cfg(not(windows))]
    let _ = command;
}

/// A command for a program of the host. Use it instead of `Command::new`.
///
/// The program is resolved with [`program_path`]; add arguments and run it as usual.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::host_command;
///
/// let output = host_command("echo").arg("hello").output().unwrap();
/// assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "hello");
/// ```
pub fn host_command(program: &str) -> Command {
    let mut command = Command::new(program_path(program));
    apply_platform_flags(&mut command);
    command
}

/// A command for a command line: the program followed by its arguments, as the progress
/// dialogs and [`command_line`](crate::helper::commands::command_line::command_line) have
/// them. An empty line gives a command that fails with `NotFound` when it is started.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{command_line, host_command_line};
///
/// let line = command_line("echo", ["a", "b"]);
/// let output = host_command_line(&line).output().unwrap();
/// assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "a b");
/// ```
pub fn host_command_line(line: &[String]) -> Command {
    let mut command = host_command(line.first().map_or("", String::as_str));
    command.args(line.iter().skip(1));
    command
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_names_to_try_follow_the_system() {
        let names = |os| program_file_names(os, "adb", Some(".EXE;.CMD"));
        assert_eq!(names(Os::Windows).unwrap(), ["adb.exe", "adb.cmd"]);
        assert_eq!(names(Os::MacOs).unwrap(), ["adb"]);
        assert_eq!(names(Os::Linux).unwrap(), ["adb"]);
    }

    #[test]
    fn registered_folders_come_before_path() {
        let dir = tempfile::tempdir().unwrap();
        let name = if cfg!(windows) {
            "atl-test-tool.exe"
        } else {
            "atl-test-tool"
        };
        let tool = dir.path().join(name);
        std::fs::write(&tool, "x").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            // a file without the execute bit is not a program
            assert_eq!(find_program("atl-test-tool"), None);
            std::fs::set_permissions(&tool, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
        register_tool_dir(dir.path());
        register_tool_dir(dir.path());
        assert_eq!(find_program("atl-test-tool"), Some(tool.clone()));
        assert_eq!(program_path("atl-test-tool"), tool);
        assert_eq!(
            TOOL_DIRS
                .lock()
                .unwrap()
                .iter()
                .filter(|d| *d == dir.path())
                .count(),
            1
        );
    }

    #[test]
    fn the_tools_of_a_package_are_found_in_its_own_folders() {
        let places = DataDirs::from_lookup(&|key| match key {
            "SNAP" => Some("/snap/app/7".into()),
            "APPDIR" => Some("/tmp/.mount_x".into()),
            "FLATPAK_ID" => Some("org.example.App".into()),
            _ => None,
        });
        let dirs = package_bin_dirs(&places);
        // the formats in the order they are tried: Flatpak, Snap, AppImage
        assert_eq!(dirs[0], PathBuf::from("/app/bin"));
        assert!(dirs.contains(&PathBuf::from("/snap/app/7/usr/bin")));
        assert!(dirs.contains(&PathBuf::from("/snap/app/7/bin")));
        assert_eq!(dirs.last(), Some(&PathBuf::from("/tmp/.mount_x/usr/bin")));
        assert!(package_bin_dirs(&DataDirs::default()).is_empty());
    }

    #[test]
    fn absolute_paths_must_exist() {
        assert!(find_program("/definitely/not/here").is_none());
        #[cfg(unix)]
        assert_eq!(
            find_program("/bin/sh").is_some(),
            Path::new("/bin/sh").exists()
        );
    }

    #[test]
    fn a_command_line_becomes_a_command() {
        let line = vec!["echo".to_string(), "a".to_string(), "b c".to_string()];
        let command = host_command_line(&line);
        let args: Vec<_> = command.get_args().collect();
        assert_eq!(args, ["a", "b c"]);
        assert_eq!(
            host_command_line(&[]).spawn().unwrap_err().kind(),
            std::io::ErrorKind::NotFound
        );
    }
}

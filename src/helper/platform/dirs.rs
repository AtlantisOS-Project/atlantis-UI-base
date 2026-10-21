// dirs.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Where things are: the user's home, the folders of the program for settings, data, cache and
//! logs, the folders with the program's read-only resources (translations, icons, guides), and
//! the system folders that must never be deleted.
//!
//! | | Linux | Windows | macOS |
//! |---|---|---|---|
//! | [`AppDirs::config_dir`] | `~/.config/<app>` | `%APPDATA%\<app>\config` | `~/Library/Application Support/rs.<app>` |
//! | [`AppDirs::data_dir`] | `~/.local/share/<app>` | `%APPDATA%\<app>\data` | `~/Library/Application Support/rs.<app>` |
//! | [`AppDirs::cache_dir`] | `~/.cache/<app>` | `%LOCALAPPDATA%\<app>\cache` | `~/Library/Caches/rs.<app>` |
//! | [`AppDirs::state_dir`] | `~/.local/state/<app>` | `%LOCALAPPDATA%\<app>\data` | `~/Library/Application Support/rs.<app>` |
//!
//! The mapping is done by the `directories` crate; Flatpak and Snap redirect the XDG variables
//! into the private data of the application, so the folders follow the sandbox automatically.
//! The settings file of [`get_config`](crate::helper::configuration::toml_config::get_config)
//! is [`AppDirs::config_file`].

use super::target::Os;
use directories::{BaseDirs, ProjectDirs, UserDirs};
use std::env;
use std::ffi::OsString;
#[cfg(unix)]
use std::fs;
use std::path::{Path, PathBuf};

/// The home directory of the user.
///
/// `$HOME` wins when it is set (a Snap sets it to its private directory, a test can point it
/// anywhere); otherwise the system is asked (`USERPROFILE`, the user database).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::home_dir;
///
/// assert!(home_dir().is_some());
/// ```
pub fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .filter(|home| !home.is_empty())
        .map(PathBuf::from)
        .or_else(|| BaseDirs::new().map(|dirs| dirs.home_dir().to_path_buf()))
}

/// The folder for the user's documents (`~/Documents`, `C:\Users\<name>\Documents`), if the
/// system names one. A good place to suggest for files the user wants to find again.
pub fn documents_dir() -> Option<PathBuf> {
    UserDirs::new().and_then(|dirs| dirs.document_dir().map(Path::to_path_buf))
}

/// The folders of one application for its settings, data, cache and logs.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::AppDirs;
///
/// let dirs = AppDirs::new("my-app").unwrap();
/// // the folders are only names: nothing is created until the program writes there
/// assert!(dirs.config_file().ends_with("default-config.toml"));
/// assert!(dirs.log_file().ends_with("my-app.log"));
/// ```
#[derive(Debug, Clone)]
pub struct AppDirs {
    name: String,
    project: ProjectDirs,
}

impl AppDirs {
    /// The folders for the application `app_name`, or `None` if the system has no home
    /// directory for the user.
    pub fn new(app_name: &str) -> Option<AppDirs> {
        // "rs" is the qualifier `confy` uses: both must give the same folder
        ProjectDirs::from("rs", "", app_name).map(|project| AppDirs {
            name: app_name.to_string(),
            project,
        })
    }

    /// The folder for the settings.
    pub fn config_dir(&self) -> &Path {
        self.project.config_dir()
    }

    /// The settings file that [`get_config`](crate::helper::configuration::toml_config::get_config)
    /// and [`set_config`](crate::helper::configuration::toml_config::set_config) use.
    pub fn config_file(&self) -> PathBuf {
        confy::get_configuration_file_path(&self.name, None)
            .unwrap_or_else(|_| self.config_dir().join("default-config.toml"))
    }

    /// The folder for the data of the application.
    pub fn data_dir(&self) -> &Path {
        self.project.data_dir()
    }

    /// The folder for data that can be built again.
    pub fn cache_dir(&self) -> &Path {
        self.project.cache_dir()
    }

    /// The folder for data that belongs to this computer and is not worth a backup (logs,
    /// history). Systems without such a folder use the data folder.
    pub fn state_dir(&self) -> PathBuf {
        self.project
            .state_dir()
            .unwrap_or_else(|| self.project.data_local_dir())
            .to_path_buf()
    }

    /// The log file of the application, for systems that have no system log
    /// ([`init_logging`](crate::helper::platform::logging::init_logging) writes it).
    pub fn log_file(&self) -> PathBuf {
        self.state_dir().join(format!("{}.log", self.name))
    }
}

/// The environment the folders with read-only resources are derived from. A type of its own
/// instead of direct access to `std::env`, so the lookup can be tested without changing the
/// environment of the process.
#[derive(Clone, Debug)]
pub struct DataDirs {
    pub(crate) os: Os,
    pub(crate) home: Option<PathBuf>,
    pub(crate) xdg_data_home: Option<PathBuf>,
    pub(crate) xdg_data_dirs: Option<OsString>,
    /// `$SNAP`, set in Snap packages
    pub(crate) snap: Option<PathBuf>,
    /// `$APPDIR`, set by the AppImage runtime
    pub(crate) appdir: Option<PathBuf>,
    /// `$FLATPAK_ID` is set, so we run in a Flatpak sandbox
    pub(crate) flatpak: bool,
    /// `<prefix>` of `<prefix>/bin/<program>`: how a relocatable install (a Windows bundle, a
    /// folder in a home directory) lays out its `share`
    pub(crate) exe_prefix: Option<PathBuf>,
}

impl Default for DataDirs {
    fn default() -> Self {
        DataDirs {
            os: Os::Linux,
            home: None,
            xdg_data_home: None,
            xdg_data_dirs: None,
            snap: None,
            appdir: None,
            flatpak: false,
            exe_prefix: None,
        }
    }
}

impl DataDirs {
    /// Reads the process environment.
    ///
    /// # Examples
    ///
    /// ```
    /// use atlbase::prelude::DataDirs;
    ///
    /// // user data first, then the system folders; only absolute paths
    /// assert!(DataDirs::current().data_dirs().iter().all(|dir| dir.is_absolute()));
    /// ```
    pub fn current() -> DataDirs {
        DataDirs::from_lookup(&|key| env::var_os(key))
    }

    /// Like [`current`](DataDirs::current), but the variables come from `var`. Empty values
    /// count as unset.
    pub(crate) fn from_lookup(var: &dyn Fn(&str) -> Option<OsString>) -> DataDirs {
        let var = |key: &str| var(key).filter(|value| !value.is_empty());
        DataDirs {
            os: Os::current(),
            home: var("HOME").map(PathBuf::from),
            xdg_data_home: var("XDG_DATA_HOME").map(PathBuf::from),
            xdg_data_dirs: var("XDG_DATA_DIRS"),
            snap: var("SNAP").map(PathBuf::from),
            appdir: var("APPDIR").map(PathBuf::from),
            flatpak: var("FLATPAK_ID").is_some(),
            exe_prefix: env::current_exe()
                .ok()
                .and_then(|exe| exe.parent()?.parent().map(Path::to_path_buf)),
        }
    }

    /// The roots of relocatable installs: `$SNAP`, then `$APPDIR`. A path like
    /// `/usr/share/locale` lives below such a root.
    pub(crate) fn app_roots(&self) -> Vec<&Path> {
        [&self.snap, &self.appdir]
            .into_iter()
            .flatten()
            .map(PathBuf::as_path)
            .collect()
    }

    /// Whether the program runs in a Flatpak sandbox.
    pub(crate) fn is_flatpak(&self) -> bool {
        self.flatpak
    }

    /// The data folders of the system: `$XDG_DATA_DIRS`, or the usual ones of the operating
    /// system. Only absolute paths.
    pub(crate) fn system_dirs(&self) -> Vec<PathBuf> {
        match &self.xdg_data_dirs {
            Some(list) => env::split_paths(list)
                .filter(|dir| dir.is_absolute())
                .collect(),
            None => default_system_dirs(self.os)
                .iter()
                .map(PathBuf::from)
                .collect(),
        }
    }

    /// All folders that may hold read-only resources, in search order, without duplicates:
    /// the data folder of the user, the data folders of the system, and the folders that
    /// Flatpak, Snap, AppImage and a relocated install add.
    ///
    /// * deb/rpm package: `/usr/share` (or `/usr/local/share`)
    /// * Flatpak: `/app/share`; Flatpak usually sets `XDG_DATA_DIRS` accordingly
    /// * Snap: `$SNAP/usr/share` and `$SNAP/share`; depending on the Snap extension they are
    ///   not part of `XDG_DATA_DIRS`
    /// * AppImage: `$APPDIR/usr/share`
    /// * a relocated install (Windows bundle): `<prefix>/share` next to `<prefix>/bin`
    pub fn data_dirs(&self) -> Vec<PathBuf> {
        let mut dirs: Vec<PathBuf> = Vec::new();
        let mut push = |dir: PathBuf| {
            if dir.is_absolute() && !dirs.contains(&dir) {
                dirs.push(dir);
            }
        };

        match (&self.xdg_data_home, &self.home) {
            (Some(data_home), _) => push(data_home.clone()),
            (None, Some(home)) => push(home.join(".local/share")),
            (None, None) => {}
        }
        self.system_dirs().into_iter().for_each(&mut push);
        if self.flatpak {
            push(PathBuf::from("/app/share"));
        }
        if let Some(snap) = &self.snap {
            push(snap.join("usr/share"));
            push(snap.join("share"));
        }
        if let Some(appdir) = &self.appdir {
            push(appdir.join("usr/share"));
        }
        if let Some(prefix) = &self.exe_prefix {
            push(prefix.join("share"));
        }
        dirs
    }
}

/// The data folders of an operating system when `$XDG_DATA_DIRS` is not set.
fn default_system_dirs(os: Os) -> &'static [&'static str] {
    match os {
        Os::Windows => &[],
        Os::MacOs => &["/usr/local/share", "/opt/homebrew/share", "/usr/share"],
        Os::Linux | Os::Other => &["/usr/local/share", "/usr/share"],
    }
}

/// Folders that must never be deleted, nor anything below them (system folders).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{Os, critical_paths};
///
/// assert!(critical_paths(Os::Linux).contains(&"/usr"));
/// assert!(critical_paths(Os::Windows).contains(&"C:\\Windows"));
/// ```
pub fn critical_paths(os: Os) -> &'static [&'static str] {
    match os {
        Os::Windows => &[
            "C:\\Windows",
            "C:\\Program Files",
            "C:\\Program Files (x86)",
            "C:\\ProgramData",
        ],
        Os::MacOs => &[
            "/System",
            "/Library",
            "/Applications",
            "/bin",
            "/sbin",
            "/usr",
            "/etc",
            "/var",
            "/private",
            "/dev",
            "/cores",
        ],
        Os::Linux | Os::Other => &[
            "/bin", "/sbin", "/usr", "/etc", "/var", "/proc", "/sys", "/dev", "/lib", "/lib32",
            "/lib64", "/boot", "/root",
        ],
    }
}

/// Folders that must not be deleted themselves; what is inside is not protected.
pub fn protected_dirs(os: Os) -> &'static [&'static str] {
    match os {
        Os::Windows => &["C:\\Users"],
        Os::MacOs => &["/Users", "/Volumes", "/tmp", "/opt"],
        Os::Linux | Os::Other => &[
            "/home", "/tmp", "/opt", "/mnt", "/media", "/run", "/srv", "/app", "/snap",
        ],
    }
}

/// Folders in the home directory that must not be deleted.
pub fn standard_home_folders(os: Os) -> &'static [&'static str] {
    match os {
        Os::Windows => &[
            "Desktop",
            "Documents",
            "Downloads",
            "Music",
            "Pictures",
            "Videos",
            "AppData",
        ],
        Os::MacOs => &[
            "Desktop",
            "Documents",
            "Downloads",
            "Music",
            "Pictures",
            "Movies",
            "Library",
            "Applications",
            ".config",
        ],
        Os::Linux | Os::Other => &[
            "Desktop",
            "Documents",
            "Downloads",
            "Music",
            "Pictures",
            "Videos",
            "Public",
            "Templates",
            ".config",
            ".local",
        ],
    }
}

/// Whether `path` is a mount point (another file system starts there). Always `false` on
/// systems where the question has no meaning for the program (Windows drives are roots).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::is_mount_point;
///
/// assert!(!is_mount_point(std::path::Path::new("/definitely/not/there")));
/// ```
pub fn is_mount_point(path: &Path) -> bool {
    on_other_device_than_parent(path).unwrap_or(false)
}

/// `Some(true)` if `path` is on another file system than its parent; `None` if that cannot
/// be told (no parent, an error, or a system without device numbers).
#[cfg(unix)]
fn on_other_device_than_parent(path: &Path) -> Option<bool> {
    use std::os::unix::fs::MetadataExt;
    let parent = path.parent().filter(|p| !p.as_os_str().is_empty())?;
    Some(fs::metadata(path).ok()?.dev() != fs::metadata(parent).ok()?.dev())
}

#[cfg(not(unix))]
fn on_other_device_than_parent(_path: &Path) -> Option<bool> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lookup(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<OsString> {
        let pairs: Vec<(String, String)> = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |key| {
            pairs
                .iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| OsString::from(v))
        }
    }

    fn dirs_of(pairs: &[(&str, &str)]) -> DataDirs {
        DataDirs {
            exe_prefix: None,
            ..DataDirs::from_lookup(&lookup(pairs))
        }
    }

    #[test]
    fn data_dirs_follow_the_package_format() {
        let dirs = DataDirs {
            os: Os::Linux,
            ..dirs_of(&[
                ("HOME", "/home/me"),
                ("XDG_DATA_DIRS", "/opt/a::/usr/share:/opt/a:relative"),
                ("FLATPAK_ID", "org.example.App"),
                ("SNAP", "/snap/x/1"),
                ("APPDIR", "/tmp/.mount_x"),
            ])
        };
        let found: Vec<_> = dirs.data_dirs();
        let found: Vec<_> = found.iter().map(|dir| dir.to_str().unwrap()).collect();
        assert_eq!(
            found,
            [
                "/home/me/.local/share",
                "/opt/a",
                "/usr/share",
                "/app/share",
                "/snap/x/1/usr/share",
                "/snap/x/1/share",
                "/tmp/.mount_x/usr/share",
            ]
        );
    }

    #[test]
    fn the_defaults_depend_on_the_operating_system() {
        let linux = DataDirs {
            os: Os::Linux,
            ..dirs_of(&[])
        };
        assert_eq!(
            linux.data_dirs(),
            [
                PathBuf::from("/usr/local/share"),
                PathBuf::from("/usr/share")
            ]
        );
        let mac = DataDirs {
            os: Os::MacOs,
            ..dirs_of(&[])
        };
        assert!(
            mac.data_dirs()
                .contains(&PathBuf::from("/opt/homebrew/share"))
        );
        let windows = DataDirs {
            os: Os::Windows,
            exe_prefix: Some(PathBuf::from("/app-root")),
            ..dirs_of(&[])
        };
        // a relocated install is found through the program
        assert_eq!(windows.data_dirs(), [PathBuf::from("/app-root/share")]);
    }

    #[test]
    fn app_roots_come_in_a_fixed_order() {
        let dirs = dirs_of(&[("APPDIR", "/b"), ("SNAP", "/a")]);
        assert_eq!(dirs.app_roots(), [Path::new("/a"), Path::new("/b")]);
        assert!(!dirs.is_flatpak());
    }

    #[test]
    fn app_dirs_and_the_settings_file_agree() {
        let dirs = AppDirs::new("atlbase-dirs-test").unwrap();
        let file = dirs.config_file();
        assert_eq!(file.parent(), Some(dirs.config_dir()));
        assert!(dirs.log_file().ends_with("atlbase-dirs-test.log"));
        assert!(AppDirs::new("x").unwrap().state_dir().is_absolute());
    }

    #[test]
    fn every_system_has_protected_folders() {
        for os in [Os::Linux, Os::Windows, Os::MacOs, Os::Other] {
            assert!(!critical_paths(os).is_empty());
            assert!(!protected_dirs(os).is_empty());
            assert!(standard_home_folders(os).contains(&"Documents"));
        }
    }

    #[test]
    fn a_root_and_a_plain_folder_are_not_mount_points() {
        assert!(!is_mount_point(Path::new("/")));
        let dir = tempfile::tempdir().unwrap();
        assert!(!is_mount_point(dir.path()));
    }
}

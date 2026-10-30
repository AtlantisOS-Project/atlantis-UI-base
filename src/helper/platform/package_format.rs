// package_format.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The package format a program runs in: Flatpak, Snap, AppImage, or none of them (a deb, an RPM,
//! `cargo install`).
//!
//! A [`PackageFormat`] describes everything the library needs to know about a format: how it is
//! recognized, where the program is mounted, where its programs and shared files are, and whether
//! it keeps the program away from the host. The rest of the library asks the format and never
//! tests `$SNAP` or `/.flatpak-info` itself.
//!
//! | File | PackageFormat | Cargo feature |
//! |---|---|---|
//! | `package_format/flatpak.rs` | Flatpak | `format-flatpak` |
//! | `package_format/snap.rs` | Snap | `format-snap` |
//! | `package_format/appimage.rs` | AppImage | `format-appimage` |
//! | `package_format/native.rs` | no format: deb, RPM, `cargo install` | `format-native` |
//!
//! **Only the formats of the enabled features are compiled.** A Snap is built with
//! `--no-default-features --features format-snap`: the code of Flatpak and AppImage is not in the
//! program, and the program does not ask at run time what it is. With several features (the
//! default: all) the format is found out at start. At least one feature is needed.
//!
//! # Adding a format
//!
//! 1. Copy the closest file in `package_format/` (for example `appimage.rs`) to
//!    `package_format/<name>.rs` and fill in its `FORMAT`.
//! 2. Add the feature `format-<name>` to `Cargo.toml` (and to the feature `default`).
//! 3. Add the module and one line to [`PACKAGE_FORMATS`] below, with the `cfg` of the feature.
//!
//! Nothing else changes: the folders, the locale lookup, the icon lookup, the capabilities and the
//! detection all read the fields of the [`PackageFormat`].

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

#[cfg(feature = "format-appimage")]
mod appimage;
#[cfg(feature = "format-flatpak")]
mod flatpak;
#[cfg(feature = "format-native")]
mod native;
#[cfg(feature = "format-snap")]
mod snap;

/// Looks up an environment variable; empty values count as unset. Tests pass their own.
pub type Lookup<'a> = &'a dyn Fn(&str) -> Option<OsString>;

/// The description of a package format (see the [module documentation](self)).
#[derive(Debug)]
pub struct PackageFormat {
    /// A short lower case name: `flatpak`, `snap`, `appimage`, `native`.
    pub id: &'static str,
    /// The name for people: `Flatpak`, `Snap`, …
    pub label: &'static str,
    /// Whether the format keeps the program away from the programs and files of the host: there is
    /// no `pkexec`, no terminal of the host, no system log.
    pub sandboxed: bool,
    /// Whether the program is mounted below a root that is only known at run time (Snap,
    /// AppImage): an absolute path that was compiled in (`/usr/share/locale`) lives below it.
    pub relocated: bool,
    /// Whether this is the format: `detect(var)`.
    pub detect: fn(Lookup) -> bool,
    /// The folder the program is mounted at (`$SNAP`, `$APPDIR`, `/app`), if the format has one.
    pub root: fn(Lookup) -> Option<PathBuf>,
    /// The folders with programs below the root: `["usr/bin", "bin"]`.
    pub bin_dirs: &'static [&'static str],
    /// The folders with shared files (translations, icons, guides) below the root.
    pub share_dirs: &'static [&'static str],
}

impl PackageFormat {
    /// The programs of the format, as absolute paths below `root`.
    pub fn bin_paths(&self, root: &std::path::Path) -> Vec<PathBuf> {
        self.bin_dirs.iter().map(|dir| root.join(dir)).collect()
    }

    /// The shared files of the format, as absolute paths below `root`.
    pub fn share_paths(&self, root: &std::path::Path) -> Vec<PathBuf> {
        self.share_dirs.iter().map(|dir| root.join(dir)).collect()
    }
}

/// The formats that are compiled in, in the order they are tried. The format that is no format
/// (`native`) is last.
///
/// To add a format, add its module above and one line here.
pub static PACKAGE_FORMATS: &[&PackageFormat] = &[
    #[cfg(feature = "format-flatpak")]
    &flatpak::FORMAT,
    #[cfg(feature = "format-snap")]
    &snap::FORMAT,
    #[cfg(feature = "format-appimage")]
    &appimage::FORMAT,
    #[cfg(feature = "format-native")]
    &native::FORMAT,
];

#[cfg(not(any(
    feature = "format-flatpak",
    feature = "format-snap",
    feature = "format-appimage",
    feature = "format-native"
)))]
compile_error!(
    "atlbase needs at least one package format: enable one of the features format-flatpak, \
     format-snap, format-appimage or format-native (the default has all of them)"
);

/// The format the program runs in, found out from the environment of the process; not remembered.
///
/// With one format compiled in, that is the answer without a look at the environment.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::detect_package_format;
///
/// // nothing in the environment: no sandbox
/// let format = detect_package_format(&|_| None);
/// assert!(!format.sandboxed);
///
/// // $FLATPAK_ID is what a Flatpak sets
/// let in_flatpak = |key: &str| (key == "FLATPAK_ID").then(|| "org.example.App".into());
/// println!("{}", detect_package_format(&in_flatpak).label);
/// ```
pub fn detect_package_format(var: Lookup) -> &'static PackageFormat {
    detect_package_format_at(var, std::env::current_exe().ok().as_deref())
}

/// The name of the environment variable that chooses the format: `ATL_PACKAGE_FORMAT=snap` (the ids
/// of the formats: `flatpak`, `snap`, `appimage`, `native`). It wins over everything else, so a
/// program can be tried as if it ran in a package (`-debug -snp` sets it) and a development build
/// can be told that it runs natively. A name that is not compiled in is ignored.
pub const FORMAT_OVERRIDE: &str = "ATL_PACKAGE_FORMAT";

/// Like [`detect_package_format`], with the place of the program given: `exe` is the executable
/// (`None`: do not check where it is, which is what the tests of a format want).
///
/// The format of a relocated package (Snap, AppImage) counts only if the program lives below its
/// root. That matters more often than it seems: a program that was started by a program of a Snap
/// (`cargo run` with the `cargo` of the rustup Snap, a terminal of a Snap) inherits `$SNAP`
/// without being in the Snap.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{PACKAGE_FORMATS, detect_package_format_at};
/// use std::path::Path;
///
/// let snap = |key: &str| (key == "SNAP").then(|| "/snap/my-app/7".into());
/// if PACKAGE_FORMATS.len() > 1 {
///     // started by a program of a Snap (`cargo run` with the cargo of a Snap): not this Snap
///     let elsewhere = Path::new("/home/me/target/debug/my-app");
///     assert_ne!(detect_package_format_at(&snap, Some(elsewhere)).id, "snap");
///     // below the root of the Snap: the Snap
///     let inside = Path::new("/snap/my-app/7/usr/bin/my-app");
///     assert_eq!(detect_package_format_at(&snap, Some(inside)).id, "snap");
/// }
/// ```
pub fn detect_package_format_at(var: Lookup, exe: Option<&Path>) -> &'static PackageFormat {
    if let [only] = PACKAGE_FORMATS {
        return only;
    }
    if let Some(chosen) = var(FORMAT_OVERRIDE)
        .and_then(|name| PACKAGE_FORMATS.iter().find(|format| name == format.id))
    {
        return chosen;
    }
    PACKAGE_FORMATS
        .iter()
        .find(|format| (format.detect)(var) && lives_in(format, var, exe))
        .or_else(|| PACKAGE_FORMATS.last())
        .copied()
        .expect("at least one package format")
}

/// Whether the program at `exe` is below the root of `format` (always true for a format that is
/// not relocated, or when the place is not known).
fn lives_in(format: &PackageFormat, var: Lookup, exe: Option<&Path>) -> bool {
    let (Some(exe), true, Some(root)) = (exe, format.relocated, (format.root)(var)) else {
        return true;
    };
    // `/snap` may be a link to `/var/lib/snapd/snap`: compare the real paths as well
    let real = |path: &Path| path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    exe.starts_with(&root) || real(exe).starts_with(real(&root))
}

/// The format the program runs in, found out once.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::current_package_format;
///
/// println!("running as {}", current_package_format().label);
/// ```
pub fn current_package_format() -> &'static PackageFormat {
    static CURRENT: OnceLock<&'static PackageFormat> = OnceLock::new();
    CURRENT.get_or_init(|| detect_package_format(&|key| std::env::var_os(key)))
}

/// The roots of all formats that report one (the folders the programs of a package live below),
/// in the order of [`PACKAGE_FORMATS`].
///
/// # Examples
///
/// ```
/// use atlbase::prelude::package_roots;
///
/// let in_snap = |key: &str| (key == "SNAP").then(|| "/snap/my-app/7".into());
/// for (format, root) in package_roots(&in_snap) {
///     assert_eq!(format.id, "snap");
///     assert_eq!(root, std::path::PathBuf::from("/snap/my-app/7"));
/// }
/// assert!(package_roots(&|_| None).is_empty());
/// ```
pub fn package_roots(var: Lookup) -> Vec<(&'static PackageFormat, PathBuf)> {
    PACKAGE_FORMATS
        .iter()
        .filter_map(|format| (format.root)(var).map(|root| (*format, root)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lookup(pairs: &'static [(&'static str, &'static str)]) -> impl Fn(&str) -> Option<OsString> {
        move |key| {
            pairs
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| OsString::from(*v))
        }
    }

    #[test]
    fn the_format_follows_the_environment() {
        assert_eq!(
            detect_package_format_at(&lookup(&[("SNAP", "/snap/x/1")]), None).id,
            "snap"
        );
        assert_eq!(
            detect_package_format_at(&lookup(&[("APPIMAGE", "/x.AppImage")]), None).id,
            "appimage"
        );
        assert_eq!(
            detect_package_format_at(&lookup(&[("FLATPAK_ID", "org.x.App")]), None).id,
            "flatpak"
        );
        // Flatpak wins over the others, and nothing at all is no format
        assert_eq!(
            detect_package_format_at(
                &lookup(&[("FLATPAK_ID", "org.x.App"), ("SNAP", "/s")]),
                None
            )
            .id,
            "flatpak"
        );
        assert_eq!(detect_package_format_at(&lookup(&[]), None).id, "native");
    }

    #[test]
    fn an_inherited_snap_is_not_the_snap_of_the_program() {
        // `cargo run` with the cargo of the rustup Snap: $SNAP is set, the program is somewhere else
        let snap = lookup(&[("SNAP", "/snap/rustup/1504")]);
        let elsewhere = Path::new("/home/me/app/target/debug/app");
        assert_eq!(
            detect_package_format_at(&snap, Some(elsewhere)).id,
            "native"
        );
        let inside = Path::new("/snap/rustup/1504/usr/bin/app");
        assert_eq!(detect_package_format_at(&snap, Some(inside)).id, "snap");

        let image = lookup(&[("APPIMAGE", "/x.AppImage"), ("APPDIR", "/tmp/.mount_x")]);
        assert_eq!(
            detect_package_format_at(&image, Some(elsewhere)).id,
            "native"
        );
        let inside = Path::new("/tmp/.mount_x/usr/bin/app");
        assert_eq!(
            detect_package_format_at(&image, Some(inside)).id,
            "appimage"
        );

        // a Flatpak sandbox is a sandbox wherever the program is
        let flatpak = lookup(&[("FLATPAK_ID", "org.x.App")]);
        assert_eq!(
            detect_package_format_at(&flatpak, Some(elsewhere)).id,
            "flatpak"
        );
    }

    #[test]
    fn the_format_can_be_chosen() {
        let forced = lookup(&[(FORMAT_OVERRIDE, "snap")]);
        assert_eq!(
            detect_package_format_at(&forced, Some(Path::new("/x/app"))).id,
            "snap"
        );
        // it wins over the environment
        let native = lookup(&[(FORMAT_OVERRIDE, "native"), ("SNAP", "/snap/x/1")]);
        assert_eq!(detect_package_format_at(&native, None).id, "native");
        // a name that does not exist is ignored
        let unknown = lookup(&[(FORMAT_OVERRIDE, "nonsense"), ("SNAP", "/snap/x/1")]);
        assert_eq!(detect_package_format_at(&unknown, None).id, "snap");
    }

    #[test]
    fn roots_and_folders_come_from_the_format() {
        let roots = package_roots(&lookup(&[
            ("SNAP", "/snap/x/1"),
            ("APPDIR", "/tmp/.mount_x"),
        ]));
        let found: Vec<_> = roots
            .iter()
            .map(|(f, r)| (f.id, r.to_str().unwrap()))
            .collect();
        assert_eq!(
            found,
            [("snap", "/snap/x/1"), ("appimage", "/tmp/.mount_x")]
        );
        let (snap, root) = &roots[0];
        assert_eq!(
            snap.share_paths(root),
            [
                PathBuf::from("/snap/x/1/usr/share"),
                PathBuf::from("/snap/x/1/share")
            ]
        );
        assert!(snap.sandboxed && snap.relocated);
        assert!(package_roots(&lookup(&[])).is_empty());
    }

    #[test]
    fn every_format_has_a_unique_name() {
        let mut ids: Vec<_> = PACKAGE_FORMATS.iter().map(|format| format.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), PACKAGE_FORMATS.len());
    }
}

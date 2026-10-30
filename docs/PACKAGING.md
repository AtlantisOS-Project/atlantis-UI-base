# Packaging applications that use AtlBase

AtlBase is an ordinary Rust crate. It is linked into your application, so every package
format works the same way: you package **your application**, and the library comes with it.
This page lists what the library needs in each format and how it behaves at run time.

## Versions you need

| Component | Needed | Where you get it |
|---|---|---|
| Rust | 1.93 or newer | rustup, Fedora 43+, Arch, the Flatpak Rust SDK extension |
| GTK | 4.20 or newer | GNOME runtime 49, Fedora 43, Ubuntu 25.10, Arch |
| Libadwaita | 1.8 or newer | the same |
| VTE (GTK 4) | only with the feature `log-viewer` | `libvte-2.91-gtk4` |
| `msgfmt` (gettext) | optional, at build time | `gettext` |

Without `msgfmt` the build still works (it prints a warning) but the translations of the
library are missing. Always install `gettext` in your build environment.

## Features and packaging

| Feature | Default | Packaging effect |
|---|---|---|
| `carousel-toml` | on | pulls in the `toml` crate; no system libraries |
| `log-viewer` | on | needs VTE. The GNOME Flatpak runtime has **no** VTE for GTK 4; either build VTE as a Flatpak module or turn the feature off |
| `archive` | off | pulls in the archive crates; the C code of `bzip2` is compiled in |
| `download` | off | `download_file`, `download_file_verified` with `ureq` and rustls (bundled root certificates) and `sha2`; no system library, no `curl` |
| `format-flatpak`, `format-snap`, `format-appimage`, `format-native` | all on | the package formats that are compiled in; see below |
| `experimental-api` | off | do not use yet; needs `libdbus-1` |

```toml
# without the log viewer and without VTE, built for a Flatpak
atlbase = { version = "0.2", default-features = false, features = ["carousel-toml", "format-flatpak"] }
```

## One build per package format

Snap, Flatpak and AppImage are three descriptions of the same kind (`PackageFormat`, one file each in
`src/helper/platform/package_format/`): where the program is mounted, where its programs and shared
files are, how it is recognized, whether it is sandboxed. The library has no other code for them. A
format is compiled in only when its Cargo feature is on, so a package contains nothing of the others:

| You build | Features | What is left out |
|---|---|---|
| deb, RPM, `cargo install` | `format-native` | Flatpak, Snap, AppImage |
| Flatpak | `format-flatpak` (no `log-viewer`: the GNOME runtime has no VTE) | Snap, AppImage, native |
| Snap | `format-snap` (no `log-viewer`) | Flatpak, AppImage, native |
| AppImage | `format-appimage` | Flatpak, Snap, native |
| Windows | any one; no Linux code is compiled for Windows at all | everything that is Linux |

```sh
cargo build --release --no-default-features --features format-snap,carousel-toml
```

With one format compiled in the program does not ask at run time what it is. With several (the
default) it finds out at start. At least one is required (`compile_error!` otherwise). In your own
`Cargo.toml` pass the choice on:

```toml
[features]
format-snap = ["atlbase/format-snap"]
format-flatpak = ["atlbase/format-flatpak"]
```

**A new format** is one file (copy `appimage.rs`), one line in `PACKAGE_FORMATS` and a feature; nothing
else in the library changes.

## Translations

* The texts **of the library** are compiled into the library. You do not install anything for
  them; at start-up `init_language` writes them to the cache directory of the user
  (`$XDG_CACHE_HOME/atlbase`, which Flatpak and Snap place inside the sandbox).
  A distribution may still install `atlbase.mo` into the locale directory; that copy is
  preferred.
* The texts **of your application** are your own catalogs (`<domain>.mo`). Install them where
  your build system points `LOCALEDIR`, and pass that directory to `init_language`:

  ```rust
  // deb/rpm: /usr/share/locale    Flatpak: /app/share/locale    Snap/AppImage: see below
  init_language("my-app", "/usr/share/locale", false);
  ```

  Inside a **Snap** the directory is searched below `$SNAP`, inside an **AppImage** below
  `$APPDIR`, so the same path works in every format.
* `ATL_LOCALEDIR=/path` forces a directory (useful to test a package).
* Text of your application overrides the text of the library: translate the same `msgid`
  in your catalog.

## Debian / Ubuntu (deb)

```text
Build-Depends: debhelper-compat (= 13), cargo, rustc (>= 1.93), pkg-config, gettext,
 libgtk-4-dev (>= 4.20), libadwaita-1-dev (>= 1.8), libvte-2.91-gtk4-dev
Depends: ${shlibs:Depends}, ${misc:Depends}
```

Debian builds without network access: run `cargo vendor` and add the `[source]` replacement
to `.cargo/config.toml`, or use `dh-cargo`. Install the catalogs of your application to
`usr/share/locale/<lang>/LC_MESSAGES/<domain>.mo`.

## Fedora / openSUSE (rpm)

```text
BuildRequires: cargo rust >= 1.93 pkgconfig gettext
BuildRequires: pkgconfig(gtk4) >= 4.20 pkgconfig(libadwaita-1) >= 1.8 pkgconfig(vte-2.91-gtk4)
```

Use `rust2rpm` or `%cargo_vendor_manifest` for the vendored sources. The run-time
dependencies (`gtk4`, `libadwaita`, `vte291-gtk4`) are found automatically from the linked
libraries.

A release profile with `strip = true` leaves nothing for a debuginfo package: add
`%global debug_package %{nil}` to the spec. Tested on Fedora 43 (GTK 4.20, Libadwaita 1.8, Rust 1.99):
the package installs in a clean system and `rpm -V` finds nothing changed.

## Flatpak

```yaml
runtime: org.gnome.Platform
runtime-version: '49'
sdk: org.gnome.Sdk
sdk-extensions: [org.freedesktop.Sdk.Extension.rust-stable]
build-options:
  append-path: /usr/lib/sdk/rust-stable/bin
  env:
    CARGO_HOME: /run/build/my-app/cargo
finish-args: [--socket=wayland, --socket=fallback-x11, --share=ipc, --device=dri]
modules:
  # only with the feature `log-viewer`: VTE for GTK 4 is not part of the runtime
  - name: vte
    buildsystem: meson
    config-opts: [-Dgtk3=false, -Dgtk4=true, -Dvapi=false, -Dgir=false]
    sources: [{type: archive, url: "…", sha256: "…"}]
  - name: my-app
    buildsystem: simple
    build-commands:
      - cargo --offline fetch --manifest-path Cargo.toml --verbose
      - cargo --offline build --release --verbose
      - install -Dm755 target/release/my-app /app/bin/my-app
```

Generate the vendored sources with `flatpak-cargo-generator.py Cargo.lock`. Inside the sandbox:

| Function | Behavior |
|---|---|
| `get_execution_environment()` | returns `"flatpak"` (detected by `/.flatpak-info`) |
| `set_config` / `get_config` | stored in `~/.var/app/<id>/config` |
| file and folder choosers | use the portal, no extra permission |
| `open_url` | uses the OpenURI portal |
| `pkexec` helpers, host terminals | not available; hide them |
| `init_logging` | `/dev/log` is not available: the log file (`~/.var/app/<id>/.local/state/<app>/<app>.log`) is the only log |
| `show_log_viewer` | shows that file; needs VTE, which the GNOME runtime lacks: build without the feature `log-viewer` |

## Snap

```yaml
base: core24
confinement: strict
parts:
  my-app:
    plugin: rust
    source: .
    build-packages: [pkg-config, gettext, libgtk-4-dev, libadwaita-1-dev, libvte-2.91-gtk4-dev]
    stage-packages: [libvte-2.91-gtk4-0]
apps:
  my-app:
    command: bin/my-app
    extensions: [gnome]
    plugs: [home, network]
```

Pick a GNOME extension whose GTK and Libadwaita are new enough (see the table above).
`$HOME` is the private directory of the snap. `get_execution_environment()` returns
`"snap"`; strict confinement cannot use `pkexec`. Translations are searched in
`$SNAP/usr/share/locale` and `$SNAP/share/locale`.

## AppImage

Bundle GTK, Libadwaita and VTE with `linuxdeploy` and its GTK plugin. The AppImage runtime sets
`$APPIMAGE` and `$APPDIR`; `get_execution_environment()` returns `"appimage"`, translations
are searched in `$APPDIR/usr/share/locale`, and images of guides in `$APPDIR/usr/share`. Put
the catalogs of your application into `AppDir/usr/share/locale`.

What a GTK 4.20 / Libadwaita AppImage needs besides the libraries (tested with a program that
runs in a container without GTK):

* **Icons:** the symbolic icons of the buttons come from the Adwaita icon theme, which only GNOME
  systems have: copy `/usr/share/icons/Adwaita` (without `cursors`) into `AppDir/usr/share/icons`.
  The icon of your program goes to `AppDir/usr/share/icons/hicolor/…`; `ensure_app_icon` finds it
  there (it searches `$APPDIR/usr/share/icons`), so the "About" dialog and the windows show it.
* **Images:** GTK 4.20 loads PNG and SVG through glycin. Copy `/usr/libexec/glycin-loaders` and
  `/usr/share/glycin-loaders` into the AppDir, give the loaders to `linuxdeploy` as
  `--executable`, and point the `Exec=` lines of the `.conf` files to `$APPDIR` in an `AppRun` hook.
* **Hook of the GTK plugin:** it forces `GDK_BACKEND=x11` and `GTK_THEME`; remove both lines
  (Libadwaita brings its own style, and GTK 4 works on Wayland).
* **Libraries:** `linuxdeploy` leaves out libraries that "every desktop has"; add what Pango needs
  and the system may lack with `--library` (`libfribidi.so.0`). Use `NO_STRIP=1` when the
  `strip` of the build system is older than its libraries.
* **Build on the oldest system you support:** an AppImage runs on systems with the same or a
  newer C library than the one it was built on.

## Packages for another architecture (arm64 on an x86_64 computer)

Without root (no `binfmt_misc`) and without an arm64 computer a container with the cross compiler
and the arm64 development packages builds the program (Ubuntu: `crossbuild-essential-arm64`,
`libgtk-4-dev:arm64`, `libadwaita-1-dev:arm64`, `PKG_CONFIG_LIBDIR=/usr/lib/aarch64-linux-gnu/pkgconfig`,
`rustup target add aarch64-unknown-linux-gnu`). The formats then only pack the result:

* **deb / rpm:** `dpkg-deb` and `rpmbuild --target aarch64` (with a spec that skips `%build` and
  takes the binary as a source) pack files of any architecture; `%global __os_install_post %{nil}`
  keeps rpm from running the `strip` of the build system on them.
* **AppImage:** `linuxdeploy` cannot look into arm64 files. Collect the libraries with the `NEEDED`
  entries of `readelf -d` (see `appimage/bundle_libs.py` of the Fastboot-Assistant) and pack with
  `appimagetool` and `runtime-aarch64`.
* **Flatpak:** put `files/` and `metadata` together by hand, `flatpak build-finish`,
  `build-export --arch=aarch64` and `build-bundle`; no build command of the other architecture is run.
* **Snap:** `snapcraft pack --destructive-mode --build-for arm64` with the plugin `dump` on the
  cross compiled files. `stage-packages` would take the packages of the architecture of the
  machine, so put the libraries you need into the files yourself.
* **Test:** `qemu-aarch64 -L <sysroot>` runs the program against the libraries it will find at run
  time (the arm64 GNOME runtime of Flatpak, a Fedora aarch64 tree unpacked from the rpm files, the
  arm64 libraries of the container).

The name of a system library can differ between distributions (`libbz2.so.1.0` on Debian and
Ubuntu, `libbz2.so.1` on Fedora): a binary built against one does not start on the other. Link such
libraries statically (the `static` feature of the `bzip2` crate, which `atlbase` uses for the
feature `archive`) or build on the distribution you package for.

## Builds that work on any system

The Rust code is the same on every architecture (the compiler emits only the code for the
target; there is nothing to leave out with `cfg(target_arch)`). What differs is where the
packages get things from. Keep that in **one file per format**:

* **Snap:** `platforms: {amd64: {build-on: [amd64], build-for: [amd64]}, arm64: …}` and
  `on amd64:` / `on arm64:` lines for `source`, `stage-packages` and `build-packages`. Each
  architecture builds on its own hardware (Launchpad, `snapcraft remote-build`, an arm64 computer).
* **Flatpak:** `only-arches: [x86_64]` / `[aarch64]` on the modules (for example Google's
  platform-tools on x86_64 and `android-tools` with its `googletest` and `protobuf` built from
  source on aarch64).
* **deb:** `Architecture: any`; Launchpad builds amd64 and arm64.

To make a build work on a computer that has nothing installed:

* Compile in a container that is built from a `Containerfile` in the repository (fixed base
  image, `rustup-init` with a checked SHA-256 sum, a fixed toolchain version); the host needs only
  `podman`. Run the build scripts through one helper that starts them in the container (the
  Fastboot-Assistant has `scripts/container.sh`).
* Download tools with a version and a SHA-256 sum, never `continuous` or `master`. A pinned,
  older tool can have bugs the newest one fixed: the `patchelf 0.15` inside
  `linuxdeploy 1-alpha-20251107-1` overwrote the initialization code of `libcrypt.so.2`, so the
  AppImage crashed when it started; unpack linuxdeploy and put the `patchelf` of the build
  environment into it.
* Make the source tarball deterministic (`tar --sort=name --owner=0 --group=0 --numeric-owner
  --mtime=@<time of the last commit>`), vendor the crates and give a source package (`.orig.tar.xz`
  with `vendor/`).
* Prove the Launchpad build: install exactly the `Build-Depends` into a bare container of the
  series, run `dpkg-buildpackage` there with `--network=none` as an unprivileged user.
* While `atlbase` is not on crates.io, fetch it with git from a tag or commit, and do it in the
  Snap build, too, so it builds from the repository.

## Checklist

1. `cargo build --release --locked` works offline (vendored sources).
2. `gettext` is installed when building.
3. The dependency list contains GTK ≥ 4.20 and Libadwaita ≥ 1.8 (and VTE, unless you
   disabled `log-viewer`).
4. The catalogs of your application are installed where you pass `LOCALEDIR` to
   `init_language`.
5. Buttons that need `pkexec`, a host terminal or `journalctl` are hidden in Flatpak and Snap.

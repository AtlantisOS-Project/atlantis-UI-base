# Packaging applications that use AtlBase

AtlBase is an ordinary Rust crate. It is linked into your application, so every package
format works the same way: you package **your application**, and the library comes with it.
This page lists what the library needs in each format and how it behaves at run time.

## Versions you need

| Component | Needed | Where you get it |
|---|---|---|
| Rust | 1.92 or newer | rustup, Fedora 43+, Arch, the Flatpak Rust SDK extension |
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
| `experimental-api` | off | do not use yet; needs `libdbus-1` |

```toml
# without the log viewer and without VTE
atlbase = { version = "0.2", default-features = false, features = ["carousel-toml"] }
```

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
Build-Depends: debhelper-compat (= 13), cargo, rustc (>= 1.92), pkg-config, gettext,
 libgtk-4-dev (>= 4.20), libadwaita-1-dev (>= 1.8), libvte-2.91-gtk4-dev
Depends: ${shlibs:Depends}, ${misc:Depends}
```

Debian builds without network access: run `cargo vendor` and add the `[source]` replacement
to `.cargo/config.toml`, or use `dh-cargo`. Install the catalogs of your application to
`usr/share/locale/<lang>/LC_MESSAGES/<domain>.mo`.

## Fedora / openSUSE (rpm)

```text
BuildRequires: cargo rust >= 1.92 pkgconfig gettext
BuildRequires: pkgconfig(gtk4) >= 4.20 pkgconfig(libadwaita-1) >= 1.8 pkgconfig(vte-2.91-gtk4)
```

Use `rust2rpm` or `%cargo_vendor_manifest` for the vendored sources. The run-time
dependencies (`gtk4`, `libadwaita`, `vte291-gtk4`) are found automatically from the linked
libraries.

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
| `init_syslog`, journal viewer | `/dev/log` and `journalctl` are not available; messages are dropped |

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

## Checklist

1. `cargo build --release --locked` works offline (vendored sources).
2. `gettext` is installed when building.
3. The dependency list contains GTK ≥ 4.20 and Libadwaita ≥ 1.8 (and VTE, unless you
   disabled `log-viewer`).
4. The catalogs of your application are installed where you pass `LOCALEDIR` to
   `init_language`.
5. Buttons that need `pkexec`, a host terminal or `journalctl` are hidden in Flatpak and Snap.

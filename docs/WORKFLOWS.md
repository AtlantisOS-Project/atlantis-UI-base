# Workflows and mechanisms of atlbase

How the work on `atlbase` goes, what runs automatically, and how the parts that are not obvious from
the code fit together. The API is in [DEVELOPER_DOCS.md](DEVELOPER_DOCS.md), the modules in
[MODULES_OVERVIEW.md](MODULES_OVERVIEW.md), the packages in [PACKAGING.md](PACKAGING.md).

## 1. Everyday work

```sh
cargo build                                   # all default features
cargo test --features archive,download        # unit tests, tests/, the examples of the documentation
cargo clippy --all-targets --features archive,download -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps                              # the default features: what `cargo doc` shows
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --features archive,download   # and with the optional ones
rustfmt --edition 2024 src/helper/file/add_line_file.rs    # one file at a time
scripts/check-platform.sh                     # the platform layer for Linux and Windows, without GTK
```

* **Never run a bare `cargo fmt`**: it follows `#[cfg]` and reformats `src/system*`, which is not to
  be touched (feature `experimental-api`, does not compile yet).
* **The GUI tests need a display.** Without one they print "skipped" and succeed. On a
  computer with a desktop `cargo test` opens windows on it: use Broadway instead.

  ```sh
  gtk4-broadwayd :9 &
  GDK_BACKEND=broadway BROADWAY_DISPLAY=:9 cargo test --features archive,download
  ```
* **One format only:** a program is built for one package format (`--no-default-features --features
  format-snap,carousel-toml`). Check that every format still compiles and passes clippy:
  `for f in flatpak snap appimage native; do cargo clippy --no-default-features --features
  format-$f,carousel-toml,archive -- -D warnings; done`.
* The checks of a security audit (and what to read) are in
  [SECURITY_AUDIT.md](SECURITY_AUDIT.md#6-how-to-run-an-audit-and-update-this-file).

## 2. Adding something

| What | What belongs to it |
|---|---|
| A public function or type | rustdoc with a summary, `# Arguments`, `# Errors`, `# Examples` (the example is compiled and run by `cargo test --doc`; one that needs a window is `no_run`), a test in `tests/` or next to the code, an entry in `tests/preludes.rs`, the export in `prelude` or `ui_prelude` (`src/lib.rs`), a row in [DEVELOPER_DOCS.md](DEVELOPER_DOCS.md) and, for a new module, in [MODULES_OVERVIEW.md](MODULES_OVERVIEW.md), and an entry in the changelog |
| A text a user reads | `gettext!("…")`, then `po/update-pot.sh` and the German text in `po/de.po` (fix `fuzzy` entries by hand); the library texts are compiled into the crate |
| An operating system or a package format | one file, one line in a list and (for a format) a Cargo feature: see [the platform layer](DEVELOPER_DOCS.md#helperplatform) |
| A function that writes a log | `log::info!`/`warn!`/`error!`/`debug!`; see [Logging](DEVELOPER_DOCS.md#logging) |
| A new example | a file in `examples/`; a row in the table of the [README](../README.md#example-applications); `required-features` in `Cargo.toml` if it needs one |

Before something goes into a pull request: `cargo test`, `cargo clippy --all-targets` and
`cargo doc` without warnings, formatted files, and the changelog.

## 3. The changelog

The text is in `CHANGELOG/`: one file per series (`v.0.2.x.md`) and `CHANGELOG/README.md`, which has
the intro and the list of the files, newest first. New entries go into the newest file (the tables
are aligned by hand: keep that). The same files make the web page `docs/index.html`: it reads the
list, loads every file in it (from `CHANGELOG/`, `../CHANGELOG/`, then raw.githubusercontent.com)
and shows the versions in a bar on the left. `cargo test --test changelog` checks that the list
and the files fit together. For GitHub Pages serve the folder `docs/`.

## 4. What runs automatically

| What | When | What it does |
|---|---|---|
| `.github/workflows/audit.yml` | every push to `main`, every pull request, every Monday | `cargo audit` (the RustSec database) on the dependencies as an application that uses `atlbase` would get them: the repository has no `Cargo.lock`, so they are resolved first. A vulnerability fails the run, a warning (unmaintained, unsound) is only shown. The action is pinned to a commit. |
| `.github/dependabot.yml` | every week | Pull requests for newer versions of the crates and of the actions of the workflows |
| `scripts/check-platform.sh` | by hand | Copies the platform layer into a small crate and checks it for Linux and for `x86_64-pc-windows-gnu` (and other targets you name) |
| `codeql-scan` | by hand | A CodeQL run on the sources; the results are in `~/.cache/codeql-scan/` |

## 5. Publishing a version

A list of what to look at, in this order:

1. The changelog: the heading says the version and the date (not `unreleased`), the upgrade guide
   names every change that needs a change in the code of a user, and "Known issues" is up to date.
2. `Cargo.toml`: `version`, `rust-version`, the `include` list (new folders or files that belong to the
   crate: `docs/*.md` and `CHANGELOG/*.md` are in), and `[package.metadata.docs.rs]` (the features
   docs.rs builds: `carousel-toml`, `archive`, `download`).
3. The version in the examples of `README.md` and of [PACKAGING.md](PACKAGING.md) (`atlbase = "0.2"`).
4. `cargo test --features archive,download`, the clippy and doc commands above, the check for every
   format, `scripts/check-platform.sh`, `cargo audit`.
5. `cargo package --list` (is everything in that must be, nothing that must not) and
   `cargo publish --dry-run`.
6. A tag, then `cargo publish`.

## 6. Mechanisms

**Only the code that runs is compiled.** The profile of the operating system the program is built for
is the only one in the program (`cfg(target_os)`; the tests compile all of them). Of the package
formats only those of the enabled `format-*` features are compiled in. With several (the default) the
format is found out at start; with one it is known at compile time.

**Which package format am I in?** From the environment: `/.flatpak-info` or `$FLATPAK_ID`, `$SNAP`,
`$APPIMAGE`/`$APPDIR`. A Snap or an AppImage counts only if the program itself lives below
`$SNAP` or `$APPDIR` (a program started by a program of a Snap inherits `$SNAP`).
`ATL_PACKAGE_FORMAT=snap|flatpak|appimage|native` chooses by hand and wins; `-debug -snp` and
`-debug -flp` set it.

**Where things are.** One implementation each: `home_directory()` reads `$HOME`; `AppDirs` gives the
folders of an application; `DataDirs` the folders with read-only resources (translations, icons,
guides) in search order for every format; `find_program` and `host_command` find and start programs
(bundled folders first, then the folders of the format, then `PATH`).

**Commands.** `run_command_chain` runs the commands of every progress dialog one after another in a
worker thread (`spawn_background`), logs each command with its result and time, and the dialog
closes by itself, also if the work panics.

**Logging.** `init_logging` installs a Tee: the log file on every system (UTC time, level, short
module name; rotated at 1 MB) and syslog where it can be reached. The log viewer follows the file
with `tail -F`. Status dialogs log what the user is told.

**Translations.** The texts of the library are compiled into the crate (`build.rs` embeds the `.mo`
files); `gettext!` looks in the text domain of the application first, then in the domain of the
library, so an application can override any text. The locale directory is searched in the places
that belong to the package format.

**Memory.** Closures that are connected to a widget must not hold the widget (`glib::clone!` with
`#[weak]`); `tests/design_gtk.rs` opens and closes every dialog and checks with weak references
that nothing is left.

**Security.** Everything that takes data from outside (archives, downloads, files, command output,
environment variables) is read like an attacker wrote it: no path leaves the folder it should
stay in, no link of an archive is followed, no size is trusted. The audits, their scale and what is
open are in [SECURITY_AUDIT.md](SECURITY_AUDIT.md).

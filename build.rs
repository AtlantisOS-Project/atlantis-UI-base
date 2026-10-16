// build.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Build script: compiles the translations of the library.
//!
//! * Every `po/<lang>.po` is compiled with `msgfmt` to
//!   `$OUT_DIR/locale/<lang>/LC_MESSAGES/<crate name>.mo`.
//! * The compiled catalogs are compiled into the library as well
//!   (`$OUT_DIR/embedded_catalogs.rs`). That way the library texts are translated no matter
//!   how the application is packaged (Flatpak, Snap, deb, rpm, AppImage, `cargo install`),
//!   without the package having to install the `.mo` files.
//! * `COMPILED_LOCALE_DIR` points at the compiled catalogs and is meant for development
//!   (`env!("COMPILED_LOCALE_DIR")` inside the examples and tests of this crate).
//!
//! `msgfmt` comes from the `gettext` package. It is optional: without it (e.g. on docs.rs)
//! the build only prints a warning and the library texts stay English.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let domain = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME is set by cargo");
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("set by cargo"));
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("set by cargo"));

    let po_dir = manifest_dir.join("po");
    let locale_root = out_dir.join("locale");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=po");
    println!("cargo:rustc-env=LIB_DOMAIN={domain}");
    println!(
        "cargo:rustc-env=COMPILED_LOCALE_DIR={}",
        locale_root.display()
    );

    let catalogs = compile_po_files(&po_dir, &locale_root, &domain);
    write_embedded_list(&out_dir.join("embedded_catalogs.rs"), &catalogs);
}

/// Compiles all `*.po` files in `po_dir`; returns `(language, path of the .mo file)`.
fn compile_po_files(po_dir: &Path, locale_root: &Path, domain: &str) -> Vec<(String, PathBuf)> {
    let Ok(entries) = fs::read_dir(po_dir) else {
        return Vec::new();
    };

    let mut po_files: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("po"))
        .collect();
    // reproducible builds: the order of read_dir is arbitrary
    po_files.sort();

    let mut catalogs = Vec::new();
    for po_file in po_files {
        let Some(lang) = po_file.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };

        let dest_dir = locale_root.join(lang).join("LC_MESSAGES");
        fs::create_dir_all(&dest_dir).expect("cannot create the locale output directory");
        let mo_file = dest_dir.join(format!("{domain}.mo"));

        let status = Command::new("msgfmt")
            .arg("--check-format")
            .arg(&po_file)
            .arg("-o")
            .arg(&mo_file)
            .status();

        match status {
            Ok(status) if status.success() => catalogs.push((lang.to_string(), mo_file)),
            Ok(_) => panic!("msgfmt rejected {}", po_file.display()),
            Err(err) => {
                println!(
                    "cargo:warning=msgfmt not found ({err}); the translations are skipped. \
                     Install the `gettext` package to get them."
                );
                return Vec::new();
            }
        }
    }
    catalogs
}

/// Writes the Rust source that embeds the compiled catalogs (see `helper::language`).
fn write_embedded_list(dest: &Path, catalogs: &[(String, PathBuf)]) {
    let mut source = String::from("static EMBEDDED_CATALOGS: &[(&str, &[u8])] = &[\n");
    for (lang, mo_file) in catalogs {
        // `{:?}` on a str yields a valid, escaped Rust string literal
        source.push_str(&format!(
            "    ({:?}, include_bytes!({:?})),\n",
            lang,
            mo_file.display().to_string()
        ));
    }
    source.push_str("];\n");
    fs::write(dest, source).expect("cannot write embedded_catalogs.rs");
}

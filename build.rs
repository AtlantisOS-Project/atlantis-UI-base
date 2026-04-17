/**
* build.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Build that to create and compile the language files
*
* Usage:
* fn main() {
*    // in real usecase use "/usr/share/locale"
*    // for tests
*    let test_dir = env!("COMPILED_LOCALE_DIR");
*    language::init_language(language::LIB_DOMAIN, test_dir, false);
*    println!("{}", lib_t!("Hello"));
* }
*/

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // extract metadata
    let domain = env::var("CARGO_PKG_NAME").expect("Cargo Name nicht gefunden");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Manifest Dir nicht gefunden");
    let out_dir = env::var("OUT_DIR").expect("Out Dir nicht gefunden");
    
    let po_path = Path::new(&manifest_dir).join("po");
    let locale_out_root = PathBuf::from(&out_dir).join("locale");

    // pass the domain to the compiler
    println!("cargo:rustc-env=LIB_DOMAIN={}", domain);
    println!("cargo:rustc-env=COMPILED_LOCALE_DIR={}", locale_out_root.display());

    // compile th .mo files
    if po_path.exists() {
        // say cargo that /po require a rebuild
        println!("cargo:rerun-if-changed=po");
        compile_po_files(&po_path, &locale_out_root, &domain);
    }
}


// function that comile the .po to .mo files
fn compile_po_files(po_dir: &Path, out_root: &Path, domain: &str) {
    let entries = fs::read_dir(po_dir).expect("Konnte po-Ordner nicht lesen");

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("po") {
            let lang = path.file_stem().expect("Kein Dateiname").to_str().expect("Ungültiges UTF-8");
            
            let dest_dir = out_root.join(lang).join("LC_MESSAGES");
            fs::create_dir_all(&dest_dir).expect("Unable to create the destination directory");
            
            let mo_file = dest_dir.join(format!("{}.mo", domain));

            let status = Command::new("msgfmt")
                .arg("-v")
                .arg(&path)
                .arg("-o")
                .arg(&mo_file)
                .status()
                .expect("The msgfmt tool was not found. Please install ‘gettext’.");

            if !status.success() {
                panic!("msgfmt File error: {:?}", path);
            }
        }
    }
}

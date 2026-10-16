// carousel_toml.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The TOML guide of the examples loads and its image is found.

#![cfg(feature = "carousel-toml")]

use atlbase::ui_prelude::*;
use std::path::PathBuf;

fn guide_file() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/assets/guide.toml")
}

#[test]
fn the_example_guide_loads() {
    let guide = GuideLoader::new().load_file(guide_file()).unwrap();

    assert_eq!(guide.title, "GSI");
    assert_eq!(guide.pages.len(), 4);
    assert_eq!(guide.pages[1].subtitle.as_deref(), Some("Preparation"));
    assert!(guide.pages.iter().all(|page| !page.blocks.is_empty()));
}

#[test]
fn the_image_is_found_next_to_the_guide() {
    let guide = GuideLoader::new().load_file(guide_file()).unwrap();

    let image = guide.pages[0]
        .blocks
        .iter()
        .find_map(|block| match block {
            Block::Image(image) => Some(image),
            _ => None,
        })
        .expect("the first page has an image");

    match &image.source {
        ImageSource::File(path) => assert!(path.is_file(), "{} exists", path.display()),
        other => panic!("expected a file, got {other:?}"),
    }
    assert!(image.alt.is_some(), "images need an alternative text");
}

#[test]
fn errors_name_the_problem() {
    let error = GuideLoader::new().load_str("title = \"T\"\n").unwrap_err();
    assert!(error.to_string().contains("no pages"), "{error}");

    let error = GuideLoader::new()
        .load_file("/does/not/exist.toml")
        .unwrap_err();
    assert!(matches!(error, GuideError::Io { .. }), "{error}");
}

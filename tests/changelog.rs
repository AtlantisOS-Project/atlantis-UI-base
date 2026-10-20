// changelog.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The changelog lives in `CHANGELOG/`: one file per version series (`v.0.2.x.md`) and a
//! `README.md` that lists the files, newest first. `docs/index.html` builds the web page from
//! them, so the list and the files must fit together.

use std::fs;
use std::path::PathBuf;

fn folder() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("CHANGELOG")
}

/// `v.0.2.x.md` -> `(0, 2)`
fn series(name: &str) -> Option<(u32, u32)> {
    let rest = name.strip_prefix("v.")?.strip_suffix(".x.md")?;
    let (major, minor) = rest.split_once('.')?;
    Some((major.parse().ok()?, minor.parse().ok()?))
}

/// The files the README lists, in the order of the list.
fn listed_files() -> Vec<String> {
    let readme = fs::read_to_string(folder().join("README.md")).expect("CHANGELOG/README.md");
    readme
        .split("](")
        .skip(1)
        .filter_map(|rest| rest.split_once(')'))
        .map(|(target, _)| target.to_string())
        .filter(|target| target.starts_with("v.") && target.ends_with(".md"))
        .collect()
}

#[test]
fn every_file_is_listed_and_every_listed_file_exists() {
    let mut on_disk: Vec<String> = fs::read_dir(folder())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("v.") && name.ends_with(".md"))
        .collect();
    let mut listed = listed_files();
    assert!(!listed.is_empty(), "CHANGELOG/README.md lists no version");

    on_disk.sort();
    let mut sorted_listed = listed.clone();
    sorted_listed.sort();
    assert_eq!(
        on_disk, sorted_listed,
        "CHANGELOG/README.md must link every v.*.md file once"
    );

    // newest first: the page shows the files in the order of the list
    for name in &listed {
        assert!(
            series(name).is_some(),
            "{name} is not called v.<major>.<minor>.x.md"
        );
    }
    let mut newest_first = listed.clone();
    newest_first.sort_by_key(|name| std::cmp::Reverse(series(name)));
    assert_eq!(listed, newest_first, "list the newest series first");
    listed.dedup();
    assert_eq!(listed.len(), on_disk.len());
}

#[test]
fn a_file_starts_with_a_version_of_its_series() {
    for name in listed_files() {
        let (major, minor) = series(&name).unwrap();
        let text = fs::read_to_string(folder().join(&name)).unwrap();
        let first = text.lines().next().unwrap();
        let heading = format!("## [{major}.{minor}.");
        assert!(
            first.starts_with(&heading),
            "{name} must start with a heading like `{heading}0] — date`, found `{first}`"
        );
        assert!(
            !text.lines().any(|line| line.starts_with("# ")),
            "{name}: the title of the page comes from README.md, use `##` for versions"
        );
    }
}

#[test]
fn the_web_page_reads_the_folder() {
    let page =
        fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/index.html"))
            .unwrap();
    assert!(page.contains("'CHANGELOG/'") && page.contains("README.md"));
}

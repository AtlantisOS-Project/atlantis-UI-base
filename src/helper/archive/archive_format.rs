// archive_format.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Recognizing and unpacking archives.
//!
//! The format of an archive is recognized by its first bytes (the "magic number"), not by
//! the ending of the file name, so a wrongly named download still works. Supported are
//! `tar`, `tar.xz`, `tar.gz`, `tar.bz2`, `xz`, `gz`, `bz2`, `zip` and `7z`.
//!
//! # Usage
//!
//! ```no_run
//! use atlbase::prelude::{decompress, decompress_file, detect_format};
//! use std::path::Path;
//!
//! let archive = Path::new("/home/me/Downloads/rom.tar.xz");
//! let target = Path::new("/home/me/work");
//!
//! // recognize and unpack in one step ...
//! let files = decompress_file(archive, target)?;
//! println!("{} files unpacked", files.len());
//!
//! // ... or ask the user when the format is unknown
//! if detect_format(archive)?.is_none() {
//!     // show a menu with `ArchiveFormat::ALL`, then:
//!     // decompress(archive, target, chosen_format)?;
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

/// A compression or archive format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    /// A tar archive.
    Tar,
    /// A tar archive compressed with xz.
    TarXz,
    /// A tar archive compressed with gzip.
    TarGz,
    /// A tar archive compressed with bzip2.
    TarBz2,
    /// One file compressed with xz.
    Xz,
    /// One file compressed with gzip.
    Gz,
    /// One file compressed with bzip2.
    Bz2,
    /// A zip archive.
    Zip,
    /// A 7-Zip archive.
    SevenZ,
}

impl ArchiveFormat {
    /// All formats, in the order they are offered.
    pub const ALL: [ArchiveFormat; 9] = [
        ArchiveFormat::Zip,
        ArchiveFormat::SevenZ,
        ArchiveFormat::Tar,
        ArchiveFormat::TarXz,
        ArchiveFormat::TarGz,
        ArchiveFormat::TarBz2,
        ArchiveFormat::Xz,
        ArchiveFormat::Gz,
        ArchiveFormat::Bz2,
    ];

    /// The usual ending of the format, without a dot (`tar.xz`).
    pub fn label(self) -> &'static str {
        match self {
            ArchiveFormat::Tar => "tar",
            ArchiveFormat::TarXz => "tar.xz",
            ArchiveFormat::TarGz => "tar.gz",
            ArchiveFormat::TarBz2 => "tar.bz2",
            ArchiveFormat::Xz => "xz",
            ArchiveFormat::Gz => "gz",
            ArchiveFormat::Bz2 => "bz2",
            ArchiveFormat::Zip => "zip",
            ArchiveFormat::SevenZ => "7z",
        }
    }
}

const XZ_MAGIC: &[u8] = &[0xFD, b'7', b'z', b'X', b'Z', 0];
const GZ_MAGIC: &[u8] = &[0x1F, 0x8B];
const BZ2_MAGIC: &[u8] = b"BZh";
const ZIP_MAGIC: &[u8] = b"PK";
const SEVENZ_MAGIC: &[u8] = &[b'7', b'z', 0xBC, 0xAF, 0x27, 0x1C];

/// The size of a tar header; `ustar` is at offset 257 in it.
const TAR_BLOCK: usize = 512;

fn is_tar_header(block: &[u8]) -> bool {
    block.len() >= 262 && &block[257..262] == b"ustar"
}

/// Whether the decompressed stream starts like a tar archive.
fn inner_is_tar(mut reader: impl Read) -> bool {
    let mut block = [0u8; TAR_BLOCK];
    reader.read_exact(&mut block).is_ok() && is_tar_header(&block)
}

/// Recognizes the format of a file.
///
/// The first bytes decide. A compressed file is opened a little to see whether a tar archive
/// is inside (`tar.gz`) or a single file (`gz`). Only a file with the ending `.tar` and no
/// recognizable content (an old tar without `ustar`) is taken from its name.
///
/// # Returns
///
/// `None` if the format is not known: the user has to say it.
///
/// # Errors
///
/// Returns the [`io::Error`] if the file cannot be read.
pub fn detect_format(path: &Path) -> io::Result<Option<ArchiveFormat>> {
    let mut head = Vec::new();
    File::open(path)?
        .take(TAR_BLOCK as u64)
        .read_to_end(&mut head)?;
    let open = || File::open(path).map(BufReader::new);

    let format = if head.starts_with(XZ_MAGIC) {
        Some(if inner_is_tar(xz2::read::XzDecoder::new(open()?)) {
            ArchiveFormat::TarXz
        } else {
            ArchiveFormat::Xz
        })
    } else if head.starts_with(GZ_MAGIC) {
        Some(if inner_is_tar(flate2::read::GzDecoder::new(open()?)) {
            ArchiveFormat::TarGz
        } else {
            ArchiveFormat::Gz
        })
    } else if head.starts_with(BZ2_MAGIC) {
        Some(if inner_is_tar(bzip2::read::BzDecoder::new(open()?)) {
            ArchiveFormat::TarBz2
        } else {
            ArchiveFormat::Bz2
        })
    } else if head.starts_with(SEVENZ_MAGIC) {
        Some(ArchiveFormat::SevenZ)
    } else if head.starts_with(ZIP_MAGIC) && head.len() >= 4 && matches!(head[2], 3 | 5 | 7) {
        Some(ArchiveFormat::Zip)
    } else if is_tar_header(&head) || has_extension(path, "tar") {
        Some(ArchiveFormat::Tar)
    } else {
        None
    };
    Ok(format)
}

fn has_extension(path: &Path, extension: &str) -> bool {
    path.extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case(extension))
}

/// The name of the file a single compressed file turns into: `system.img.xz` → `system.img`.
fn decompressed_name(path: &Path, format: ArchiveFormat) -> String {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    match name.strip_suffix(&format!(".{}", format.label())) {
        Some(stem) if !stem.is_empty() => stem.to_string(),
        _ => format!("{name}.out"),
    }
}

/// All entries below `dir` that are not folders. A symbolic link is listed as itself and never
/// followed: an archive can contain a link to any place on the computer.
fn files_below(dir: &Path, found: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            files_below(&entry.path(), found)?;
        } else {
            found.push(entry.path());
        }
    }
    Ok(())
}

/// Decompresses `path` into `output_dir`.
///
/// The archive is unpacked into a temporary folder inside `output_dir` first. When that
/// worked, its content replaces files with the same names in `output_dir`; a damaged
/// archive therefore leaves `output_dir` as it was.
///
/// # Returns
///
/// All files that were created, with their final paths.
///
/// # Errors
///
/// Returns an error if the file cannot be read or is damaged, or does not fit `format`.
pub fn decompress(
    path: &Path,
    output_dir: &Path,
    format: ArchiveFormat,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    fs::create_dir_all(output_dir)?;
    let temp = tempfile::Builder::new()
        .prefix(".extract-")
        .tempdir_in(output_dir)?;
    let out = temp.path();
    let reader = || -> io::Result<BufReader<File>> { Ok(BufReader::new(File::open(path)?)) };
    let single = |mut source: &mut dyn Read| -> io::Result<u64> {
        io::copy(
            &mut source,
            &mut File::create(out.join(decompressed_name(path, format)))?,
        )
    };

    match format {
        ArchiveFormat::Tar => tar::Archive::new(reader()?).unpack(out)?,
        ArchiveFormat::TarXz => {
            tar::Archive::new(xz2::read::XzDecoder::new(reader()?)).unpack(out)?
        }
        ArchiveFormat::TarGz => {
            tar::Archive::new(flate2::read::GzDecoder::new(reader()?)).unpack(out)?
        }
        ArchiveFormat::TarBz2 => {
            tar::Archive::new(bzip2::read::BzDecoder::new(reader()?)).unpack(out)?
        }
        ArchiveFormat::Xz => {
            single(&mut xz2::read::XzDecoder::new(reader()?))?;
        }
        ArchiveFormat::Gz => {
            single(&mut flate2::read::GzDecoder::new(reader()?))?;
        }
        ArchiveFormat::Bz2 => {
            single(&mut bzip2::read::BzDecoder::new(reader()?))?;
        }
        ArchiveFormat::Zip => zip::ZipArchive::new(reader()?)?.extract(out)?,
        ArchiveFormat::SevenZ => sevenz_rust2::decompress_file(path, out)?,
    }

    // move the result up: the top-level entries replace what has the same name
    let mut created = Vec::new();
    for entry in fs::read_dir(out)? {
        let from = entry?.path();
        let to = output_dir.join(from.file_name().ok_or("an entry without a name")?);
        if to.is_dir() {
            fs::remove_dir_all(&to)?;
        } else if to.exists() {
            fs::remove_file(&to)?;
        }
        fs::rename(&from, &to)?;
        if to.symlink_metadata()?.is_dir() {
            files_below(&to, &mut created)?;
        } else {
            created.push(to);
        }
    }
    created.sort();
    Ok(created)
}

/// Recognizes the format and decompresses; a convenience for [`detect_format`] and
/// [`decompress`].
///
/// # Errors
///
/// Returns an error if the format is not known (use [`decompress`] with the format the user
/// chose then), or if the file cannot be read or is damaged.
pub fn decompress_file(
    path: &Path,
    output_dir: &Path,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let format = detect_format(path)?
        .ok_or_else(|| format!("Unknown or unsupported format: {}", path.display()))?;
    decompress(path, output_dir, format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_with(path: &Path, bytes: &[u8]) {
        fs::write(path, bytes).unwrap();
    }

    /// A tar archive with one file `boot.img`.
    fn tar_bytes() -> Vec<u8> {
        let mut builder = tar::Builder::new(Vec::new());
        let data = b"boot data";
        let mut header = tar::Header::new_gnu();
        header.set_size(data.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        builder
            .append_data(&mut header, "boot.img", &data[..])
            .unwrap();
        builder.into_inner().unwrap()
    }

    fn gz(bytes: &[u8]) -> Vec<u8> {
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        encoder.write_all(bytes).unwrap();
        encoder.finish().unwrap()
    }

    fn xz(bytes: &[u8]) -> Vec<u8> {
        let mut encoder = xz2::write::XzEncoder::new(Vec::new(), 1);
        encoder.write_all(bytes).unwrap();
        encoder.finish().unwrap()
    }

    fn bz2(bytes: &[u8]) -> Vec<u8> {
        let mut encoder = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::fast());
        encoder.write_all(bytes).unwrap();
        encoder.finish().unwrap()
    }

    fn zip_bytes() -> Vec<u8> {
        let mut writer = zip::ZipWriter::new(io::Cursor::new(Vec::new()));
        writer
            .start_file("boot.img", zip::write::SimpleFileOptions::default())
            .unwrap();
        writer.write_all(b"boot data").unwrap();
        writer.finish().unwrap().into_inner()
    }

    #[test]
    fn the_format_is_recognized_by_the_content_not_the_name() {
        let root = tempfile::tempdir().unwrap();
        let cases: Vec<(&str, Vec<u8>, Option<ArchiveFormat>)> = vec![
            ("a.zip", zip_bytes(), Some(ArchiveFormat::Zip)),
            ("no-ending", zip_bytes(), Some(ArchiveFormat::Zip)),
            ("a.tar", tar_bytes(), Some(ArchiveFormat::Tar)),
            ("a.dat", tar_bytes(), Some(ArchiveFormat::Tar)),
            ("a.tar.gz", gz(&tar_bytes()), Some(ArchiveFormat::TarGz)),
            ("a.tar.xz", xz(&tar_bytes()), Some(ArchiveFormat::TarXz)),
            ("a.tar.bz2", bz2(&tar_bytes()), Some(ArchiveFormat::TarBz2)),
            ("system.img.gz", gz(b"system"), Some(ArchiveFormat::Gz)),
            ("system.img.xz", xz(b"system"), Some(ArchiveFormat::Xz)),
            ("system.img.bz2", bz2(b"system"), Some(ArchiveFormat::Bz2)),
            ("wrong-name.zip", gz(b"system"), Some(ArchiveFormat::Gz)),
            ("notes.txt", b"just some text".to_vec(), None),
            ("empty.bin", Vec::new(), None),
        ];
        for (name, bytes, expected) in cases {
            let path = root.path().join(name);
            write_with(&path, &bytes);
            assert_eq!(detect_format(&path).unwrap(), expected, "{name}");
        }
        assert!(detect_format(&root.path().join("missing")).is_err());
    }

    #[test]
    fn an_old_tar_without_ustar_is_taken_from_its_name() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("old.tar");
        write_with(&path, &[0u8; 1024]);
        assert_eq!(detect_format(&path).unwrap(), Some(ArchiveFormat::Tar));
    }

    #[test]
    fn every_format_can_be_unpacked() {
        let root = tempfile::tempdir().unwrap();
        let cases: Vec<(&str, Vec<u8>, &str)> = vec![
            ("a.zip", zip_bytes(), "boot.img"),
            ("a.tar", tar_bytes(), "boot.img"),
            ("a.tar.gz", gz(&tar_bytes()), "boot.img"),
            ("a.tar.xz", xz(&tar_bytes()), "boot.img"),
            ("a.tar.bz2", bz2(&tar_bytes()), "boot.img"),
            ("system.img.gz", gz(b"boot data"), "system.img"),
            ("system.img.xz", xz(b"boot data"), "system.img"),
            ("system.img.bz2", bz2(b"boot data"), "system.img"),
        ];
        for (name, bytes, result) in cases {
            let input = root.path().join(name);
            write_with(&input, &bytes);
            let out = root.path().join(format!("out-{name}"));

            let files = decompress_file(&input, &out).unwrap();
            assert_eq!(files, [out.join(result)], "{name}");
            assert_eq!(fs::read(out.join(result)).unwrap(), b"boot data", "{name}");
            // the temporary folder is gone
            assert_eq!(fs::read_dir(&out).unwrap().count(), 1, "{name}");
        }
    }

    #[test]
    fn a_7z_archive_is_unpacked() {
        let root = tempfile::tempdir().unwrap();
        let source = root.path().join("source");
        fs::create_dir(&source).unwrap();
        fs::write(source.join("boot.img"), b"boot data").unwrap();
        let archive = root.path().join("a.7z");
        sevenz_rust2::compress_to_path(&source, &archive).unwrap();
        assert_eq!(
            detect_format(&archive).unwrap(),
            Some(ArchiveFormat::SevenZ)
        );

        let out = root.path().join("out");
        let files = decompress_file(&archive, &out).unwrap();
        assert_eq!(fs::read(&files[0]).unwrap(), b"boot data");
    }

    /// A 7z archive with one file under the name `name`, written without any check of the name.
    fn seven_z_with_entry(path: &Path, name: &str) {
        let mut writer = sevenz_rust2::ArchiveWriter::create(path).unwrap();
        writer
            .push_archive_entry(
                sevenz_rust2::ArchiveEntry::new_file(name),
                Some(&b"written outside"[..]),
            )
            .unwrap();
        writer.finish().unwrap();
    }

    #[test]
    fn a_7z_entry_cannot_leave_the_output_folder() {
        let root = tempfile::tempdir().unwrap();
        let out = root.path().join("work/out");
        fs::create_dir_all(&out).unwrap();
        let outside = root.path().join("outside.txt");
        let beside = root.path().join("work/beside.txt");

        // `..` (also written as a Windows path) and an absolute path
        for (index, name) in [
            "../../outside.txt".to_string(),
            "..\\..\\outside.txt".to_string(),
            "../beside.txt".to_string(),
            outside.to_string_lossy().into_owned(),
        ]
        .iter()
        .enumerate()
        {
            let archive = root.path().join(format!("evil{index}.7z"));
            seven_z_with_entry(&archive, name);
            assert!(decompress_file(&archive, &out).is_err(), "{name}");
        }

        assert!(!outside.exists() && !beside.exists());
        assert_eq!(
            fs::read_dir(&out).unwrap().count(),
            0,
            "nothing is left behind"
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_zip_does_not_keep_setuid_bits() {
        use std::os::unix::fs::PermissionsExt;

        let root = tempfile::tempdir().unwrap();
        let mut zip = zip_bytes();
        // the writer masks the mode: set `-rwsr-xr-x` in the central directory by hand (the
        // 4 bytes of the external attributes start 38 bytes after the signature `PK\x01\x02`)
        let header = zip.windows(4).position(|w| w == b"PK\x01\x02").unwrap();
        zip[header + 38..header + 42].copy_from_slice(&(0o104755u32 << 16).to_le_bytes());
        let archive = root.path().join("suid.zip");
        write_with(&archive, &zip);

        let files = decompress_file(&archive, &root.path().join("out")).unwrap();
        let mode = fs::metadata(&files[0]).unwrap().permissions().mode();
        assert_eq!(mode & 0o7000, 0, "mode is {mode:o}");
    }

    #[cfg(unix)]
    #[test]
    fn a_zip_cannot_write_through_a_symbolic_link_to_another_folder() {
        let root = tempfile::tempdir().unwrap();
        let outside = root.path().join("outside");
        fs::create_dir(&outside).unwrap();

        let options = zip::write::SimpleFileOptions::default();
        let mut writer = zip::ZipWriter::new(io::Cursor::new(Vec::new()));
        writer
            .add_symlink("link", outside.to_str().unwrap(), options)
            .unwrap();
        writer.start_file("link/pwned.txt", options).unwrap();
        writer.write_all(b"written outside").unwrap();
        let archive = root.path().join("link.zip");
        write_with(&archive, &writer.finish().unwrap().into_inner());

        // whether the archive is refused or only the link is skipped does not matter
        let _ = decompress_file(&archive, &root.path().join("out"));
        assert_eq!(fs::read_dir(&outside).unwrap().count(), 0);
    }

    #[cfg(unix)]
    #[test]
    fn a_symbolic_link_in_an_archive_is_listed_but_never_followed() {
        let root = tempfile::tempdir().unwrap();
        let secret = root.path().join("secret");
        fs::create_dir_all(secret.join("deeper")).unwrap();
        fs::write(secret.join("private.txt"), b"x").unwrap();
        fs::write(secret.join("deeper/more.txt"), b"x").unwrap();

        let link = |builder: &mut tar::Builder<Vec<u8>>, name: &str| {
            let mut header = tar::Header::new_gnu();
            header.set_entry_type(tar::EntryType::Symlink);
            header.set_size(0);
            builder.append_link(&mut header, name, &secret).unwrap();
        };
        let mut builder = tar::Builder::new(Vec::new());
        let mut dir = tar::Header::new_gnu();
        dir.set_entry_type(tar::EntryType::Directory);
        dir.set_mode(0o755);
        dir.set_size(0);
        builder.append_data(&mut dir, "d/", &b""[..]).unwrap();
        link(&mut builder, "d/link");
        link(&mut builder, "top");
        let archive = root.path().join("links.tar");
        write_with(&archive, &builder.into_inner().unwrap());

        let out = root.path().join("out");
        let files = decompress_file(&archive, &out).unwrap();
        assert_eq!(files, [out.join("d/link"), out.join("top")]);
        assert!(files.iter().all(|file| file.starts_with(&out)));
    }

    #[test]
    fn a_wrong_format_or_a_damaged_file_changes_nothing() {
        let root = tempfile::tempdir().unwrap();
        let out = root.path().join("out");
        fs::create_dir(&out).unwrap();
        fs::write(out.join("keep.img"), b"keep").unwrap();

        // an unknown format is an error
        let text = root.path().join("notes.txt");
        write_with(&text, b"text");
        let error = decompress_file(&text, &out).unwrap_err().to_string();
        assert!(error.contains("Unknown or unsupported format"), "{error}");

        // text that is claimed to be a zip is damaged
        assert!(decompress(&text, &out, ArchiveFormat::Zip).is_err());
        assert!(decompress(&text, &out, ArchiveFormat::TarGz).is_err());

        let mut names: Vec<_> = fs::read_dir(&out)
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        names.sort();
        assert_eq!(names, ["keep.img"], "no temporary folder is left behind");
    }

    #[test]
    fn existing_files_are_replaced() {
        let root = tempfile::tempdir().unwrap();
        let out = root.path().join("out");
        fs::create_dir(&out).unwrap();
        fs::write(out.join("boot.img"), b"old").unwrap();
        let input = root.path().join("a.tar");
        write_with(&input, &tar_bytes());

        decompress_file(&input, &out).unwrap();
        assert_eq!(fs::read(out.join("boot.img")).unwrap(), b"boot data");
    }

    #[test]
    fn the_names_of_the_formats() {
        let labels: Vec<&str> = ArchiveFormat::ALL.iter().map(|f| f.label()).collect();
        assert_eq!(
            labels,
            [
                "zip", "7z", "tar", "tar.xz", "tar.gz", "tar.bz2", "xz", "gz", "bz2"
            ]
        );
        assert_eq!(
            decompressed_name(Path::new("/a/system.img.xz"), ArchiveFormat::Xz),
            "system.img"
        );
        assert_eq!(
            decompressed_name(Path::new("/a/image"), ArchiveFormat::Gz),
            "image.out"
        );
    }
}

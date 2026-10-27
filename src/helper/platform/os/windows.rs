// windows.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Windows. Run only in tests so far: the mappings are text, covered by the unit tests below.

use super::{Has, OsProfile, words};
use crate::helper::platform::package::PackageManager;
use crate::helper::platform::target::Os;

fn quote(word: &str) -> String {
    // inside a single quoted PowerShell string only the quote itself has a meaning
    format!("'{}'", word.replace('\'', "''"))
}

/// One argument the way the C runtime and `CommandLineToArgvW` read it back: in double quotes if
/// it has a space, a tab or a quote, and the quotes and the backslashes in front of them escaped.
/// `Start-Process -ArgumentList` does no quoting of its own: it joins its list with spaces, so an
/// argument with a space would become two.
fn quote_argument(arg: &str) -> String {
    if !arg.is_empty() && !arg.contains([' ', '\t', '\n', '\u{b}', '"']) {
        return arg.to_string();
    }
    let mut quoted = String::from('"');
    let mut backslashes = 0;
    for c in arg.chars() {
        match c {
            '\\' => backslashes += 1,
            '"' => {
                // the backslashes before a quote are doubled, and the quote gets one more
                quoted.push_str(&"\\".repeat(backslashes * 2 + 1));
                quoted.push('"');
                backslashes = 0;
            }
            c => {
                quoted.push_str(&"\\".repeat(backslashes));
                quoted.push(c);
                backslashes = 0;
            }
        }
    }
    // the backslashes before the closing quote are doubled, so they do not escape it
    quoted.push_str(&"\\".repeat(backslashes * 2));
    quoted.push('"');
    quoted
}

/// `powershell Start-Process <program> -Verb RunAs -Wait`: the UAC prompt.
fn elevate(command: &[String]) -> Vec<String> {
    let (program, args) = command.split_first().expect("a command");
    let arguments = if args.is_empty() {
        String::new()
    } else {
        // one string with the whole command line of the program
        let line: Vec<String> = args.iter().map(|arg| quote_argument(arg)).collect();
        format!(" -ArgumentList {}", quote(&line.join(" ")))
    };
    words(&[
        "powershell",
        "-NoProfile",
        "-Command",
        &format!(
            "$p = Start-Process -FilePath {}{} -Verb RunAs -Wait -PassThru; exit $p.ExitCode",
            quote(program),
            arguments
        ),
    ])
}

fn terminal(_has: Has, _desktop: &str, command: &str) -> Option<Vec<String>> {
    Some(words(&[
        "cmd",
        "/C",
        &format!("start cmd.exe /K \"{command}\""),
    ]))
}

/// The ending of executable files, from `PATHEXT` (`.COM;.EXE;.BAT;.CMD`).
fn extensions(pathext: Option<&str>) -> Vec<String> {
    pathext
        .unwrap_or(".COM;.EXE;.BAT;.CMD")
        .split(';')
        .filter(|ext| ext.starts_with('.'))
        .map(str::to_ascii_lowercase)
        .collect()
}

/// The name itself, or with every executable ending unless it has one.
fn program_names(name: &str, pathext: Option<&str>) -> Vec<String> {
    let extensions = extensions(pathext);
    let lower = name.to_ascii_lowercase();
    if extensions.iter().any(|ext| lower.ends_with(ext.as_str())) {
        return vec![name.to_string()];
    }
    extensions
        .iter()
        .map(|ext| format!("{name}{ext}"))
        .collect()
}

/// Windows.
pub static PROFILE: OsProfile = OsProfile {
    os: Os::Windows,
    exe_suffix: ".exe",
    posix_shell: false,
    default_share_dirs: &[],
    critical_paths: &[
        "C:\\Windows",
        "C:\\Program Files",
        "C:\\Program Files (x86)",
        "C:\\ProgramData",
    ],
    protected_dirs: &["C:\\Users"],
    standard_home_folders: &[
        "Desktop",
        "Documents",
        "Downloads",
        "Music",
        "Pictures",
        "Videos",
        "AppData",
    ],
    elevation_program: Some("powershell"),
    elevate,
    terminal,
    terminal_available: |_| true,
    package_managers: &[PackageManager::Winget],
    journal_follow: None,
    syslog: false,
    program_names,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn programs_get_their_ending() {
        assert_eq!(
            program_names("adb", None),
            ["adb.com", "adb.exe", "adb.bat", "adb.cmd"]
        );
        assert_eq!(
            program_names("adb", Some(".EXE;.CMD")),
            ["adb.exe", "adb.cmd"]
        );
        assert_eq!(program_names("ADB.EXE", None), ["ADB.EXE"]);
    }

    #[test]
    fn quotes_for_powershell() {
        let command = words(&["tool.exe", "it's", "b"]);
        let line = elevate(&command);
        assert_eq!(line[..3], ["powershell", "-NoProfile", "-Command"]);
        assert_eq!(
            line[3],
            "$p = Start-Process -FilePath 'tool.exe' -ArgumentList 'it''s b' -Verb RunAs -Wait -PassThru; exit $p.ExitCode"
        );
        assert!(!elevate(&words(&["tool.exe"]))[3].contains("-ArgumentList"));
    }

    #[test]
    fn an_argument_stays_one_argument() {
        // plain: untouched
        assert_eq!(quote_argument("flash"), "flash");
        assert_eq!(quote_argument(r"C:\dir\file.img"), r"C:\dir\file.img");
        // a space, a quote, nothing at all
        assert_eq!(quote_argument("a b"), "\"a b\"");
        assert_eq!(quote_argument(r#"say "hi""#), r#""say \"hi\"""#);
        assert_eq!(quote_argument(""), "\"\"");
        // backslashes only count in front of a quote
        assert_eq!(quote_argument(r"C:\my dir\"), r#""C:\my dir\\""#);
        assert_eq!(quote_argument(r#"a\"b"#), r#""a\\\"b""#);

        let line = elevate(&words(&["tool.exe", "C:\\my dir\\x.img", "--note=a \"b\""]));
        assert_eq!(
            line[3],
            "$p = Start-Process -FilePath 'tool.exe' -ArgumentList '\"C:\\my dir\\x.img\" \"--note=a \\\"b\\\"\"' -Verb RunAs -Wait -PassThru; exit $p.ExitCode"
        );
    }

    #[test]
    fn has_a_terminal_but_no_shell_and_no_journal() {
        assert_eq!(
            terminal(&|_: &str| false, "", "dir").unwrap(),
            ["cmd", "/C", "start cmd.exe /K \"dir\""]
        );
        assert!(PROFILE.journal_follow.is_none() && !PROFILE.posix_shell);
    }
}

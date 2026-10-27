# Security audits of atlbase

This file is the log of the security audits and code reviews of `atlbase`. It has four parts:

1. [**Scale**](#1-the-scale): how a finding is rated (the ATL scale), and the other words used here.
2. [**Latest result**](#2-latest-result) and the [**audit history**](#3-audit-history).
3. [**Open findings**](#4-open-findings): what is *not* fixed, and why.
4. [**Fixed findings**](#5-fixed-findings): what was found, what was done, and which test keeps it fixed.

Nothing is ever deleted from this file. A finding that is fixed moves from "Open" to "Fixed"; a
new audit adds a row to the history and new findings with the next free number. How to run an
audit and how to update this file is at the [end](#6-how-to-run-an-audit-and-update-this-file).

Findings were first reported in chat messages with informal labels (High, Medium, Low). The levels
in this file follow the scale below and are the ones that count; a few differ from the first
labels. The column "Ref" gives the label of the first report (`H1`, `M6`, `N3`, `L-2`, …) so a
finding can be traced back.

## 1. The scale

### 1.1 Level: two questions

A finding gets its **level** from two questions. *Who can trigger it?* gives the **reach** (A1 to
A4), *what does it cost?* gives the **damage** (B1 to B3). The table gives the level.

**Reach: who can trigger it?**

| Reach | Meaning | Examples |
|---|---|---|
| **A4** | From the network, without anybody doing anything | a service that answers a crafted packet |
| **A3** | By data that reaches the program in normal use: a file the user opens, an archive, a download, the output of a command or a device | a crafted `.7z`, a tar with a link, the stderr of a failed `fastboot` |
| **A2** | Needs a mistake of the calling program (untrusted text passed on), a special setup (a debug option, a privileged caller, an unusual `umask`), or a second user on the same computer | `open_terminal_by_desktop(text from a file)`, `-debug -snp` and another user in `/tmp` |
| **A1** | Needs another vulnerability first, or only affects the correctness of the program itself | a data race that needs a second thread to change the environment |

**Damage: what does it cost?**

| Damage | Meaning | Examples |
|---|---|---|
| **B3** | Severe: code runs, files are written or deleted that the user did not expect, rights grow, or a secret leaks | write outside the output folder, delete an existing `Pictures/`, a password in a log that is sent around |
| **B2** | Moderate: the program's own data is lost or shown, the program is blocked, or the user is shown something false | a full disk, a dialog that cannot be closed, a log line that hides another one |
| **B1** | Minor: a wrong result without lasting damage, a missing hardening step, or text that promises too much | documentation that claims a protection that is not there |

**Level**

| Reach ↓ / Damage → | **B1** minor | **B2** moderate | **B3** severe |
|---|---|---|---|
| **A4** network, no action | Medium | High | **Critical** |
| **A3** data in normal use | Low | Medium | **High** |
| **A2** mistake, setup or local user | Info | Low | Medium |
| **A1** needs another bug | Info | Info | Low |

### 1.2 What the levels ask of us

| Level | Meaning | What happens |
|---|---|---|
| **Critical** | Can be used over the network by anybody, and does real harm | Stop, fix first, release a fixed version at once, tell the users |
| **High** | A crafted file or download does real harm in normal use | Fix before the next release; no release with an open High finding |
| **Medium** | Harm needs a mistake of the caller, a special setup or a local attacker | Fix in the current release cycle, or write down why not |
| **Low** | Small harm, or hard to reach | Fix when the code is touched anyway; may stay open with a reason |
| **Info** | Hardening, wrong text, robustness, no direct harm | Fix when cheap; may stay open |

No finding of this library has been **Critical** so far: `atlbase` is a library for desktop
programs and listens on no port.

### 1.3 Kind of finding

| Kind | Meaning |
|---|---|
| **S** | Security: an attacker gains something |
| **R** | Robustness: data loss, hang, full disk, crash, memory, without an attacker having to be clever |
| **B** | Bug or documentation that promises too much |
| **P** | Process: no check in place that would find the next one |

### 1.4 Evidence: how sure are we?

| Mark | Meaning |
|---|---|
| **V** | Verified: reproduced with a proof of concept against the unchanged code |
| **T** | A test in the repository fails with the old code (or shows the behavior) and passes now |
| **C** | Code review: the code does it, no run |
| **R** | Reported by a tool (`cargo audit`, `clippy`, CodeQL) |
| **U** | Unverified: could not be run here (for example the code for Windows and macOS) |

### 1.5 Status

| Status | Meaning |
|---|---|
| **Open** | Not fixed. The reason is in the table. |
| **Accepted** | Not fixed on purpose, with a reason, until the reason ceases to hold. |
| **Fixed** | Fixed in the commit named. "pending" means: in the working tree, not committed yet; put the hash in when it is. |

## 2. Latest result

Audit 5 of 2026-10-27, after the fixes of the same day (the working tree, see
[history](#3-audit-history)).

| Check | Result |
|---|---|
| `cargo audit` (RustSec database of 2026-10-27, 1380 advisories, 260 crates) | **0 vulnerabilities**, 2 warnings (`anyhow` RUSTSEC-2026-0190, `tokio` RUSTSEC-2026-0386; both in [Open](#4-open-findings)) |
| CodeQL 2.26.1, suite `security-extended`, all Rust files and the GitHub workflow | **0 findings** (`rust`: 27 rules, 18 of them for security; `actions`) |
| `rustfmt --check`, `clippy --all-targets`, `RUSTDOCFLAGS="-D warnings" cargo doc` | clean |
| Tests with all features on Broadway | 105 unit tests, 161 doc tests, 24 GTK checks and all integration tests: all pass |
| `scripts/check-platform.sh` (Linux and `x86_64-pc-windows-gnu`) | compiles |
| Findings | **36 fixed**, **9 open** (5 Low, 4 Info), none Critical, none High, none Medium |

| Level | Fixed | Open |
|---|---|---|
| Critical | 0 | 0 |
| High | 2 | 0 |
| Medium | 16 | 0 |
| Low | 9 | 5 |
| Info | 9 | 4 |
| **Total** | **36** | **9** |

CodeQL finds data flows from a source to a sink. Most findings here are logic and design errors
(a wrong timeout, a folder that is replaced), which it cannot find: a clean CodeQL result does not
replace a review.

## 3. Audit history

| # | Date | Scope | Tools and method | New findings |
|---|---|---|---|---|
| 1 | 2026-10-22 | The whole crate, `build.rs`, scripts, history of the repository | Read through the code, proofs of concept (7z, tar link, decompression bomb, temporary file, `Path::starts_with`), search for secrets in the history | 20 |
| 2 | 2026-10-22 | Dependencies | `cargo audit` (the database was available now), reading the sources of `zip`, `tar`, `sevenz-rust`, proofs of concept for `zip` | 6 advisories (two more were known already as H1 and L7) |
| 3 | 2026-10-23 | Complete code review and security audit | `clippy` with `unwrap_used`, `expect_used`, `indexing_slicing`, casts and `arithmetic_side_effects`; licenses; proofs of concept (folder replaced, `set_config`, download timeout) | 10 |
| 4 | 2026-10-27 | Re-scan and the new logging code (working tree) | `cargo audit`, CodeQL, proof of concept (control characters and mode of the log file), 329 447 time stamps against Python | 6 |
| 5 | 2026-10-27 | Verification of all fixes | `cargo audit`, CodeQL, `clippy`, `rustdoc`, all tests, a mutation test for three fixes | 0 |

Fix rounds: **2026-10-22** (commit `60951c0`: dependencies, 7z, `files_below`, temporary file),
**2026-10-23** (commit `a03b091`, by the maintainer: `delete_directory`, `get_home_directory`,
`add_line_file` and the two search functions), **2026-10-27** (pending: everything else).

## 4. Open findings

Nothing that is High or Medium is open. What is left:

| ID | Level | Kind | Finding | Reach · Damage | Evidence | Status and reason |
|---|---|---|---|---|---|---|
| ATL-SEC-037 | Info | R | `anyhow` 1.0.102 is unsound in `Error::downcast_mut` (RUSTSEC-2026-0190) | A1 · B1 | R | **Accepted.** Only the WASM tooling (`wit-bindgen`) uses it, not the build of the library. Goes with the next lock update. |
| ATL-SEC-038 | Info | R | `tokio` 1.52.3: `AsyncFd::new` violates I/O safety (RUSTSEC-2026-0386) | A1 · B1 | R | **Accepted.** Only the feature `experimental-api` (module `system`, does not compile yet) pulls it in. Look again when that module is finished. |
| ATL-SEC-039 | Low | R | `init_language`, `set_renderer` and `apply_debug_options` change the environment; that is sound only if no other thread exists. The rule is in the documentation, not enforced. | A1 · B3 | C | **Open.** Cannot be enforced with the API of the process. `setlocale` of `gettext-rs` 0.8 is `unsafe` now, so the rule is at least visible in the code. |
| ATL-SEC-040 | Low | R | `run_command` and `capture_command_output` block with no limit of time or size | A2 · B2 | C | **Accepted.** They are blocking calls for a thread of the caller; the progress dialogs and `command_output_timeout` have the limits. Documented. |
| ATL-SEC-041 | Low | R | A command started through `pkexec` runs as `root` and cannot be killed by the user. After the time limit of a chain (2 h) the chain fails, but the process goes on. | A2 · B2 | C | **Accepted.** The system does not allow more. The result says that the command was stopped (it was given up on). |
| ATL-SEC-042 | Info | R | After the 30 s of silence of `download_file` the thread of the transfer may stay blocked in the read until the connection ends | A2 · B1 | C | **Accepted.** A read from a socket cannot be interrupted. The caller is free, the file is deleted, the thread ends with the connection. |
| ATL-SEC-043 | Low | R | `set_config`: two programs that save at the same moment can overwrite one key (read, change, write) | A2 · B2 | C | **Accepted.** The file is never half written (atomic), only a key can be lost. Documented. A lock file would need a `libc` dependency. |
| ATL-SEC-044 | Info | R | The rotation of the log file is not coordinated between several running instances | A2 · B1 | C | **Accepted.** At worst one instance keeps writing to the `.old` file for a while. |
| ATL-SEC-045 | Low | S | `add_line_file` opens `path` itself with `append`, so it follows a symbolic link *at `path`* | A2 · B3 | C | **Accepted.** The caller names the file; it is meant to edit it. Dangerous only for a path in a folder that others can write to. Documented; the backup file is safe (ATL-SEC-026). |

### Not covered (gaps, not findings)

* **Windows and macOS.** The code for the elevation (`Start-Process`, `osascript`), the terminal and
  the quoting is only compiled and tested as text (evidence **U**). It has never run on those systems.
* **Module `system`** (`experimental-api`): does not compile, left out on purpose (decision of the
  maintainer). Look again when it is finished: it is about Wi-Fi passwords.
* **Dependencies inside:** the HTTP and TLS stack (`ureq`, `rustls`), the decoders (`xz2`, `bzip2`,
  `flate2`) and the reading of `.mo` files by gettext were not reviewed; only their advisories are
  checked.
* **The new workflows** `audit.yml` and `dependabot.yml` have not run on GitHub yet.
* **Programs that use the library** (the Fastboot-Assistant, …) were not tested against the changed
  behavior of the fixes of 2026-10-27 (merging of folders, limits, `https` only, …). It is in the
  [changelog](../CHANGELOG/v.0.2.x.md).

## 5. Fixed findings

Order: by level, then by number. "Found" is the number of the audit in the
[history](#3-audit-history). Every fix has a test in the repository. Four were checked against the
old code and fail with it: ATL-SEC-004, ATL-SEC-005, ATL-SEC-031, and ATL-SEC-024 (the first run of
its test failed on the mode of the folder). The others were written together with the fix.

### Level High

| ID | Ref | Kind | Finding | Found | Fixed | How it was fixed · test |
|---|---|---|---|---|---|---|
| ATL-SEC-001 | H1 | S | **7z archive writes anywhere.** `sevenz-rust` joined the name of an entry to the folder without a check; `../../x` or `/home/me/.bashrc` left the output folder (RUSTSEC-2026-0245, no fix upstream). A3 · B3. Evidence **V**. | 1, 2 | `60951c0` | Replaced by `sevenz-rust2`, which refuses such names. Test `a_7z_entry_cannot_leave_the_output_folder`: `..`, a Windows path, an absolute path. |
| ATL-SEC-010 | N2 | R | **An archive deletes an existing folder.** `decompress` removed a folder of the same name before it moved the new one in: `Pictures/` in the archive deleted the `Pictures/` of the user. A3 · B3. Evidence **V**. | 3 | pending | Folders are merged, a file replaces a file, a file and a folder never replace each other (error, nothing deleted). Tests `a_folder_of_the_archive_is_merged_into_an_existing_folder`, `a_file_and_a_folder_never_replace_each_other`. |

### Level Medium

| ID | Ref | Kind | Finding | Found | Fixed | How it was fixed · test |
|---|---|---|---|---|---|---|
| ATL-SEC-002 | H2 | S | `zip` 1.3.1 kept the setuid and setgid bits of an entry (RUSTSEC-2026-0367). A2 (matters when a program with root rights unpacks) · B3. **V** | 2 | `60951c0`, pending | `zip` 9; since 2026-10-27 the library unpacks zips itself and cuts the mode to `rwxrwxrwx`. Test `a_zip_does_not_keep_setuid_bits`. |
| ATL-SEC-004 | M1 | S | `files_below` followed symbolic links of an archive: a link to `/` walked the whole file system and listed files outside the output folder. A3 · B2. **V** | 1 | `60951c0` | `file_type()` (does not follow), `symlink_metadata`. Test `a_symbolic_link_in_an_archive_is_listed_but_never_followed` (fails with the old code). |
| ATL-SEC-009 | M2 | R | No limit on the unpacked data: 65 KB of gzip became 64 MiB, and so on to a full disk. A3 · B2. **V** | 1 | pending | `decompress` stops at `MAX_UNPACKED_BYTES` (64 GiB), `decompress_limited` takes another limit; counted over all files in every format. Test `unpacking_stops_at_the_limit_in_every_format`. |
| ATL-SEC-011 | M3 | S | `download_file` took plain `http://` (and `HTTPS://` was not seen as https), and had no way to check the file that a program then runs. A2 · B3. **C** | 1 | pending | Only `https://` (in any case); `http://` only to `localhost`, `127.0.0.1`, `[::1]` (tricks with `user@host` and backslashes are refused). New `download_file_verified` with SHA-256. Tests `only_https_and_the_own_computer_are_allowed`, `a_wrong_checksum_deletes_the_file`. |
| ATL-SEC-013 | M2, M3 | S | No size limit of a download, and `.part` was created through a symbolic link that was there. A3 · B2. **T** | 1 | pending | At most 16 GiB (header and body); `.part` is made with `create_new` after the old file was removed. Tests `a_download_over_the_limit_is_refused`, `a_link_in_the_place_of_the_temporary_file_is_not_followed`. |
| ATL-SEC-015 | L-1 | S | Control characters in log messages (`ESC`, `CR`, the 8 bit `CSI`, direction marks) reached the log file, which the log viewer shows in a terminal: lines could be cleared, hidden or the title set. Output of a failed command or of a device gets there. A3 · B2. **V** | 4 | pending | `clean_message` writes them as `\x1b`, `\x9b`, `\u{202e}`; also for syslog. Tests `nothing_a_terminal_would_act_on_reaches_the_log`, `messages_are_cleaned_and_excerpts_are_short`. |
| ATL-SEC-016 | L-2 | S | The log file and its folder were created with the rights of the `umask` (`0664` / `0775` here). A2 · B3 (the log holds command lines). **V** | 4 | pending | `0600` and `0700`; a log from before is closed too. Test `the_log_is_private`. |
| ATL-SEC-017 | L-3 | S | The log got the complete command line (a Wi-Fi password in an argument), the text of every dialog and the output of a task. Bug reports pass logs on. A2 · B3. **C** | 1, 4 | pending | Values after `password`, `psk`, `token`, `secret`, … become `***` (best effort); dialog and task texts are cut to 5 lines and 500 bytes (`excerpt`). Test `secrets_do_not_reach_the_log`. |
| ATL-SEC-018 | L2 | R | `run_command_chain` (behind all progress dialogs) collected output without a limit and waited for ever; a dialog cannot be closed, so a command that waits for a device blocked the window. A3 · B2. **C** | 1 | pending | 8 MiB per stream, 2 hours per command, a note in `stderr` says so. Test `a_chain_keeps_at_most_8_mib_of_the_output_of_a_command` and the unit tests of `run_captured`. |
| ATL-SEC-020 | M4 | S | `open_terminal_by_desktop(&str)` hands the text to `bash -c` (and on WSL to `sh`, `cmd.exe` and `wsl.exe`); the quoting of `shell_line` does not hold for `cmd.exe`. A2 · B3. **C** | 1 | pending | New `open_terminal_command(&[String])`: quoting for POSIX shells, on Windows and WSL only plain words. `shell_line` and `open_terminal_by_desktop` say what they are for. Tests `every_word_is_quoted_for_a_posix_shell`, `windows_takes_plain_words_only`. |
| ATL-SEC-021 | L8 | S | Elevation on Windows: `Start-Process -ArgumentList 'a','b'` joins with spaces, so an argument with a space became two and could bring options into the elevated program. A2 · B3. **U** (no Windows here) | 1 | pending | One command line string, each argument quoted as the C runtime reads it. Test `an_argument_stays_one_argument` (the text only). |
| ATL-SEC-022 | M7 | S | The deletion guards compared the path as written: `/tmp/../etc` and a link to `/etc` passed; on Windows `C:\WINDOWS` was not `C:\Windows`. A2 · B3. **V** | 1 | pending | The path is resolved (`..`, links) first; case does not matter on Windows; a standard folder that is a link is still recognized. Tests in `delete_files.rs`, `helper_home_protection.rs`. |
| ATL-SEC-023 | M7 | S | `clear_directory_except` and `remove_dir_all_if_exists` had no protection of system folders or home. A2 · B3. **C** | 1 | pending | They refuse protected folders (`PermissionDenied`) before they touch anything. Test `home_and_standard_folders_are_protected`. |
| ATL-SEC-024 | M5 | S | `-debug -snp` set `$SNAP` to `/tmp/snap-debug`, and programs below `$SNAP` are searched before `PATH`: another user could place a fake `pkexec` there. A2 · B3. **V** | 1 | pending | A new folder `snap-debug-XXXXXX` with the mode `0700`. The test found that `tempfile` follows the `umask`; the mode is set explicitly. Test `the_debug_options_change_the_process`. |
| ATL-SEC-025 | N8, L-5 | S | `open_url` opened every scheme (`javascript:`, `ssh:`, the handlers of other programs), and read a path with a `:` as an address. A2 · B3. **C** | 3, 4 | pending | Only `http`, `https`, `mailto`, `file`, `tel`; a path with a colon is a path. Tests `only_known_schemes_are_opened`, `a_path_is_a_path_even_with_a_colon`. |
| ATL-SEC-026 | L5 | S | `write_file` and `move_file_into` took `..`, separators and absolute paths in names; the `.bak` of `add_line_file` was written through a link. A2 · B3. **C** | 1 | pending | Names must be plain, folders plain and relative (`InvalidInput`); the backup is made with `create_new`. Tests `write_file_takes_names_and_folders_not_paths`, `move_file_into_takes_a_name_not_a_path`, `the_backup_of_add_line_file_never_goes_through_a_link`. |

### Level Low

| ID | Ref | Kind | Finding | Found | Fixed | How it was fixed · test |
|---|---|---|---|---|---|---|
| ATL-SEC-003 | zip | S | Three more `zip` advisories for `extract` (RUSTSEC-2025-0168, 2026-0368, 2026-0369: symbolic links lead out of the folder). On 1.3.1 not reproducible: the link target was always dropped by a bug. A1 · B3. **V** | 2 | `60951c0`, pending | `zip` 9; the own zip unpacking leaves links out. Test `a_zip_cannot_write_through_a_symbolic_link_to_another_folder`. |
| ATL-SEC-005 | M6 | S | The temporary file (`write_to_temp`) was a named file in a `static`; statics are not dropped, so it stayed in `$TMPDIR` with its content. A2 · B2. **V** | 1 | `60951c0` | A file without a name (`tempfile::tempfile()`). Test `nothing_stays_behind_when_the_program_ends` (a child process; fails with the old code). |
| ATL-SEC-006 | L7 | S | `gettext-rs` 0.7: `setlocale` and `TextDomain::init` are unsound without a lock on the environment (RUSTSEC-2026-0244). A1 · B3. **R** | 2 | `60951c0` | `gettext-rs` 0.8; the calls are `unsafe` with a stated contract (see ATL-SEC-039). |
| ATL-SEC-008 | L6 | R | `delete_directory` and `get_home_directory` ended the process (no cleanup, a denial of service for the caller). A2 · B2. **C** | 1 | `a03b091` | Both return `Result` / `Option`. |
| ATL-SEC-012 | N1 | B | `download_file` failed after 30 seconds, however well it went: `timeout_recv_body` limits the whole body. A download of more than 30 s always failed. A3 · B1. **V** | 3 | pending | Silence, not duration, is limited: the transfer runs in a thread and the caller gives up after 30 s without data. Tests `a_slow_download_that_keeps_coming_is_not_cut_off`, `a_server_that_goes_silent_is_given_up_on`. |
| ATL-SEC-014 | N3 | R | `set_config` threw away all other keys when the file was damaged, wrote in place, and used the `umask` (`0664`). A2 · B2. **V** | 3 | pending | A damaged file is kept as `<name>.bad`; the new file is written atomically with the mode `0600`. Test `settings_are_stored_below_xdg_config_home`. |
| ATL-SEC-019 | L1 | B | `command_output_timeout` did not hold its limit when the program had started another program that kept the pipe (0.2 s limit, 4 s wait). A2 · B2. **V** | 1 | pending | The wait is bound by the limit; output of a program that has ended is waited for 250 ms. Tests `the_limit_holds_when_the_program_has_children_that_keep_the_pipe`, `output_of_a_program_that_left_a_child_behind_is_returned`. |
| ATL-SEC-028 | L11 | S | The web page of the changelog filtered links with a list of forbidden schemes: `\x01javascript:` passed. No Content-Security-Policy. A2 (needs a bad changelog) · B2. **V** (the filter) | 1 | pending | A list of allowed schemes (`http`, `https`, `mailto`, relative); CSP with the hashes of the script and the style, `no-referrer`. Test `the_web_page_runs_only_its_own_code`; the filter was run in Node (17 cases), the page in a browser. |
| ATL-SEC-032 | N5 | R | The number of a step (`start + i`, `start` from a TOML file) overflowed: a panic in debug, a wrap in release. A2 · B2. **C** | 3 | pending | `saturating_add`. Check `steps with a huge number` in `design_gtk`. |

### Level Info

| ID | Ref | Kind | Finding | Found | Fixed | How it was fixed · test |
|---|---|---|---|---|---|---|
| ATL-SEC-007 | – | R | `sevenz-rust` is unmaintained (RUSTSEC-2026-0246). A1 · B1. **R** | 2 | `60951c0` | Replaced by `sevenz-rust2` (see ATL-SEC-001). |
| ATL-SEC-027 | L4 | B | The documentation of `get_config_value` said that a path built from user input cannot leave its folder; only `..` is refused. A2 · B1. **C** | 1 | pending | The text says what the check does and does not. |
| ATL-SEC-029 | L10 | P | No check for known vulnerabilities in the dependencies, and no CI at all. | 1 | pending | `.github/workflows/audit.yml` (`cargo audit` on push, pull request and weekly; the action is pinned to a commit) and `.github/dependabot.yml`. **Not run on GitHub yet.** |
| ATL-SEC-030 | L12 | B | The example `test_app` printed the password. A2 · B1. **C** | 1 | pending | It prints the number of characters. |
| ATL-SEC-031 | N4 | R | `use_adw_provider*` left a handler on the style manager for ever (it holds the CSS text of every provider ever made). A2 · B1. **T** | 3 | pending | The handler disconnects itself when its provider is gone. Check `css provider` in `design_gtk` (fails when the `disconnect` is removed). |
| ATL-SEC-033 | N6 | B | A `markup` block with invalid Pango markup (or a translation that broke it) gave a wrong label. A2 · B1. **T** | 3 | pending | Checked when the guide is loaded (with the page in the message); a broken translation falls back to the original. Test `markup_must_be_valid_and_a_broken_translation_is_not_used`. |
| ATL-SEC-034 | N7 | B | A file of a server chosen in a file chooser vanished without a word. A2 · B1. **C** | 3 | pending | Logged as a warning. |
| ATL-SEC-035 | N9, L-4 | B | `unwrap()` after a check in the guide loader, `unreachable!`, and the doc comment of `run_chain` sat above another function. | 3, 4 | pending | `need()` instead of `unwrap()`; the comment moved. |
| ATL-SEC-036 | – | R | `ctutils` 0.4.3: not constant time (RUSTSEC-2026-0385); reached through `zip` and `sevenz-rust2`. The library has no password API, so it cannot be reached. A1 · B1. **R** | 3 | upstream | Fixed in `ctutils` 0.4.4. The repository has no `Cargo.lock` (it is a library): programs get the new version with `cargo update`. |

## 6. How to run an audit and update this file

### The checks

```sh
cargo audit                                  # dependencies, fetches the RustSec database
codeql-scan --fail-on none .                 # CodeQL, results in ~/.cache/codeql-scan/
cargo clippy --all-targets --features archive,download
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --features archive,download
rustfmt --edition 2024 --check src/lib.rs tests/*.rs examples/*.rs     # never a bare `cargo fmt`
gtk4-broadwayd :9 &                          # the GTK checks need a display
GDK_BACKEND=broadway BROADWAY_DISPLAY=:9 cargo test --features archive,download
scripts/check-platform.sh                    # the platform layer for Windows
```

The lints that find panics and unchecked arithmetic in the library (not in the tests):

```sh
cargo clippy --lib --features archive,download -- \
    -W clippy::unwrap_used -W clippy::expect_used -W clippy::panic -W clippy::indexing_slicing \
    -W clippy::string_slice -W clippy::cast_possible_truncation -W clippy::arithmetic_side_effects \
    -W clippy::undocumented_unsafe_blocks -W clippy::missing_panics_doc
```

### What to read, every time

* Everything that takes **data from outside**: archives, downloads, files in a folder others can
  write to, the output of commands and devices, TOML files, environment variables.
* Everything that **runs a program**, **deletes** or **overwrites**, and everything that
  **writes a log** (what, who can read it, what a terminal makes of it).
* New dependencies and new `unsafe`.

### Updating this file

1. Add a row to the [history](#3-audit-history) (number, date, scope, tools, new findings).
2. Give each new finding the next free `ATL-SEC-nnn`, rate it with the [scale](#1-the-scale) (reach,
   damage, level, evidence), put it in "Open" with the reason.
3. When a finding is fixed, move the row to the right level in "Fixed": the commit, how, and the
   test. A fix should come with a test that fails with the old code; if that is not possible, say
   why in the row.
4. Update the counts in [Latest result](#2-latest-result) and the date of the audit.
5. Replace each `pending` by the commit hash after the commit.
6. A finding that is accepted needs a reason, and a date to look at it again if the reason can change.

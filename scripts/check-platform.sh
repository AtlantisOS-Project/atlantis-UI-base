#!/bin/sh
# Checks that the platform layer and the files that depend on the operating system compile for
# other systems than Linux, without GTK: they are copied into a small crate and `cargo check`ed
# for each target.
#
#   scripts/check-platform.sh                 Linux and Windows (x86_64-pc-windows-gnu)
#   scripts/check-platform.sh <target>...     other targets
#
# The targets must be installed (rustup target add x86_64-pc-windows-gnu); a target that is
# missing is skipped. The user interface (design) needs the GTK libraries of the target and is
# not checked here.
set -eu
root=$(cd "$(dirname "$0")/.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
helper="$root/src/helper"

mkdir -p "$work/src/helper"
cp "$helper/platform.rs" "$work/src/helper/"
cp -r "$helper/platform" "$work/src/helper/"
for file in commands/command_line commands/open_terminal_desktop commands/execute_command \
    configuration/application_environment filesystem/search_string_name \
    filesystem/search_file_directory filesystem/delete_files filesystem/home_dir \
    file/get_config_value sysinfo/system_info log/write_log; do
    mkdir -p "$work/src/helper/$(dirname "$file")"
    cp "$helper/$file.rs" "$work/src/helper/$file.rs"
done

cat > "$work/Cargo.toml" <<'CARGO'
[package]
name = "platform-check"
version = "0.0.0"
edition = "2024"

[dependencies]
directories = "6.0"
confy = "0.6"
log = { version = "0.4", features = ["std"] }
serde = { version = "1.0", features = ["derive"] }
open = "5.3"
tempfile = "3"

[target.'cfg(unix)'.dependencies]
syslog = "7.0"

[features]
default = ["format-flatpak", "format-snap", "format-appimage", "format-native"]
format-flatpak = []
format-snap = []
format-appimage = []
format-native = []
CARGO
cat > "$work/src/lib.rs" <<'RUST'
pub mod helper {
    pub mod platform;
    pub mod commands {
        pub mod command_line;
        pub mod execute_command;
        pub mod open_terminal_desktop;
    }
    pub mod configuration {
        pub mod application_environment;
    }
    pub mod filesystem {
        pub mod delete_files;
        pub mod home_dir;
        pub mod search_file_directory;
        pub mod search_string_name;
    }
    pub mod file {
        pub mod get_config_value;
    }
    pub mod sysinfo {
        pub mod system_info;
    }
    pub mod log {
        pub mod write_log;
    }
}
RUST

[ "$#" -gt 0 ] || set -- "$(rustc -vV | sed -n 's/^host: //p')" x86_64-pc-windows-gnu
status=0
for target in "$@"; do
    if ! rustup target list --installed 2>/dev/null | grep -qx "$target"; then
        echo "skipped $target (rustup target add $target)"
        continue
    fi
    if (cd "$work" && cargo check --quiet --target "$target" 2>&1 | grep -E '^error' -A8); then
        echo "FAILED $target"
        status=1
    else
        echo "ok     $target"
    fi
done
exit $status

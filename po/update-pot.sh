#!/bin/sh
# Collects the texts of the library into po/atlbase.pot and merges them into the catalogs
# po/<lang>.po. Needs xgettext and msgmerge (package gettext).
#
#   po/update-pot.sh
#
# xgettext does not know Rust. In a copy of the sources the macro `gettext!(` is renamed to
# the C function `gettext(` (the line numbers stay the same). The unfinished module `system`
# is left out. Translate the new entries in po/<lang>.po afterwards and check for "fuzzy".
set -eu
root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

(cd src && find . -name '*.rs' -not -path './system*' | while read -r file; do
    mkdir -p "$work/src/$(dirname "$file")"
    sed -e 's/gettext!(/gettext(/g' -e "s/'static/static_/g" "$file" > "$work/src/$file"
done)

version=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
(cd "$work" && xgettext -L C --keyword=gettext --from-code=UTF-8 --sort-output --no-wrap \
    --package-name=atlbase --package-version="$version" \
    --copyright-holder="AtlantisOS Project" \
    --msgid-bugs-address=https://github.com/AtlantisOS-Project/atlantis-UI-base/issues \
    -o "$root/po/atlbase.pot" $(find src -name '*.rs' | sort)) 2>/dev/null

# fill in what xgettext leaves as a placeholder
created=$(sed -n 's/^"POT-Creation-Date: \(.*\)\\n"$/\1/p' po/atlbase.pot)
sed -i -e 's/^# SOME DESCRIPTIVE TITLE\.$/# Translation template of atlbase./' \
    -e 's/YEAR AtlantisOS/2026 AtlantisOS/' \
    -e '/^# FIRST AUTHOR/d' -e '/^#, fuzzy$/d' \
    -e "s/YEAR-MO-DA HO:MI+ZONE/$created/" po/atlbase.pot

for po in po/*.po; do
    [ -e "$po" ] || continue
    msgmerge --update --backup=none --quiet --no-wrap "$po" po/atlbase.pot
    # texts that are not in the library any more
    msgattrib --no-obsolete --no-wrap -o "$po" "$po"
done
echo "po/atlbase.pot updated"

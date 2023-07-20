#!/usr/bin/env bash
set -euo pipefail

# shellcheck disable=2016
jq_script='{"key": (.CFBundleDisplayName // .CFBundleName // .CFBundleExecutable), "value": .CFBundleIdentifier, "icon": "\($app_path)/Resources/\(.CFBundleIconFile | if . | test("\\.") then . else "\(.).icns" end)" }'

find -L "/Applications" "$HOME/Applications" \
	-maxdepth 4 \
	-type f \
	-iname 'Info.plist' \
	-path '*/Contents/*' \
	-exec sh -c 'path="$1"; script="$2"; plutil -convert json -o - "$path" | jq -c --arg app_path "$(dirname -- "$path")" "$script"' shell {} "$jq_script" \;

# TODO: pipe to menu, and pipe selection to `open -b`

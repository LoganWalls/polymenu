#!/usr/bin/env bash
set -euo pipefail

CACHE_DIR="$1"
ICON_DIR="${CACHE_DIR}/icons"
INDEX_FILE="$CACHE_DIR/index.json"

echo "Building app launcher cache at: $CACHE_DIR"

# Where to look for .desktop files
desktop_dirs=(
  "$HOME/.local/share/applications"
  /usr/local/share/applications
  /usr/share/applications
  /var/lib/snapd/desktop
  "$HOME/.local/share/flatpak/exports/share/applications"
  /var/lib/flatpak/exports/share/applications
)

# Where to look for icons (themes + pixmaps)
icon_dirs=(
  "$HOME/.local/share/icons" "$HOME/.icons"
  /usr/share/icons /usr/local/share/icons
  /usr/share/pixmaps /usr/local/share/pixmaps
)
icon_exts=(png svg xpm)

mkdir -p "$CACHE_DIR"
mkdir -p "$ICON_DIR"

resolve_icon() {
  local icon="$1"
  [[ -z ${icon:-} ]] && return 0

  if [[ $icon == /* ]]; then
    if [[ -f $icon ]]; then
      printf '%s' "$icon"
      return
    fi
    local stem="${icon%.*}"
    for ext in "${icon_exts[@]}"; do
      [[ -f "${stem}.${ext}" ]] && {
        printf '%s' "${stem}.${ext}"
        return
      }
    done
    return 0
  fi

  for dir in "${icon_dirs[@]}"; do
    [[ -d $dir ]] || continue
    for ext in "${icon_exts[@]}"; do
      if [[ -f "$dir/$icon.$ext" ]]; then
        printf '%s' "$dir/$icon.$ext"
        return
      fi
    done
  done

  for dir in "${icon_dirs[@]}"; do
    [[ -d $dir ]] || continue
    local found=""
    for ext in "${icon_exts[@]}"; do
      found="$(find "$dir" -type f -iname "$icon.$ext" -print -quit 2> /dev/null || true)"
      [[ -n $found ]] && {
        printf '%s' "$found"
        return
      }
    done
    found="$(find "$dir" -type f -iname "$icon.*" -print -quit 2> /dev/null || true)"
    [[ -n $found ]] && {
      printf '%s' "$found"
      return
    }
  done
}

stage_icon() {
  local src="$1" id="$2"
  [[ -z ${src:-} ]] && return 0
  [[ -f $src ]] || return 0

  local ext="${src##*.}"
  ext="${ext,,}"
  case "$ext" in
    png | svg | xpm) : ;;
    *) ext="png" ;;
  esac
  local fname="${id}.${ext}"
  local dest="$ICON_DIR/$fname"

  if [[ -e $dest ]]; then
    if ! cmp -s -- "$src" "$dest"; then
      cp -fL -- "$src" "$dest"
    fi
  else
    cp -fL -- "$src" "$dest"
  fi

  printf '%s' "$fname"
}

declare -A SEEN
tmpfile="$(mktemp)"
trap 'rm -f "$tmpfile"' EXIT

for ddir in "${desktop_dirs[@]}"; do
  [[ -d $ddir ]] || continue
  find "$ddir" -maxdepth 2 -type f -name '*.desktop' -print0 2> /dev/null \
    | while IFS= read -r -d '' f; do
      id="$(basename "$f" .desktop)"
      [[ ${SEEN[$id]:-} ]] && continue
      SEEN[$id]=1

      type=""
      name=""
      exec=""
      icon=""
      nod=""
      hid=""
      while IFS='=' read -r k v; do
        case $k in
          TYPE) type=$v ;;
          NAME) [[ -z $name ]] && name=$v ;;
          EXEC) exec=$v ;;
          ICON) icon=$v ;;
          NOD) nod=$v ;;
          HID) hid=$v ;;
        esac
      done < <(
        awk '
        BEGIN{insec=0}
        /^\[Desktop Entry\]/{insec=1;next}
        /^\[/ && insec{insec=0}
        insec && /^Type=/{sub(/^Type=/,""); print "TYPE="$0}
        insec && /^Name=/{sub(/^Name=/,""); print "NAME="$0}
        insec && /^Exec=/{sub(/^Exec=/,""); print "EXEC="$0}
        insec && /^Icon=/{sub(/^Icon=/,""); print "ICON="$0}
        insec && /^NoDisplay=/{sub(/^NoDisplay=/,""); print "NOD="$0}
        insec && /^Hidden=/{sub(/^Hidden=/,""); print "HID="$0}
      ' "$f"
      )

      [[ $type == "Application" ]] || continue
      [[ ${hid,,} == true ]] && continue
      [[ ${nod,,} == true ]] && continue
      [[ -z $name || -z $exec ]] && continue

      icon_path="$(resolve_icon "$icon")"
      fname="$(stage_icon "$icon_path" "$id")"

      jq -n --arg key "$name" --arg val "$f" --arg icon "$fname" \
        '{key:$key, value:$val, icon:$icon}' >> "$tmpfile"
    done
done

# Write a proper JSON array
jq -s '.' "$tmpfile" > "$INDEX_FILE"

echo "✅ Done!"

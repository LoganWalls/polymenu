#!/usr/bin/env nu

def main [cache_root: path] {
  const script_dir = (path self | path dirname)
  let cache_icons = $"($cache_root)/icons"

  print $"Building app launcher cache at: ($cache_root)"

  mkdir $cache_root $cache_icons
  fd "^Info.plist$" "/Applications" $"($env.HOME)/Applications" --type file --max-depth 4
  | lines 
  | par-each {|path|
    let info = (plutil -convert json -o - $path | from json)
    let app_path = ($path | path dirname | path dirname)
    let app_name = ($info.CFBundleDisplayName? 
                    | default  $info.CFBundleName?
                    | default $info.CFBundleExecutable?)
    let app_icon = $"($cache_icons)/($app_name).png"
    if (not ($app_icon | path exists)) {
      osascript $"($script_dir)/render-icon.applescript" $app_path $app_icon 512
    }
    { "key": $app_name,
      "value": $info.CFBundleIdentifier,
      "icon": ($app_icon | path basename)
    }
  }
  | sort-by key
  | to json
  | save -f $"($cache_root)/index.json"

  print "✅ Done!"
}


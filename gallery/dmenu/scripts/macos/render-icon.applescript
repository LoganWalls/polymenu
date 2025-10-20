use framework "Foundation"
use framework "AppKit"
use scripting additions

-- Usage:
--   osascript extract_app_icon.applescript "/Applications/App.app" "./AppIcon.png" 512
--   Size argument is optional; defaults to 512.

on run argv
    if (count of argv) < 2 then error "Usage: extract_app_icon.applescript <AppPath> <OutPNG> [Size]"

    set appPath to item 1 of argv
    set outPath to item 2 of argv
    set iconSize to 512
    if (count of argv) ≥ 3 then set iconSize to (item 3 of argv) as integer

    set posixOut to POSIX path of outPath
    set outDir to do shell script "dirname -- " & quoted form of posixOut
    do shell script "mkdir -p -- " & quoted form of outDir

    set ws to current application's NSWorkspace's sharedWorkspace()
    set img to ws's iconForFile_(appPath)
    if img is missing value then error "iconForFile_ returned missing value"

    img's setSize_(current application's NSMakeSize(iconSize, iconSize))

    set tiffData to img's TIFFRepresentation()
    if tiffData is missing value then error "No TIFFRepresentation from NSImage"

    set rep to current application's NSBitmapImageRep's imageRepWithData_(tiffData)
    if rep is missing value then error "NSBitmapImageRep.imageRepWithData_ failed"

    set emptyDict to current application's NSDictionary's dictionary()
    set pngData to rep's representationUsingType_properties_(current application's NSPNGFileType, emptyDict)
    if pngData is missing value then error "PNG encode failed"

    set ok to pngData's writeToFile_atomically_(outPath, true)
    if (ok as boolean) is false then error "writeToFile failed"
end run

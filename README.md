# Polymenu
`polymenu` is a cross-platform (Wayland, X11, macOS, Windows) tool that allows you to
create personalized ephemeral GUI interfaces for your CLI tools and scripts
using your favorite front-end web framework.

In terms of use-cases, `polymenu` is similar to
[`rofi`](https://github.com/davatorium/rofi), but cross-platform, supports
structured input, and customized using HTML, CSS, and TypeScript / JavaScript
instead of `rasi` files. In terms of philosophy, `polymenu` has a lot in common
with [`neovim`](https://neovim.io/) and [`emacs`](https://www.gnu.org/software/emacs/emacs.html): 
it provides a set of composable features from which you are encouraged to 
customize the software to your liking.

<!-- Gallery screenshots here -->

## Usage
You can use `polymenu` to define one or more custom "apps" (for example, you
might have one app that acts like a fuzzy finder menu, another app that
controls your system audio, and another that is a wallpaper switcher).

Each app needs two things: a `config.toml` file that defines how information 
will be passed between the CLI and GUI, and a client-side web app that defines 
the GUI.

Apps are invoked from the CLI via the `polymenu` binary:

```sh
polymenu --config path/to/config.toml
```

### `config.toml`
The only property required for a `config.toml` is `src`, which specifies
the path to the project root of your client-side web app's source code.
For example:

```toml
src = "$HOME/code/polymenu-apps/my-first-app"
```

Note that you can use environment variables (and `~`) in your `config.toml`
paths.


### App input
Apps can receive input from `STDIN`:
```sh
cat foo.json | polymenu --format json
```

Or read from a file directly:
```sh
polymenu --file foo.json
```

Inputs can be structured JSON-like objects, where the following formats are
supported:
- JSON
- JSON Lines
- CSV (parsed as one JSON object per row)
- Headless CSV (parsed as `string[][]`)
- Raw (parsed as `string`)

If you always want to read from the same file, you can specify it in your
`config.toml`:
```toml
file = "~/path/to/foo.json"
# format = "json" # You can also specify an input format, but polymenu will infer it from the file's extension by default
```

You can access the input from JS/TS:

```ts
input: Promise<JsonValue> = app.input();
```

### Calling CLI tools and scripts
To allow your app to call CLI tools or scripts, you can define `commands` in
your `config.toml`:

```toml
[commands.say_anything]
command = [
  "echo",
  "$message", # Use `$argument_name` to reference arguments that you will pass from your app 
]
# output_format = "raw" # Same options as `format`, defaults to `raw`

[commands.count_lines]
command = ["wc", "-l"]

[commands.get_records]
command = ["query_database.sh"]
output_format = "csv"
```

You can then call these commands from JS/TS and get access to the outputs:
```ts
// You can pass arguments to a command:
let response: Promise<""> = app.runCommand("say_anything", { message: "Watermelon!" });
// You can also pass lines to the command's STDIN:
let lineCount: Promise<string> = app.runCommand("say_anything",  null, ["first_line", "second_line", "third_line"]);
// Depending on the command's `output_format`, you will receive structured data:
let databaseRecords: Promise<JsonValue[]> = app.runCommand("get_records");
```

### Reading files
Sometimes your app may want to read files from disk (for example, to 
display images). You can achieve this by defining mounted directories in
your `config.toml`:

```toml
[mounts]
pictures = "$MY_PROJECT/icons"
music = "$MY_PROJECT/audio"
```

Your files will then be accessible at `/files/{mount_name}/{file_path}`.
So for example, using the `config.toml` fragment above, you can display
`$MY_PROJECT/icons/foo/bar.png` in your app like so:

```html
<img src="/files/pictures/foo/bar.png" />
```

You can also override mounts at runtime my passing `--mount name:path`:
```sh
polymenu --mount pictures:$MY_OTHER_PROJECT/photos
```


### Runtime options
Sometimes you may want an app to have options that can be overridden at runtime.
You can achieve this by specifying an `options` block in your `config.toml` with
the option names and their default values:

```toml
[options]
placeholder = "Search for something!"
case_sensitive = false
```

You can override these values at runtime from the CLI by passing: `--option name=value`:
```sh
polymenu --option 'placeholder="Or not"' --option case_sensitive=true
```

Then you can access these values in JS/TS:
```ts
const options: Record<string, JsonValue> = app.options;
```

### Customizing the window
You can customize attributes of the app window itself using both
CLI options:
```
  -w, --window-width <WIDTH>
          Window's width in pixels

  -h, --window-height <HEIGHT>
          Window's height in pixels

  -x, --window-x <X>
          Window's x coordinate in pixels

  -y, --window-y <Y>
          Window's y coordinate in pixels

      --window-opaque
          Whether or not to use an opaque window (default is transparent)

      --window-no-focus
          Do not autofocus the window

      --window-decorations
          Whether or not the window should have decorations
```

And `config.toml`:

```toml
[window]
width = 700
height = 1000
x = 100
y = 100
no_focus = false
opaque = false
window-decorations = false
```

## Installation
### Cargo
1. Make sure you have installed the [rust toolchain](https://www.rust-lang.org/tools/install)
2. Clone this repository and `cd` into it
3. Compile and install the binary: `cargo install --path ./app`


### Nix
To try out temporarily:
```sh
nix shell 'github:LoganWalls/polymenu'
```

Install via nix flakes:
```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    targets = {
        url = "github:LoganWalls/polymenu";
        inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {nixpkgs, ...}@inputs: let
    inherit (nixpkgs) lib;
    withSystem = f:
      lib.fold lib.recursiveUpdate {}
      (map f ["x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin"]);
  in
    withSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        polymenu = inputs.polymenu.packages.${system}.default;
        # Now you can use `polymenu` in packages lists (in shells, or in your
        # `configuration.nix` / `home.nix`)
      in
        with pkgs; {
          devShells.${system}.default = mkShell {packages = [ polymenu ];};
        }
    );
};
```


## Acknowledgements
This project would not have been possible without [a little help from my
friends](https://www.youtube.com/watch?v=0C58ttB2-Qg). Thank you all, and
especially to:
- [Maddison Hellstrom](https://github.com/b0o)
- [AlphaKeks](https://github.com/AlphaKeks)
- [Stefan Todorov](https://github.com/coravacav)

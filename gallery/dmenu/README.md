This app is a `dmenu` -like interface that allows you to fuzzy-find through a list
of inputs, select one or more of those inputs, and then prints values associated
with your selection to `STDOUT`.

**All example commands assume you are in the same directory as this README.**

## Installation
Compile the app using polymenu:
```sh
polymenu --config ./config.toml --src ./src --compile
```

## Basic usage

```sh
"Hello\nFriend" | polymenu --config ./config.toml --src ./src
```

For structured inputs, each input should have the following spec:
```ts
interface Item {
    // Used as the display name, the search key, and the printed value
    key: string, 
    // If present, this value will be printed instead of `key` when you select this item.
    value?: string, 
    // If present, this will be interpreted as a path relative to `/files/icons` and loaded
    // as an icon for this option. (Mount a directory with the name `icons` to use this).
    icon?: string,
}
```

There are several options that can be used to customize the app's behavior. For
example, passing `--option case_sensitive=true` will enable case-sensitivity for
the fuzzy matcher. For a full list of options, take a look at the app's
[`config.toml`](./config.toml).


## App-Launcher Script 
A common usage of `dmenu` is to launch applications. This directory provides

### Dependencies
These scripts perform some non-trivial computation to gather all of your
launch-able applications and their icons. To use the launcher scripts you will
need the following dependencies:

#### Linux
- [`jq`](https://jqlang.org/download/)
- [`gio`](https://en.wikipedia.org/wiki/GIO_(software)) (most distros include this)

#### macOS
- [`nu`](https://www.nushell.sh/book/installation.html)
- [`fd`](https://github.com/sharkdp/fd)

### Usage
```sh
scripts/{linux or macos}/launch.sh
```

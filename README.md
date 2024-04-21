<h1 align="center">rofi-games</h1>

<p align="center">Game launching plugin for `rofi`. Requires a good theme for the best results.</p>

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL_v2-green.svg" alt="License: GPL v2" />
  <img src="https://img.shields.io/github/v/tag/rolv-apneseth/rofi-games?label=version&color=blueviolet" alt="version" />
  <a href="https://aur.archlinux.org/packages/rofi-games"><img src="https://img.shields.io/aur/version/rofi-games" alt="AUR version" /></a>
</p>

![rofi-games - demo image](https://github.com/Rolv-Apneseth/rofi-games/assets/69486699/62b89187-c94d-464f-a942-2e66385db5e0)

## Installation

### AUR

```bash
paru -S rofi-games
```

---

### just

1. Clone repo:

    ```bash
    git clone https://github.com/Rolv-Apneseth/rofi-games.git
    ```

2. Use `just` to install (requires `cargo` and `just`)

    ```bash
    cd rofi-games && sudo just install
    ```

Uninstall with `sudo just uninstall`

---

### Manual (not recommended)

```bash
git clone https://github.com/Rolv-Apneseth/rofi-games.git
cd rofi-games
cargo build --release --lib
sudo cp target/release/librofi_games.so /usr/lib/rofi/games.so
```

- If you are using the latest changes from the `rofi` repo (e.g. `rofi-lbonn-wayland-git`, `rofi-git`), then the build step needs to be preceded by `RUSTFLAGS="--cfg rofi_next"` for it to work

## Theme

For the optimal experience, and to achieve what is shown in the demo image, use a good `rofi` theme. You can use one of the default themes provided with `rofi-games` but I recommend customising it at least a little bit (at least add the search icon). More information about themes [here](./themes/).

- To run with the default theme, the full command becomes:

    ```bash
    rofi -modi games -show games -theme games-default
    ```

- If you customised a theme and named the file `games.rasi`, it becomes:

    ```bash
    rofi -modi games -show games -theme games
    ```

## Keybinds

| Keybind           | Default rofi keybind              | Action                     |
|-------------------|-----------------------------------|----------------------------|
| `kb-accept-entry` | <kbd>Enter</kbd>                  | Launch game                |
| `kb-accept-alt`   | <kbd>Shift</kbd>+<kbd>Enter</kbd> | Open game's root directory |

- To change a `rofi` keybind, you can, for example, use `-kb-accept-entry Ctrl+c`

## Currently supported launchers / game sources

Parsing of installed games has been extracted into a separate library: [lib_game_detector](https://github.com/Rolv-Apneseth/lib_game_detector)

However, only games which have box art are valid for this launcher so not everything detected by that will be available for launch here. The following sources are currently supported:

- Steam
  - Not non-Steam games though, if anyone knows where the box art for these is stored let me know as I can't find them
- Heroic Games Launcher (all sources, including Amazon, should work)
  - The `GOG` source doesn't store the box art like the others, but the icons, so they won't look good
- Lutris
  - Must have box/cover art configured (or define a custom entry - see configuration section below). Some may already have cover art (like the Epic Games launcher) but I would recommend having a look over at [SteamGridDB](https://www.steamgriddb.com/grids) if you want to look for a better one. To set the cover art, simply right click the entry in the Lutris library, select "Configure", and click on the left-most image to select a new image file.
- Bottles
  - Only games which are in the Library and have a box/cover art (or have box art defined in a custom entry - see configuration section below) are displayed

## Configuration

Custom entries, for unsupported games (or technically anything you want), can be made by creating a config file at `~/.config/rofi-games/config.toml` (`$XDG_CONFIG_HOME` is respected). Here is an example configuration:

    ```toml
    box_art_dir = "/home/rolv/.config/rofi-games/box-art"

    [[entries]]
    title = "GDLauncher"
    launch_command = "gdlauncher"
    path_box_art = "gdlauncher.png"
    path_game_dir = "/opt/GDLauncher"

    [[entries]]
    title = "Cyberpunk 2077"
    path_box_art = "/home/rolv/images/cyberpunk.png"
    ```

- The `box_art_dir` field is optional, but will be used if the `path_box_art` field of a given entry is not an absolute path.

- In the first entry, a custom entry is defined for `GDLauncher`. All fields must be defined.

- In the second entry, a game already detected by `rofi-games` is matched by the `title` field. The
  custom entry is used to override certain fields for that entry, e.g. changing the box art image.
  - This can also be used to set box art images for the sources listed above that require a box art
  image defined in the launcher itself e.g. Lutris
  - **Note:** if you have multiple entries which match a given title, doing this will override the fields
      on the first match, and remove all the other matches

## Credit

The original idea belongs (as far as I know) to [@ntcarlson](https://github.com/ntcarlson), so big thanks to them. The original script I used to use for this was derived from [this Reddit post](https://www.reddit.com/r/unixporn/comments/p5b0qv/i3_misusing_rofi_as_a_game_launcher/) they shared.

Converting to an actual `rofi` plugin is thanks to the author of [rofi-mode.rs library](https://github.com/SabrinaJewson/rofi-mode.rs), and thanks also to the author [rofi-vscode-mode](https://github.com/fuljo/rofi-vscode-mode) as that was very helpful as an example for `rofi-mode` usage and the `PKGBUILD`.

<h1 align="center">rofi-games</h1>

<p align="center">Game launching plugin for <code>rofi</code>. Requires a good theme for the best results.</p>

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
- The last `cp` command assumes that the `rofi` plugins directory is `/usr/lib/rofi`, which may not be the case for you. Use `pkg-config --variable pluginsdir rofi` to find the one for your system, though if there is no output from that command, you may need to just try `/usr/lib64/rofi` or `/usr/lib/rofi` (the install script in the `justfile` falls back to these in that order).

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
  - Steam shortcuts (non-Steam games) are also supported, just ensure the shortcuts has a box art image, using either the Steam UI or checking out the configuration section below.

> [!TIP]
> To add box art for a shortcut using the Steam UI, navigate to the Steam library page (where the different games' box art is shown) and find the desired shortcut, right-click -> Manage -> Set custom artwork

- Heroic Games Launcher (all sources, including Amazon, should work)
  - The `GOG` source doesn't store the box art like the others, but the icons, so they won't look good
- Lutris
  - Must have box/cover art configured (or define a custom entry - see configuration section below). Some may already have cover art (like the Epic Games launcher) but I would recommend having a look over at [SteamGridDB](https://www.steamgriddb.com/grids) if you want to look for a better one. To set the cover art, simply right click the entry in the Lutris library, select "Configure", and click on the left-most image to select a new image file.
- Bottles
  - Only games which are in the Library and have a box/cover art (or have box art defined in a custom entry - see configuration section below) are displayed

- Instances from the following modded Minecraft launchers:
    1. Prism Launcher
    2. ATLauncher

> [!NOTE]
> Modded Minecraft instances are not shown by default (as they don't have box art), so look at the configuration section below to see how to match an instance title to define box art for it

## Configuration

Custom entries, for unsupported games (or technically anything you want), can be made by creating a config file at `~/.config/rofi-games/config.toml` (`$XDG_CONFIG_HOME` is respected). Here is an example configuration:

```toml
# Directory to find box art in if an absolute path is not given
box_art_dir = "/home/rolv/.config/rofi-games/box-art"

# Define a new custom entry
[[entries]]
title = "GDLauncher"
launch_command = "gdlauncher"
# Looks for the image in the defined `box_art_dir`
path_box_art = "gdlauncher.png"
path_game_dir = "/opt/GDLauncher"

# Change box art for a title
[[entries]]
title = "Cyberpunk 2077"
path_box_art = "/home/rolv/images/cyberpunk.png"

# Define box art for a title e.g. Minecraft instances
# Modded Minecraft instances are always prefixed with "Minecraft: "
[[entries]]
title = "Minecraft: Fabulously Optimized"
path_box_art = "fabulously_optimized.png"
```

- The `box_art_dir` field is optional, but will be used if the `path_box_art` field of a given entry is not an absolute path.

- In the first entry, a custom entry is defined for `GDLauncher`. All fields must be defined, except for `path_game_dir`.

- In the second entry, a game already detected by `lib_game_detector` is matched by the `title` field. The
  custom entry is used to override certain fields for that entry, e.g. changing the box art image.
  - This can also be used to set box art images for the sources listed above that require a box art
  image defined in the launcher itself e.g. Lutris

> [!WARNING]
> If you have multiple games which match a given title (e.g. from multiple launchers), creating a custom entry will only override the fields on the first match, and other matches will be ignored. If you have multiple entries in your configuration file with the same title, they will override each other.

> [!TIP]
> To run a script with spaces in the launch command, you will need to use `\\` before any space characters, e.g. `launch_command = "/home/user/GOG\\ Games/Stardew\\ Valley/start.sh"`. Other options can be passed in directly without escaping spaces.

## Credit

The original idea belongs (as far as I know) to [@ntcarlson](https://github.com/ntcarlson), so big thanks to them. The original script I used to use for this was derived from [this Reddit post](https://www.reddit.com/r/unixporn/comments/p5b0qv/i3_misusing_rofi_as_a_game_launcher/) they shared.

Converting to an actual `rofi` plugin is thanks to the author of [rofi-mode.rs library](https://github.com/SabrinaJewson/rofi-mode.rs), and thanks also to the author [rofi-vscode-mode](https://github.com/fuljo/rofi-vscode-mode) as that was very helpful as an example for `rofi-mode` usage and the `PKGBUILD`.

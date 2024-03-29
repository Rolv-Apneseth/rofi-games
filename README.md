<h1 align="center">rofi-games</h1>

<p align="center">A rofi plugin which adds a mode that will list available games for launch along with their box art. Requires a good theme for the best results.</p>

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
  - The `GOG` source doesn't store the box art like the others, but the icons so they won't look good
- Lutris
  - Must have box/cover art configured. Some may already have cover art (like the Epic Games launcher) but I would reccommend having a look over at [SteamGridDB](https://www.steamgriddb.com/grids) if you want to look for a better one. To set the cover art, simply right click the entry in the Lutris library, select "Configure", and click on the left-most image to select a new image file.
- Bottles
  - Only games which are in the Library and have a box/cover art are displayed

## Acknowledgement

The original idea belongs (as far as I know) to [@ntcarlson](https://github.com/ntcarlson), so big thanks to them. The original script I used to use for this was derived from [this Reddit post](https://www.reddit.com/r/unixporn/comments/p5b0qv/i3_misusing_rofi_as_a_game_launcher/) they shared.

Converting to an actual `rofi` plugin is thanks to the author of [rofi-mode.rs library](https://github.com/SabrinaJewson/rofi-mode.rs), and thanks also to the author [rofi-vscode-mode](https://github.com/fuljo/rofi-vscode-mode) as that was very helpful as an example for `rofi-mode` usage and the `PKGBUILD`.

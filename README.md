<h1 align="center">rofi-games</h1>

<p align="center">A rofi plugin which adds a mode that will list available games for launch along with their box art. Requires a good theme for the best results.</p>

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL_v2-green.svg" alt="License: GPL v2" />
  <img src="https://img.shields.io/github/v/tag/rolv-apneseth/rofi-games?label=version&color=blueviolet" alt="version" />
  <a href="https://aur.archlinux.org/packages/rofi-games"><img src="https://img.shields.io/aur/version/rofi-games" alt="AUR version" /></a>
</p>

<img alt="rofi-games - demo image" src="https://user-images.githubusercontent.com/69486699/235387869-ecf5aa58-99bb-46d2-96e8-871773adc4d1.png" />

## Installation

### Manual

1. Clone repo:

    ```bash
    git clone https://github.com/Rolv-Apneseth/rofi-games.git
    ```

2. Use make to install (requires `cargo`)

    ```bash
    cd rofi-games && sudo make install
    ```

Uninstall with `sudo make uninstall`

---

### AUR

```bash
paru -S rofi-games
```

## Usage

After installing, for (very) basic usage simply run the following command:

```bash
rofi -modi games -show games -show-icons
```

However, it is highly recommended to use a theme that can properly show off the box art images

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

Keybinds are set by `rofi`, the following are just the default values.

- *Launch game:* `Enter` OR `Ctrl+j` OR `Ctrl+m`
- *Open game's root directory*: `Shift+Enter`
  - This uses `xdg-open path/to/dir` so if it's not opening with your desired program, try setting a different default with, e.g., `xdg-mime default thunar.desktop inode/directory`

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

## Attribution

The original idea belongs (as far as I know) to [@ntcarlson](https://github.com/ntcarlson), so big thanks to them. The original script I used to use for this was derived from [this Reddit post](https://www.reddit.com/r/unixporn/comments/p5b0qv/i3_misusing_rofi_as_a_game_launcher/) they shared.

Converting to an actual `rofi` plugin is thanks to the author of [rofi-mode.rs library](https://github.com/SabrinaJewson/rofi-mode.rs), and thanks also to the author [rofi-vscode-mode](https://github.com/fuljo/rofi-vscode-mode) as that was very helpful as an example for `rofi-mode` usage, the `Makefile` and `PKGBUILD`.

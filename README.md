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
    cd rofi-games && make install
    ```

Uninstall with `make uninstall`

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

For the optimal experience, and to achieve what is shown in the demo image, use a good `rofi` theme.

The theme used in the demo image can be found in my dotfiles [here](https://github.com/Rolv-Apneseth/.dotfiles/tree/main/rofi/.config/rofi). To use it, follow these steps:

1. Clone that repo and take the `.rasi` files (or just copy the contents of the files). The relevant files are `colours.rasi`, `launcher.rasi` and `games.rasi`
2. Put these with your `rofi` config, usually at `~/.config/rofi`
3. Run `rofi` with `-theme games`, so the full command becomes:

    ```bash
    rofi -modi games -show games -theme games
    ```

    - Yes I know, *games games games*, at least it's easy to remember

4. Modify the theme to suit your needs / preferences by editing the `.rasi` files

## Currently supported launchers / game sources

- Steam
- Heroic Games Launcher

## Credits

### Idea
The original idea belongs (as far as I know) to [@ntcarlson](https://github.com/ntcarlson), so big thanks to them. The original script I used to use for this is from [this Reddit post](https://www.reddit.com/r/unixporn/comments/p5b0qv/i3_misusing_rofi_as_a_game_launcher/) they shared.

However, I had modified it as I didn't like that sub menu for each game, and the script didn't work to update entries sometimes (as well as it didn't remove old entries for games I no longer had installed), so eventually I decided to have a go at making a Rust program out of it.

### Plugin
Converting to an actual `rofi` plugin is thanks to the author of [rofi-mode.rs library](https://github.com/SabrinaJewson/rofi-mode.rs), and thanks also to the author [rofi-vscode-mode](https://github.com/fuljo/rofi-vscode-mode) as that was very helpful as an example for both `rofi-mode` usage, the `Makefile` and `PKGBUILD`.

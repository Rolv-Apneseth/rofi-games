<h1 align="center">rofi-games</h1>

<p align="center">A small program which makes a `rofi` game launcher menu possible by creating `.desktop` entries for games</p>

<p align="center">
  <img src="https://img.shields.io/github/v/tag/rolv-apneseth/rofi-games?label=version" alt="version" />
  <a href="https://crates.io/crates/rofi-games"><img src="https://img.shields.io/crates/v/rofi-games.svg" alt="crates.io link"></a>
  <img src="https://img.shields.io/badge/License-GPL_v2-blue.svg" alt="License: GPL v2" />
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

### Cargo

```bash
cargo install rofi-games
```

#### Note

If installing with `cargo`, make sure the `bin` folder is on your `$PATH` by adding the following line to your `.bashrc` or `.xprofile` etc.:

```bash
export PATH="${PATH:+${PATH}}:$HOME/.cargo/bin"
```

If you have set `$CARGO_HOME`, use this instead:

```bash
export PATH="${PATH:+${PATH}}:$CARGO_HOME/bin"
```

---

### AUR

```bash
paru -S rofi-games
```

## Usage

After installing, simply run `rofi-games sync` to generate `.desktop` files at `$XDG_DATA_HOME/applications/rofi_games`

To display a menu with these using `rofi`, use the following command:

```bash
rofi -show drun -drun-categories RofiGames -show-icons
```

These can be combined for an easy one-liner to run with a keybind:

```bash
rofi-games sync && rofi -show drun -drun-categories RofiGames -show-icons
```

### Commands

- **sync**: Sync desktop entries by deleting any entries which don't match any currently detected game, then creating desktop entries for any detected games which don't have a matching desktop entry. This is what I would recommend as the default way to run this program

- **reset**: Reset desktop entries by completely deleting the current desktop entries folder this program creates, then re-generating desktop entries for all detected games

- **delete**: Simply deletes the desktop entries folder created by this program

## Theme

For the optimal experience, and to achieve what is shown in the demo image, use a good `rofi` theme.

The theme used in the demo image can be found in my dotfiles [here](https://github.com/Rolv-Apneseth/.dotfiles/tree/main/rofi/.config/rofi). To use it, follow these steps:

1. Clone that repo and take the `.rasi` files (or just copy the contents of the files). The relevant files are `colours.rasi`, `launcher.rasi` and `games.rasi`
2. Put these with your `rofi` config, usually at `~/.config/rofi`
3. Run `rofi` with `-theme games`, so the full command becomes:

    ```bash
    rofi-games sync && rofi -show drun -drun-categories RofiGames -theme games
    ```

4. Modify config files to suit your needs / preferences

## Todo

- Support more than just Steam games (depends how difficult this is to implement)
- Change approach to no longer use desktop entries so as to not clutter up the `drun` menu
- Figure out how to make this into actual `rofi` extension to be run with `-modi`

## Credits

The original idea belongs (as far as I know) to [@ntcarlson](https://github.com/ntcarlson), so big thanks to them. The original script I used to use for this is from [this Reddit post](https://www.reddit.com/r/unixporn/comments/p5b0qv/i3_misusing_rofi_as_a_game_launcher/) they shared.

However, I had modified it as I didn't like that sub menu for each game, and the script didn't work to update entries sometimes (as well as it didn't remove old entries for games I no longer had installed), so eventually I decided to have a go at making a Rust program out of it.

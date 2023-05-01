# rofi-games

A small program which makes a rofi game launcher menu possible by creating .desktop entries for games

![demo_image](https://user-images.githubusercontent.com/69486699/235387869-ecf5aa58-99bb-46d2-96e8-871773adc4d1.png)

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

Uninstall with `make uninsntall`

## Usage

After installing, simply run `rofi-games` to generate `.desktop` files at `$XDG_DATA_HOME/applications/rofi_games`

To display a menu with these using `rofi`, use the following command:

```bash
rofi -show drun -drun-categories SteamLibrary
```

These can be combined for an easy one-liner to run with a keybind:

```bash
rofi-games && rofi -show drun -drun-categories SteamLibrary
```

## Theme

For the optimal experience, and to achieve what is shown in the demo image, use a good `rofi` theme.

The theme used in the demo image can be found in my dotfiles [here](https://github.com/Rolv-Apneseth/.dotfiles/tree/main/rofi/.config/rofi). To use it, follow these steps:

1. Clone that repo and take the `.rasi` files (or just copy the contents of the files). The relevant files are `colours.rasi`, `launcher.rasi` and `games.rasi`
2. Put these with your `rofi` config, usually at `~/.config/rofi`
3. Run `rofi` with `-theme games`, so the full command becomes:

    ```bash
    rofi-games && rofi -show drun -drun-categories RofiGames -theme games
    ```

4. Modify config files to suit your needs / preferences

## Todo

- Add install methods (`crate` and possibly `AUR`)
- Make into proper cli which can accept flags to take different actions e.g. there are already functions defined for deleting all desktop entries, or forcing a complete refresh of all entries, which are currently unused
- Support more than just Steam games (depends how difficult this is to implement)

## Credits

The original idea belongs (as far as I know) to [@ntcarlson](https://github.com/ntcarlson), so big thanks to them. The original script I used to use for this is from [this Reddit post](https://www.reddit.com/r/unixporn/comments/p5b0qv/i3_misusing_rofi_as_a_game_launcher/) they shared.

However, I had modified it as I didn't like that sub menu for each game, and the script didn't work to update entries sometimes (as well as it didn't remove old entries for games I no longer had installed), so eventually I decided to have a go at making a Rust program out of it.

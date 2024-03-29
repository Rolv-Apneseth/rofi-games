# Themes

While it is not a requirement, it is strongly recommended to use one of the provided themes, or make your own theme which includes a decently large icon size to show off the box arts in all their glory.

To use the themes provided with `rofi-games`, just use the `rofi` theme flag: `-theme theme_name`. Included themes:

1. **Fullscreen (default)**

    Use with  `-theme games-default`

    ![default theme](https://github.com/Rolv-Apneseth/rofi-games/assets/69486699/1d15ef43-56fd-4be6-9bf3-0171c8f45b6a)

2. **Smaller**

    Use with `-theme games-smaller`

    ![smaller](https://github.com/Rolv-Apneseth/rofi-games/assets/69486699/25e74ae8-7376-45cd-a822-b91bebf6b70a)

## Customising (Recommended)

To customise further, import one of the base themes provided. For example, I personally have `~/.config/rofi/games.rasi`:

```rasi
@import "games-default"

textbox-prompt {
    str:                            "";
    font:                           "Inconsolata Nerd Font Mono 24";
    padding:                        0px 5px 0px 5px;
}
```

This adds in the search icon, which is not included by default since it requires a Nerd font.

---

Alternatively, instead of importing, you could also just copy the contents of `/usr/share/rofi/themes/games-default.rasi` and edit directly.

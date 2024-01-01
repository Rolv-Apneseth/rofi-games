alias b := build
alias i := install
alias u := uninstall
alias c := clean
alias t := test
alias tb := test-bare

# VARIABLES ----------------------------------------------------------------------------------------
LIB_NAME := "librofi_games.so"
PLUGIN_NAME := "games.so"

THEMES_DIR := "/usr/share/rofi/themes"
LICENSES_DIR := "/usr/share/licenses/" + PKGNAME

PLUGINS_DIR := `pkg-config --variable pluginsdir rofi || echo "/lib/rofi/"`
PLUGIN_PATH := PLUGINS_DIR + PLUGIN_NAME

# Set rust flags if running a version of `rofi` with changes newer than the base `1.7.5`
# See https://github.com/SabrinaJewson/rofi-mode.rs/issues/8#event-11112343153
# Examples of version outputs
#     rofi: Version: 1.7.5
#     rofi-git: Version: 1.7.5-187-gb43a82f8 (makepkg)
#     rofi-lbonn-wayland-git: Version: 1.7.5+wayland2-154-g36621af0 (makepkg)
RUSTFLAGS := if `rofi -version` == "Version: 1.7.5" { "" } else { "--cfg rofi_next" }

# For PKGBUILD
PKGNAME := env("PKGNAME", "rofi-games")
PKGDIR := env("PKGDIR", "")

# COMMANDS -----------------------------------------------------------------------------------------
build:
    RUSTFLAGS="{{RUSTFLAGS}}" cargo build --release --lib

install: build
    # Plugin
    install -DT "target/release/{{LIB_NAME}}" "{{PKGDIR}}{{PLUGIN_PATH}}"
    
    # Themes
    install -m=0644 -Dt "{{PKGDIR}}{{THEMES_DIR}}" themes/games-default.rasi
    install -m=0644 -Dt "{{PKGDIR}}{{THEMES_DIR}}" themes/games-smaller.rasi
    
    # License
    install -Dt "{{PKGDIR}}{{LICENSES_DIR}}" LICENSE
    
    cargo clean

uninstall:
    rm {{PLUGIN_PATH}}
    rm {{THEMES_DIR}}/games-default.rasi
    rm {{THEMES_DIR}}/games-smaller.rasi
    rm -rf {{LICENSES_DIR}}

clean:
    cargo clean --verbose

test $THEME:
    rofi -modi games -show games -theme $THEME

test-bare:
    rofi -modi games -show games -show-icons

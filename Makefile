PKGNAME := rofi-games
LIB_NAME := librofi_games.so
PLUGIN_NAME := games.so

CARGO ?= cargo
CARGO_TARGET_DIR ?= target
CARGO_RELEASE_DIR ?= $(CARGO_TARGET_DIR)/release
ROFI_VERSION := $(shell rofi -version)

licensesdir ?= /usr/share/licenses/$(PKGNAME)
themesdir ?= /usr/share/rofi/themes

# Find the directory to install rofi plugins
plugins_dir_pc = $(shell pkg-config --variable pluginsdir rofi)
plugins_dir ?= $(if $(plugins_dir_pc),$(plugins_dir_pc),lib/rofi)
plugin_path := "$(plugins_dir)$(PLUGIN_NAME)"

build:
	# Set rust flags if running a version of `rofi` with changes newer than the base `1.7.5`
	# See https://github.com/SabrinaJewson/rofi-mode.rs/issues/8#event-11112343153
	# Examples of version outputs
	#     rofi: Version: 1.7.5
	#     rofi-git: Version: 1.7.5-187-gb43a82f8 (makepkg)
	#     rofi-lbonn-wayland-git: Version: 1.7.5+wayland2-154-g36621af0 (makepkg)
	if [[ "Version: 1.7.5" != "${ROFI_VERSION}" ]]; then \
		RUSTFLAGS="--cfg rofi_next" cargo build --release --lib; \
	else \
	    cargo build --release --lib; \
	fi

install: build

	# Plugin
	install -DT "$(CARGO_RELEASE_DIR)/$(LIB_NAME)" "$(DESTDIR)$(plugin_path)"

	# Themes
	install -m=0644 -Dt $(DESTDIR)$(themesdir) themes/*

	# License
	install -Dt $(DESTDIR)$(licensesdir) LICENSE

	cargo clean

uninstall:
	rm ${plugin_path}
	rm -rf ${licensesdir}
	rm ${themesdir}/games-default.rasi
	rm ${themesdir}/games-smaller.rasi

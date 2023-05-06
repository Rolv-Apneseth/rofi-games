PKGNAME := rofi-games
LIB_NAME := librofi_games.so
PLUGIN_NAME := games.so

CARGO ?= cargo
CARGO_TARGET_DIR ?= target
CARGO_RELEASE_DIR ?= $(CARGO_TARGET_DIR)/release

datarootdir ?= usr/share
licensesdir ?= $(datarootdir)/licenses/$(PKGNAME)

# Find the directory to install rofi plugins
plugins_dir_pc = $(shell pkg-config --variable pluginsdir rofi)
plugins_dir ?= $(if $(plugins_dir_pc),$(plugins_dir_pc),lib/rofi)
plugin_path := "$(plugins_dir)/$(PLUGIN_NAME)"

install:
	cargo build --release --lib

	# Plugin
	install -DT "$(CARGO_RELEASE_DIR)/$(LIB_NAME)" "$(DESTDIR)$(plugin_path)"

	# License
	install -Dt $(DESTDIR)$(licensesdir) LICENSE

	cargo clean

uninstall:
	rm ${plugin_path}
	rm -rf ${licensesdir}

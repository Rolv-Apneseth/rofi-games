PKGNAME := rofi-games
LIB_NAME := librofi_games.so
PLUGIN_NAME := games.so
PLUGINS_DIR := /lib/rofi
PLUGIN_PATH := ${PLUGINS_DIR}/${PLUGIN_NAME}

CARGO ?= cargo
CARGO_TARGET_DIR ?= target
CARGO_RELEASE_DIR ?= $(CARGO_TARGET_DIR)/release

# Set DESTDIR for staged installs

datarootdir ?= usr/share
licensesdir ?= $(datarootdir)/licenses/$(PKGNAME)


install:
	cargo build --release --lib

	# Plugin
	install -DT "$(CARGO_RELEASE_DIR)/$(LIB_NAME)" "$(DESTDIR)$(PLUGIN_PATH)"

	# License
	install -Dt $(DESTDIR)$(licensesdir) LICENSE

	cargo clean

uninstall:
	rm ${PLUGIN_PATH}
	rm -rf ${licensesdir}

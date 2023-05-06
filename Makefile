LIB_NAME := librofi_games.so
PLUGIN_NAME := games.so
PLUGIN_PATH := /lib/rofi/${PLUGIN_NAME}

install:
	cargo build --release --lib
	cp ./target/release/${LIB_NAME} ${PLUGIN_PATH}
	cargo clean

uninstall:
	rm ${PLUGIN_PATH}

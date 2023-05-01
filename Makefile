bin_name := rofi-games
bin_path := ~/.local/bin/${bin_name}

install:
	cargo build --release
	cp ./target/release/${bin_name} ${bin_path}

uninstall:
	rm ${bin_path}

.PHONY: all build install uninstall

all: build

build:
	cargo build --release

install:
	install -Dm755 target/release/windot /usr/local/bin/windot
	desktop-file-install meta/windot.desktop
	install -Dm644 meta/icon.png /usr/share/icons/hicolor/64x64/apps/windot.png

uninstall:
	rm /usr/local/bin/windot
	rm /usr/share/icons/hicolor/64x64/apps/windot.png
	rm /usr/share/applications/windot.desktop

.PHONY: all build install uninstall

all: build install

build:
	cargo build --release

install:
	install -Dm755 target/release/windot /usr/bin/windot
	desktop-file-install ./windot.desktop
	install -Dm644 icon.png /usr/share/icons/hicolor/64x64/apps/windot.png

uninstall:
	rm /usr/bin/windot
	rm /usr/share/icons/hicolor/64x64/apps/windot.png
	rm /usr/share/applications/windot.desktop

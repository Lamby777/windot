.PHONY: all build install uninstall

prefix ?= /usr/local

all: build

build:
	cargo build --release

install:
	install -Dm755 target/release/windot $(DESTDIR)$(prefix)/bin/windot
	desktop-file-install --dir=$(DESTDIR)/usr/share/applications meta/org.sparklet.windot.desktop
	install -Dm644 meta/icon.png $(DESTDIR)/usr/share/icons/hicolor/64x64/apps/windot.png

uninstall:
	rm -f $(DESTDIR)$(prefix)/bin/windot
	rm -f $(DESTDIR)/usr/share/icons/hicolor/64x64/apps/windot.png
	rm -f $(DESTDIR)/usr/share/applications/org.sparklet.windot.desktop

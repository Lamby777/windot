.PHONY: all build install uninstall

prefix ?= /usr/local
bindir ?= $(prefix)/bin
datadir ?= /usr/share

all: build

build:
	cargo build --release

install:
	# Install the binary to the specified bin directory
	install -Dm755 target/release/windot $(DESTDIR)$(bindir)/windot
	
	# Install the desktop entry
	install -Dm644 meta/org.sparklet.windot.desktop $(DESTDIR)$(datadir)/applications/org.sparklet.windot.desktop
	
	# Install the icon
	install -Dm644 meta/icon.png $(DESTDIR)$(datadir)/icons/hicolor/48x48/apps/windot.png

uninstall:
	rm -f $(DESTDIR)$(bindir)/windot
	rm -f $(DESTDIR)$(datadir)/icons/hicolor/48x48/apps/windot.png
	rm -f $(DESTDIR)$(datadir)/applications/org.sparklet.windot.desktop

#!/bin/bash

# This script installs the desktop file and stuff so the app shows up in the menus
# and has a proper icon in the taskbar.

SCRIPT_PATH=$(dirname "$(readlink -f "$0")")

desktop-file-install "$SCRIPT_PATH/windot.desktop"

# if root
if [ "$EUID" -ne 0 ]
    then echo "Warning: unable to install icon as root. Please run this script as root. (or teach me how to do this properly lmfao)"
        exit
fi

cp "$SCRIPT_PATH/icon.png" /usr/share/icons/hicolor/64x64/apps/windot.png

echo "Done!"


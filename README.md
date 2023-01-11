# Wpilman
A simple clipboard manager for wayland.

## Installation
Currently, you shoud compile Wlipman yourself. But I'm working on an AUR package.

## Usage
1. Run `wl-paste -w path/to/wlipman store` on startup. It will save your clipboard state whenever you copy something.
2. Run `path/to/pick pick` to open a rofi dialog to restore a clipboard item from your history. You probably want to bind this to a keyboard shortcut.

## Features
1. Preserves clipboard history between logouts and shutdowns.
2. Preserves all mime types. This allows you to restore copied images and other rich content.

## Limitations
1. Only works with rofi
2. Copying multi-line strings cat mess up the rofi menu
2. Stores your clipboard history forever at `~/.cache/wlipman.msgpck`, unless manually cleared either by removing that file or by running `wlipman clean`. This file can grow pretty large over time.

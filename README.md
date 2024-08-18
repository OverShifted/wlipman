# Wpilman
A simple clipboard manager for wayland.

## Installation
Just compile it yourself or install the AUR package:
```sh
paru -S wlipman-git
# or
yay -S wlipman-git
```

## Usage
1. Run `wl-paste -w /bin/wlipman store` on startup. It will save your clipboard state whenever you copy something.
2. Run `/bin/wlipman pick` to open a rofi dialog to restore a clipboard item from your history. You probably want to bind this to a keyboard shortcut.

## Features
1. Preserves clipboard history between logouts and shutdowns.
2. Preserves all mime types. This allows you to restore copied images and other rich content.

## Limitations
1. Only works with rofi
2. Copying multi-line strings can mess up the rofi menu
2. Stores your clipboard history forever at `~/.cache/wlipman.msgpck`, unless manually cleared either by removing that file or by running `wlipman clean`. This file can grow pretty large over time.

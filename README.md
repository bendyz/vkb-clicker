# vkb-clicker

A minimal autoclicker for Linux/Wayland. It sends mouse clicks through a
virtual `/dev/uinput` device, so it works regardless of your compositor
(Wayland deliberately blocks apps from listening for global hotkeys, but it
does not block input injection via uinput).

Clicks land at the current cursor position — the app never moves the mouse,
it only generates button press/release events.

## Building

```sh
cargo build --release
```

The binary ends up at `target/release/vkb-clicker`. Copy it somewhere on
your `$PATH`, e.g.:

```sh
cp target/release/vkb-clicker ~/.local/bin/
```

## Usage

```sh
# start clicking: 20ms hold, 10ms pause (defaults)
vkb-clicker --click-ms 20 --pause-ms 10

# left/right/middle button
vkb-clicker --click-ms 20 --pause-ms 10 --button left

# stop the running instance
vkb-clicker --kill
```

The program enforces a single instance per user (PID file at
`$XDG_RUNTIME_DIR/vkb-clicker.pid`) — starting it again without `--kill`
while it's already clicking will refuse to run.

## Permissions for /dev/uinput

On some systems `/dev/uinput` is writable only by root. If you get
`Permission denied`, add a udev rule, e.g.
`/etc/udev/rules.d/60-uinput.rules`:

```
KERNEL=="uinput", MODE="0660", GROUP="input", TAG+="uaccess"
```

then add yourself to the `input` group (`sudo usermod -aG input $USER`) and
log back in.

## Binding to keyboard shortcuts

Wayland does not let applications capture global hotkeys, so the compositor
has to invoke the command for you. Bind two shortcuts — one to start, one to
stop:

**KDE Plasma** — System Settings -> Shortcuts -> Custom Shortcuts -> right
click "Custom" -> New -> Command/URL:
- "Start clicking": command `vkb-clicker --click-ms 20 --pause-ms 10`, e.g.
  bound to `Ctrl+Alt+F9`.
- "Stop clicking": command `vkb-clicker --kill`, e.g. bound to
  `Ctrl+Alt+F10`.

**GNOME** — Settings -> Keyboard -> Keyboard Shortcuts -> Custom Shortcuts,
same idea: one entry running `vkb-clicker --click-ms 20 --pause-ms 10`,
another running `vkb-clicker --kill`.

**Sway / Hyprland** — add to your config:

```
bindsym Ctrl+Alt+F9 exec vkb-clicker --click-ms 20 --pause-ms 10
bindsym Ctrl+Alt+F10 exec vkb-clicker --kill
```

(Hyprland uses `bind = ` instead of `bindsym`.)

Once bound, the first shortcut starts clicking wherever your cursor is
(e.g. over a button in a browser game), and the second stops it — it works
globally, regardless of which window has focus.

## Note

Automated clicking in browser games may violate that game's or service's
terms of use — that's on you as the user.

# vkb-clicker

A minimal autoclicker for Linux/Wayland. It sends mouse clicks through a
virtual `/dev/uinput` device, so it works regardless of your compositor
(Wayland deliberately blocks apps from listening for global hotkeys, but it
does not block input injection via uinput).

Clicks land at the current cursor position — the app never moves the mouse,
it only generates button press/release events.

## Installation

Requires the Rust toolchain (`rustc`/`cargo`) — install via
[rustup](https://rustup.rs) if you don't have it.

```sh
git clone <this-repo-url>
cd vkb-clicker
cargo install --path .
```

This builds a release binary and installs it to `~/.cargo/bin/vkb-clicker`,
which `cargo`/`rustup` already put on your `$PATH`. Run `vkb-clicker
--version` to confirm it's found.

To update later, `git pull` and re-run `cargo install --path .` (add
`--force` if cargo complains a binary with that name already exists).

### Manual build (no install)

```sh
cargo build --release
```

The binary ends up at `target/release/vkb-clicker`; copy it wherever you
like on your `$PATH`, e.g. `cp target/release/vkb-clicker ~/.local/bin/`.

## Usage

```sh
# start clicking: 20ms hold, 10ms pause (defaults)
vkb-clicker --click-ms 20 --pause-ms 10

# left/right/middle button
vkb-clicker --click-ms 20 --pause-ms 10 --button left

# stop the running instance
vkb-clicker --kill
```

The program tracks a single instance per user (PID file at
`$XDG_RUNTIME_DIR/vkb-clicker.pid`). If you run it again *without* `--kill`
while it's already clicking, it toggles off — it stops the running instance
instead of starting a second one. That means one shortcut, bound to the
plain start command, can act as both start and stop. `--kill` is still
there if you'd rather use a dedicated stop shortcut.

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
has to invoke the command for you. Since a plain launch toggles start/stop,
you only need **one** shortcut — though a two-shortcut setup (start /
`--kill`) works too if you prefer explicit control.

**KDE Plasma** — System Settings -> Shortcuts -> Custom Shortcuts -> right
click "Custom" -> New -> Command/URL. Name it e.g. "Toggle clicking",
command `vkb-clicker --click-ms 20 --pause-ms 10`, bind it to e.g.
`Ctrl+Alt+F9`.

**GNOME** — Settings -> Keyboard -> Keyboard Shortcuts -> Custom Shortcuts,
same idea: one entry running `vkb-clicker --click-ms 20 --pause-ms 10`.

**Sway / Hyprland** — add to your config:

```
bindsym Ctrl+Alt+F9 exec vkb-clicker --click-ms 20 --pause-ms 10
```

(Hyprland uses `bind = ` instead of `bindsym`.)

Once bound, pressing the shortcut starts clicking wherever your cursor is
(e.g. over a button in a browser game); pressing it again stops it — it
works globally, regardless of which window has focus.

If you'd rather have two separate shortcuts, bind a second one to
`vkb-clicker --kill` and use the first one only to start.

## Note

Automated clicking in browser games may violate that game's or service's
terms of use — that's on you as the user.

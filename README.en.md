# oatmeal

[日本語](README.md) | English

A small CLI that lets you pick a USB serial device **with a single command** and monitor it bidirectionally right away.

It exists to remove the chore of **looking up a port name and typing it by hand**, as in `screen /dev/tty.usbserial-XXXX 115200`.
Type `oatmeal`, and interactive menus list the available ports and baud rates — just pick, and the session starts.

## Features

- **Auto-enumerates** connected serial ports (`/dev/tty*`); USB devices also show manufacturer, product, and `VID:PID`
- **Pick a port** with the arrow keys (no manual typing)
- **Pick a baud rate** (common presets plus a custom-value entry)
- **Bidirectional serial monitor**: displays incoming data while forwarding your keystrokes to the device
- Quit with `Ctrl-C` / `Ctrl-]` (the terminal is automatically restored to cooked mode)

## What it looks like

```
$ oatmeal
? Select a serial port:
  > usbserial-1420  (Silicon Labs CP2102 USB to UART 10c4:ea60)
    usbmodem14201   (Arduino 2341:0043)
? Select a baud rate:
  > 115200
    9600
    ...
    Custom…
--- Connected: /dev/tty.usbserial-1420 @ 115200 (Ctrl-] / Ctrl-C to quit) ---
(bidirectional monitor from here: shows RX, sends your keystrokes)
--- Disconnected ---
```

> The list shows a short name with `/dev/tty` stripped, followed by manufacturer, product, and `VID:PID` in parentheses (whatever is available).

## Requirements

- Rust toolchain (`cargo`)
  - Homebrew: `brew install rust`
  - or rustup: <https://rustup.rs>
- macOS / Linux / Windows (both `serialport` and `crossterm` are cross-platform)

## Build & run

```bash
# run directly
cargo run

# install as a command (then launch with `oatmeal`)
cargo install --path .
```

## Usage

1. Run `oatmeal` (or `cargo run` inside the repo)
2. Select the serial port you want with the arrow keys
3. Select a baud rate (choose "Custom…" to enter an arbitrary value)
4. The bidirectional monitor starts: device output streams in, and your keystrokes are sent to the device
5. Quit with `Ctrl-]` or `Ctrl-C` (the terminal returns to normal mode)

If no `/dev/tty*` port is found, it prints
`No serial ports found. Check that your device is connected.`
and exits.

## Keys (while monitoring)

| Key | Action |
|-----|--------|
| Any character key | Send to the device |
| Other control chars | Passed through to the device (useful for interrupting a microcontroller, etc.) |
| `Ctrl-]` (0x1d) | Quit the monitor and return to the terminal |
| `Ctrl-C` (0x03) | Quit the monitor and return to the terminal |

> In raw mode `Ctrl-C` arrives as a byte rather than a signal, so it is treated as a quit key.

## Project layout

```
oatmeal/
├── Cargo.toml
└── src/
    ├── main.rs      # orchestrates the flow (enumerate → select → open → monitor)
    ├── ports.rs     # interactive port / baud-rate selection
    └── monitor.rs   # bidirectional serial monitor
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| [`serialport`](https://crates.io/crates/serialport) | Port enumeration and I/O |
| [`dialoguer`](https://crates.io/crates/dialoguer) | Interactive selection menus |
| [`crossterm`](https://crates.io/crates/crossterm) | Terminal raw-mode control |
| [`anyhow`](https://crates.io/crates/anyhow) | Error handling |

## Troubleshooting

- **Cannot open the port / Permission denied**
  On Linux, add your user to the `dialout` group (`sudo usermod -aG dialout $USER`, then log back in), or run with `sudo`.
- **`not a terminal` error**
  Because it uses interactive menus and terminal raw mode, it must run in a real terminal (it won't work over a pipe, in CI, or any non-TTY environment).
- **`cargo: command not found`**
  With a Homebrew install, `cargo` lives in `/usr/local/bin` (`/opt/homebrew/bin` on Apple Silicon). Check your PATH.

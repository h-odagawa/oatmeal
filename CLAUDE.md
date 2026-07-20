# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`oatmeal` is a small cross-platform CLI (Rust) that lists connected USB serial ports, lets the user pick one and a baud rate via interactive menus, then runs a bidirectional serial monitor. It replaces manually looking up a port name for `screen /dev/tty.usbserial-XXXX 115200`.

Note: user-facing strings (prompts, messages, error text shown to the user) are in English. Source comments, doc comments, and the README are in Japanese; keep new comments consistent with that.

## Commands

```bash
cargo run              # build + run the interactive CLI
cargo build            # debug build
cargo build --release  # optimized build
cargo install --path . # install as the `oatmeal` command
cargo fmt              # format
cargo clippy           # lint
```

There are no tests. The app **requires an interactive TTY** (dialoguer menus + terminal raw mode), so it cannot be driven from a pipe, CI, or a non-TTY tool shell — verify changes by running `cargo run` in a real terminal with a serial device attached.

## Architecture

Three modules, run as a linear pipeline from `main.rs`: enumerate → select → open → monitor.

- **`src/main.rs`** — orchestrates the flow. Selects a port (`None` → print "not found" and exit 0), selects a baud rate, opens the port with a short 50 ms read timeout, then hands the open port to `monitor::run`. The short timeout is deliberate: the monitor polls rather than blocking on reads.
- **`src/ports.rs`** — port/baud enumeration and interactive selection via `dialoguer::Select`/`Input`. `describe_port` formats USB ports with manufacturer/product when available. `COMMON_BAUD_RATES` (first entry is the default) plus a "custom input" menu item for arbitrary values.
- **`src/monitor.rs`** — the bidirectional monitor. Clones the port handle so read and write use separate handles. A background thread reads bytes → stdout (treating `TimedOut`/`WouldBlock` as "no data yet", sleeping 5 ms between polls); the main thread reads stdin one byte at a time in raw mode and writes to the port. Exit keys are `Ctrl-]` (`0x1d`, telnet-style) and `Ctrl-C` (`0x03`) — both leave the monitor cleanly (raw mode disables ISIG, so `Ctrl-C` arrives as a byte rather than SIGINT). Other control chars pass through to the device. `RawModeGuard` restores terminal cooked mode on Drop, including on panic — anything that enters raw mode must go through it so the terminal is never left broken.

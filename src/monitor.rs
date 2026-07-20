//! 双方向シリアルモニタ。
//!
//! - 受信スレッド: ポートから読んだバイトを標準出力へそのまま流す。
//! - 送信（メインスレッド）: raw mode の標準入力を1バイトずつポートへ送る。
//!   終了キー `Ctrl-]`(0x1d) または `Ctrl-C`(0x03) で抜ける。

use std::io::{self, Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use serialport::SerialPort;

/// モニタ終了キー: Ctrl-]（telnet の escape と同じ）。
const EXIT_KEY: u8 = 0x1d;

/// モニタ終了キー: Ctrl-C（正常終了）。
const CTRL_C: u8 = 0x03;

/// raw mode を確実に元へ戻すためのガード。パニック時も Drop で復帰する。
struct RawModeGuard;

impl RawModeGuard {
    fn enable() -> Result<Self> {
        enable_raw_mode().context("Failed to put the terminal into raw mode")?;
        Ok(RawModeGuard)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

/// 開いたポートで双方向モニタを実行する。`Ctrl-]` または `Ctrl-C` で戻る。
pub fn run(port: Box<dyn SerialPort>, port_name: &str, baud: u32) -> Result<()> {
    // 読み書きで別ハンドルを使う。
    let mut reader = port
        .try_clone()
        .context("Failed to clone the port (read handle)")?;
    let mut writer = port;

    println!(
        "--- Connected: {port_name} @ {baud} (Ctrl-] / Ctrl-C to quit) ---\r"
    );
    io::stdout().flush().ok();

    let stop = Arc::new(AtomicBool::new(false));

    // raw mode 有効化（Drop で自動復帰）。
    let _guard = RawModeGuard::enable()?;

    // 受信スレッド。
    let rx_stop = Arc::clone(&stop);
    let rx_handle = thread::spawn(move || {
        let mut buf = [0u8; 1024];
        let mut stdout = io::stdout();
        while !rx_stop.load(Ordering::Relaxed) {
            match reader.read(&mut buf) {
                Ok(0) => {}
                Ok(n) => {
                    if stdout.write_all(&buf[..n]).is_err() {
                        break;
                    }
                    let _ = stdout.flush();
                }
                Err(ref e)
                    if e.kind() == io::ErrorKind::TimedOut
                        || e.kind() == io::ErrorKind::WouldBlock =>
                {
                    // データ無し。少しだけ待って再ポーリング。
                    thread::sleep(Duration::from_millis(5));
                }
                Err(_) => break,
            }
        }
    });

    // 送信（メインスレッド）。stdin を1バイトずつ。
    let mut stdin = io::stdin();
    let mut byte = [0u8; 1];
    loop {
        match stdin.read(&mut byte) {
            Ok(0) => break, // EOF
            Ok(_) => {
                if byte[0] == EXIT_KEY || byte[0] == CTRL_C {
                    break;
                }
                if writer.write_all(&byte).is_err() {
                    break;
                }
                let _ = writer.flush();
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => break,
        }
    }

    // 後始末: 受信スレッドを止めて join。raw mode は _guard の Drop で復帰。
    stop.store(true, Ordering::Relaxed);
    let _ = rx_handle.join();

    // 復帰後に改行してプロンプトを整える。
    drop(_guard);
    println!("\r\n--- Disconnected ---");

    Ok(())
}

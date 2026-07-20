//! oatmeal — USBシリアルデバイスをコマンド一発で選んで双方向モニタするCLI。
//!
//! `oatmeal` と打つと、ポート一覧とボーレートが対話メニューで出て、
//! 選ぶだけで双方向シリアルモニタが始まる。

mod monitor;
mod ports;

use std::time::Duration;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    // 1. ポート選択（0件なら正常終了）。
    let port_name = match ports::select_port()? {
        Some(name) => name,
        None => {
            println!("No serial ports found. Check that your device is connected.");
            return Ok(());
        }
    };

    // 2. ボーレート選択。
    let baud = ports::select_baud_rate()?;

    // 3. 接続。read timeout は短め（モニタ側でポーリングするため）。
    let port = serialport::new(&port_name, baud)
        .timeout(Duration::from_millis(50))
        .open()
        .with_context(|| format!("Failed to open port {port_name} (@ {baud})"))?;

    // 4. 双方向モニタ開始。
    monitor::run(port, &port_name, baud)?;

    Ok(())
}

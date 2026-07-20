//! シリアルポートとボーレートの列挙・対話選択。

use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serialport::{SerialPortInfo, SerialPortType};

/// ボーレートの定番リスト（先頭がデフォルト選択）。
const COMMON_BAUD_RATES: &[u32] = &[
    115200, 9600, 19200, 38400, 57600, 230400, 460800, 921600,
];

/// `/dev/tty` 配下のポートだけを対象にするためのプレフィックス。
const TTY_PREFIX: &str = "/dev/tty";

/// `/dev/tty` を除いた表示名を返す。macOS の `/dev/tty.usbserial-…` の
/// ように続く `.` も落とし、`usbserial-…` のように見せる。
fn short_name(port_name: &str) -> &str {
    port_name
        .strip_prefix(TTY_PREFIX)
        .unwrap_or(port_name)
        .trim_start_matches('.')
}

/// USBシリアル情報を分かりやすいラベルに整形する。
///
/// 表示は `/dev/tty` を除いた短い名前を主とし、取れる範囲で
/// ベンダー（manufacturer / product）と VID:PID を括弧で添える。
fn describe_port(info: &SerialPortInfo) -> String {
    let name = short_name(&info.port_name);
    match &info.port_type {
        SerialPortType::UsbPort(usb) => {
            let mut parts: Vec<String> = Vec::new();
            if let Some(m) = &usb.manufacturer {
                if !m.is_empty() {
                    parts.push(m.clone());
                }
            }
            if let Some(p) = &usb.product {
                if !p.is_empty() {
                    parts.push(p.clone());
                }
            }
            // ベンダーID:プロダクトID（16進4桁）も添える。
            parts.push(format!("{:04x}:{:04x}", usb.vid, usb.pid));
            format!("{}  ({})", name, parts.join(" "))
        }
        _ => name.to_string(),
    }
}

/// 利用可能なポートを列挙して対話選択させる。
///
/// `/dev/tty` 配下のポートだけを対象にする。該当が無い場合は
/// `Ok(None)` を返す（呼び出し側で正常終了）。
pub fn select_port() -> Result<Option<String>> {
    let all = serialport::available_ports().context("Failed to enumerate serial ports")?;

    // `/dev/tty*` のみに絞る。
    let ports: Vec<SerialPortInfo> = all
        .into_iter()
        .filter(|p| p.port_name.starts_with(TTY_PREFIX))
        .collect();

    if ports.is_empty() {
        return Ok(None);
    }

    let labels: Vec<String> = ports.iter().map(describe_port).collect();

    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a serial port")
        .items(&labels)
        .default(0)
        .interact()
        .context("Port selection was cancelled")?;

    Ok(Some(ports[idx].port_name.clone()))
}

/// ボーレートを対話選択させる。末尾の「カスタム入力」で任意値も指定可能。
pub fn select_baud_rate() -> Result<u32> {
    let mut items: Vec<String> = COMMON_BAUD_RATES.iter().map(|b| b.to_string()).collect();
    let custom_idx = items.len();
    items.push("Custom…".to_string());

    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a baud rate")
        .items(&items)
        .default(0)
        .interact()
        .context("Baud rate selection was cancelled")?;

    if idx == custom_idx {
        let baud: u32 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter a baud rate")
            .interact_text()
            .context("Baud rate input was cancelled")?;
        Ok(baud)
    } else {
        Ok(COMMON_BAUD_RATES[idx])
    }
}

# oatmeal

USBシリアルデバイスを **コマンド一発** で選んで、そのまま双方向モニタするCLIツール。

`screen /dev/tty.usbserial-XXXX 115200` のように **ポート名を自分で調べて手入力する** 手間をなくすのが目的。
`oatmeal` と打つと、ポート一覧とボーレートが対話メニューで出て、選ぶだけで通信が始まる。

## 特徴

- 接続中のシリアルポートを **自動列挙**（USB機器はメーカー名・製品名も表示）
- 矢印キーで **ポートを選択**（手入力不要）
- **ボーレートを選択**（定番リスト＋任意値のカスタム入力）
- **双方向シリアルモニタ**：デバイスからの受信を表示しつつ、キー入力をそのまま送信
- `Ctrl-C` などの制御文字もデバイスへ送信（マイコンへの割り込みに使える）
- 終了は `Ctrl-]`（telnet風のエスケープキー）

## 動作イメージ

```
$ oatmeal
? シリアルポートを選択:
  > /dev/tty.usbserial-1420  (Silicon Labs CP2102 USB to UART)
    /dev/tty.usbmodem14201   (Arduino)
? ボーレートを選択:
  > 115200
    9600
    ...
    カスタム入力…
--- Connected: /dev/tty.usbserial-1420 @ 115200 (Ctrl-] で終了) ---
（ここから双方向モニタ：受信を表示、キー入力を送信）
```

## 必要環境

- Rust ツールチェイン（`cargo`）
  - Homebrew: `brew install rust`
  - もしくは rustup: <https://rustup.rs>
- macOS / Linux / Windows（`serialport` / `crossterm` がクロスプラットフォーム対応）

## ビルドと実行

```bash
# そのまま実行
cargo run

# コマンドとしてインストール（以後 `oatmeal` で起動）
cargo install --path .
```

## 使い方

1. `oatmeal`（またはリポジトリ内で `cargo run`）を実行する
2. 一覧から接続したいシリアルポートを矢印キーで選ぶ
3. ボーレートを選ぶ（一覧に無ければ「カスタム入力…」で任意値を指定）
4. 双方向モニタが始まる。デバイスの出力が流れ、キー入力がデバイスへ送られる
5. `Ctrl-]` で終了（端末は通常モードに戻る）

ポートが1件も見つからない場合は
`シリアルポートが見つかりません。デバイスの接続を確認してください。`
と表示して終了する。

## キー操作（モニタ中）

| キー | 動作 |
|------|------|
| 任意の文字キー | デバイスへ送信 |
| `Ctrl-C` (0x03) | そのままデバイスへ送信（マイコンの割り込みなどに使う） |
| `Ctrl-]` (0x1d) | モニタを終了して端末に戻る |

## プロジェクト構成

```
oatmeal/
├── Cargo.toml
└── src/
    ├── main.rs      # フロー統括（列挙 → 選択 → 接続 → モニタ）
    ├── ports.rs     # ポート／ボーレートの対話選択
    └── monitor.rs   # 双方向シリアルモニタ
```

## 依存クレート

| クレート | 用途 |
|----------|------|
| [`serialport`](https://crates.io/crates/serialport) | ポート列挙・入出力 |
| [`dialoguer`](https://crates.io/crates/dialoguer) | 対話選択メニュー |
| [`crossterm`](https://crates.io/crates/crossterm) | 端末の raw mode 制御 |
| [`anyhow`](https://crates.io/crates/anyhow) | エラー処理 |

## トラブルシューティング

- **ポートを開けない / Permission denied**
  Linux ではユーザーを `dialout` グループに追加する（`sudo usermod -aG dialout $USER` 後に再ログイン）か、`sudo` で実行する。
- **`not a terminal` エラー**
  対話メニューを使うため、実際のターミナルで実行する必要がある（パイプやCIなど非TTY環境では動かない）。
- **`cargo: command not found`**
  Homebrew 導入時は `cargo` が `/usr/local/bin`（Apple Silicon は `/opt/homebrew/bin`）にある。PATH を確認する。

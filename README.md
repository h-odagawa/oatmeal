# oatmeal

日本語 | [English](README.en.md)

USBシリアルデバイスを **コマンド一発** で選んで、そのまま双方向モニタするCLIツール。

`screen /dev/tty.usbserial-XXXX 115200` のように **ポート名を自分で調べて手入力する** 手間をなくすのが目的。
`oatmeal` と打つと、ポート一覧とボーレートが対話メニューで出て、選ぶだけで通信が始まる。

## 特徴

- 接続中のシリアルポート（`/dev/tty*`）を **自動列挙**（USB機器はメーカー名・製品名・`VID:PID` も表示）
- 矢印キーで **ポートを選択**（手入力不要）
- **ボーレートを選択**（定番リスト＋任意値のカスタム入力）
- **双方向シリアルモニタ**：デバイスからの受信を表示しつつ、キー入力をそのまま送信
- `Ctrl-C` / `Ctrl-]` で終了（端末は自動で通常モードに戻る）

## 動作イメージ

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
（ここから双方向モニタ：受信を表示、キー入力を送信）
--- Disconnected ---
```

> 表示は `/dev/tty` を落とした短い名前で、括弧内にメーカー名・製品名・`VID:PID`（取得できた範囲）を添える。

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

## ビルド済みバイナリを作る

配布用に、リリースビルドを `tar.gz` にまとめて SHA256 を添える手順。
`dist/` は `.gitignore` 済みなので、成果物はリポジトリには含めない。

```bash
# リリースビルド（バイナリは target/release/oatmeal）
cargo build --release

# 配布ファイル名に使うバージョンとターゲットを取得
VERSION="v$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)"   # 例: v0.1.0
TARGET="$(rustc -vV | sed -n 's/host: //p')"                          # 例: x86_64-apple-darwin
NAME="oatmeal-$VERSION-$TARGET"

# tar.gz にまとめて SHA256 チェックサムを生成
mkdir -p dist
tar czf "dist/$NAME.tar.gz" -C target/release oatmeal
shasum -a 256 "dist/$NAME.tar.gz" > "dist/$NAME.tar.gz.sha256"
# Linux では shasum の代わりに `sha256sum` を使う
```

生成物（例）:

```
dist/oatmeal-v0.1.0-x86_64-apple-darwin.tar.gz
dist/oatmeal-v0.1.0-x86_64-apple-darwin.tar.gz.sha256
```

チェックサムの検証（リポジトリのルートで実行）:

```bash
shasum -a 256 -c dist/oatmeal-v0.1.0-x86_64-apple-darwin.tar.gz.sha256
```

配布先での使い方（受け取った側）:

```bash
tar xzf oatmeal-v0.1.0-x86_64-apple-darwin.tar.gz
./oatmeal
```

> 別プラットフォーム向けにクロスビルドする場合は、対象ターゲットを追加してから
> `--target` を付けてビルドする。バイナリは `target/<triple>/release/oatmeal` に出る。
>
> ```bash
> rustup target add aarch64-apple-darwin
> cargo build --release --target aarch64-apple-darwin
> ```

## 使い方

1. `oatmeal`（またはリポジトリ内で `cargo run`）を実行する
2. 一覧から接続したいシリアルポートを矢印キーで選ぶ
3. ボーレートを選ぶ（一覧に無ければ「Custom…」で任意値を指定）
4. 双方向モニタが始まる。デバイスの出力が流れ、キー入力がデバイスへ送られる
5. `Ctrl-]` または `Ctrl-C` で終了（端末は通常モードに戻る）

`/dev/tty*` のポートが1件も見つからない場合は
`No serial ports found. Check that your device is connected.`
と表示して終了する。

## キー操作（モニタ中）

| キー | 動作 |
|------|------|
| 任意の文字キー | デバイスへ送信 |
| その他の制御文字 | そのままデバイスへ送信（マイコンへの割り込みなどに使える） |
| `Ctrl-]` (0x1d) | モニタを終了して端末に戻る |
| `Ctrl-C` (0x03) | モニタを終了して端末に戻る |

> raw mode 中は `Ctrl-C` がシグナルにならずバイトとして届くため、終了キーとして扱う。

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
  対話メニューと端末の raw mode を使うため、実際のターミナルで実行する必要がある（パイプやCIなど非TTY環境では動かない）。
- **`cargo: command not found`**
  Homebrew 導入時は `cargo` が `/usr/local/bin`（Apple Silicon は `/opt/homebrew/bin`）にある。PATH を確認する。

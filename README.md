# vrcwatch.rs (experimental) 🚧

VRChat向けにOSCメッセージを送信する[vrcwatch](https://github.com/mezum/vrcwatch)のRust実装です。

**⚠このプロジェクトは現在、実験的な状態です。⚠**

今後、破壊的変更が発生する可能性がありますのでご注意ください。
フィードバックやバグ報告は大歓迎です🎉

## ビルド

```bash
git clone https://github.com/Quesys-tech/vrcwatch.rs.git
cargo build --release
```

## コマンドラインオプション

以下のコマンドラインオプションをサポートしています。基本的にデフォルトで動きます。

-  `-a`, `--address <ADDRESS>` 送信先のIPアドレス (デフォルト: `127.0.0.1`)
-  `-p`, `--port <PORT>`       送信先のポート (デフォルト: `9000`)
-  `-v`, `--verbose`           詳細な出力
-  `-h`, `--help`              ヘルプ
-  `-V`, `--version`           バージョンの表示（後日実装予定）

実行例を以下に示します。IPアドレスやポートは環境に合わせて変えてください。
```bash
./target/release/vrcwatch-rs.exe -a 192.0.2.1 -p 9876
```

## 出力するOSCのパラメータ

以下のパラメータを出力します。これを使うことで、VRChatのアバターに現在時刻を表示することが可能になります。

| OSCアドレス                           | データ型 | 説明                                                                 |
| ------------------------------------- | -------- | -------------------------------------------------------------------- |
| `/avatar/parameters/DateTimeSecondFA` | float32  | 現在時刻の秒を60で割ったもの                                         |
| `/avatar/parameters/DateTimeMinuteFA` | float32  | 現在時刻の分を60で割ったものに、`DateTimeSecondFA`の1/60を足したもの |
| `/avatar/parameters/DateTimeHourFA`   | float32  | 現在時刻の時を60で割ったものに、`DateTimeMinuteFA`の1/24を足したもの |

### 使い方

VRChatでアバターに時計を表示したい場合、以下のようにアバターのパラメータを設定できます：

- `DateTimeSecondFA` を使って秒針の回転を制御
- `DateTimeMinuteFA` を使って分針の回転を制御
- `DateTimeHourFA` を使って時針の回転を制御



## ライセンス

MITライセンス
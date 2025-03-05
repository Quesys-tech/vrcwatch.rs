# vrcwatch.rs

[vrcwatch](https://github.com/mezum/vrcwatch) のRust実装です。

## コマンドラインオプション
-  `-a`, `--address <ADDRESS>` 送信先のIPアドレス (デフォルト: `127.0.0.1`)
-  `-p`, `--port <PORT>`       送信先のポート (デフォルト: `9000`)
-  `-v`, `--verbose`           詳細な出力
-  `-h`, `--help`              ヘルプ
-  `-V`, `--version`           バージョンの表示（未実装）

## 出力するOSCのパラメータ

以下のパラメータを出力します。

| OSCアドレス | データ型 | 説明 |
|-------------|-----------|-------------|
|`/avatar/parameters/DateTimeSecondFA` | float32  | 現在時刻の秒を60で割ったもの |
|`/avatar/parameters/DateTimeMinuteFA` | float32  | 現在時刻の分を60で割ったものに、`DateTimeSecondFA`の1/60を足したもの |
|`/avatar/parameters/DateTimeHourFA` | float32  | 現在時刻の時を60で割ったものに、`DateTimeMinuteFA`の1/60を足したもの |

# howdy-notifier

Howdy（顔認証）の状態変化を通知する Rust ツール。

## 設定

起動前に次の環境変数で監視対象と通知方法を指定する。

```bash
export HOWDY_CAMERA_DEVICE=/dev/video2
export HOWDY_NOTIFY_SOUND=/usr/share/sounds/alsa/Front_Center.wav
export HOWDY_SUDO_USER=<notification-user>
```

`HOWDY_SUDO_USER` が未設定の場合は音声通知をスキップする。

## ライセンス

`AGPL-3.0-or-later`

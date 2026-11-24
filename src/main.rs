use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::process::Command;
use std::path::Path;

/// 監視対象と通知音の再生ユーザーは環境変数で指定する。
///   HOWDY_CAMERA_DEVICE : 実体を直接監視するカメラデバイス
///   HOWDY_NOTIFY_SOUND  : 再生する音声ファイル
///   HOWDY_SUDO_USER     : paplay を実行するユーザー (未設定なら音声通知をスキップ)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device_path = env::var("HOWDY_CAMERA_DEVICE")
        .unwrap_or_else(|_| "/dev/video2".to_string());
    let sound_path = env::var("HOWDY_NOTIFY_SOUND")
        .unwrap_or_else(|_| "/usr/share/sounds/alsa/Front_Center.wav".to_string());
    let sudo_user = env::var("HOWDY_SUDO_USER").unwrap_or_default();

    println!("Monitoring: {}", device_path);

    let mut watcher = RecommendedWatcher::new(
        move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                // カメラデバイスは open されるとアクセスされる
                if event.kind.is_access() {
                    println!("Camera accessed!");
                    if sudo_user.is_empty() {
                        eprintln!("HOWDY_SUDO_USER is not set; skipping sound notification");
                        return;
                    }
                    let _ = Command::new("sudo")
                        .arg("-u")
                        .arg(&sudo_user)
                        .arg("paplay")
                        .arg(&sound_path)
                        .spawn();
                }
            }
        },
        Config::default(),
    )?;

    watcher.watch(Path::new(device_path), RecursiveMode::NonRecursive)?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

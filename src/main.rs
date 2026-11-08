use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::process::Command;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // シンボリックリンクではなく実体を直接監視
    let device_path = "/dev/video2";
    let sound_path = "/usr/share/sounds/alsa/Front_Center.wav";

    println!("Monitoring: {}", device_path);

    let mut watcher = RecommendedWatcher::new(
        move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                // カメラデバイスは open されるとアクセスされる
                if event.kind.is_access() {
                    println!("Camera accessed!");
                    let _ = Command::new("sudo")
                        .arg("-u")
                        .arg("koppe")
                        .arg("paplay")
                        .arg(sound_path)
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

// 260601 播放 WAV 檔與即時音量控制
use rodio::{Decoder, Player};
use std::fs::File;

fn main() {
    let handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("Error to open default audio stream");
    let file = File::open("test.wav").unwrap();
    let player = Player::connect_new(handle.mixer());
    let source = Decoder::try_from(file).unwrap();

    // handle.mixer().add(source);
    player.append(source);

    std::thread::sleep(std::time::Duration::from_secs(2));

    player.set_volume(0.5);
    std::thread::sleep(std::time::Duration::from_secs(2));

    player.set_volume(1.5);
    std::thread::sleep(std::time::Duration::from_secs(2));
}

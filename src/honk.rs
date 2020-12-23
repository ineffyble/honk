extern crate rand;

use std::io::Cursor;
use rand::Rng;

#[derive(RustEmbed)]
#[folder = "honks/"]
struct Honk;

pub fn honk() {
    let mut rng = rand::thread_rng();
    let honk_number = rng.gen_range(0..3) + 1;
    let honk_file = format!("{}.mp3", honk_number.to_string());
    play_sound(honk_file);
}

fn play_sound(sound_name: String) {
    let honk = Honk::get(&sound_name).unwrap();
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let sink = rodio::OutputStreamHandle::play_once(&stream_handle, Cursor::new(honk));
    sink.unwrap().sleep_until_end();
}

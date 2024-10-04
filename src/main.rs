extern crate timer;
extern crate chrono;

use std::fs::File;
use std::io::BufReader;
use std::thread;

use rodio::{Decoder, OutputStream, source::Source};
use chrono::{Utc, Duration};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let timer = timer::Timer::new();

    let _guard = timer.schedule(
        Utc::now() + Duration::seconds(0),
        Some(Duration::minutes(20)),
        move || {
            let dt = Utc::now();
            println!("{}: Play sound", dt.format("%Y-%m-%d %H:%M:%S"));
            let file = BufReader::new(File::open("assets/wrist-watch-alarm.mp3").unwrap());
            let source = Decoder::new(file).unwrap();
            let _ = stream_handle.play_raw(source.convert_samples());
        },
    );

    loop {
        thread::sleep(std::time::Duration::new(1, 0));
    }
}

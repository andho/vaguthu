extern crate chrono;
extern crate timer;

use std::io;
use std::io::Read;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;

use chrono::{Duration, Local, NaiveTime};
use notify_rust::Notification;
use rodio::Sink;
use rodio::{source::Source, OutputStream};

struct Schedule {
    start: String,
    end: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sound = Sound::load("assets/wrist-watch-alarm.mp3")?;
    let sink = Sink::try_new(&stream_handle).unwrap();

    let schedule = Schedule {
        start: "07:30".to_string(),
        end: "22:00".to_string(),
    };
    let start_time = NaiveTime::from_str(&schedule.start)?;
    let end_time = NaiveTime::from_str(&schedule.end)?;

    let timer = timer::Timer::new();

    let _guard = timer.schedule(
        Local::now() + Duration::seconds(0),
        Some(Duration::minutes(20)),
        move || {
            let dt = Local::now();
            let start = dt.clone().with_time(start_time).unwrap();
            let end = dt.clone().with_time(end_time).unwrap();

            if dt.cmp(&start) == std::cmp::Ordering::Less {
                println!(
                    "{}: Too early to play sound",
                    dt.format("%Y-%m-%d %H:%M:%S")
                );
                return;
            }

            if dt.cmp(&end) == std::cmp::Ordering::Greater {
                println!("{}: Too late to play sound", dt.format("%Y-%m-%d %H:%M:%S"));
                return;
            }

            let dt_string = dt.format("%Y-%m-%d %H:%M:%S");
            println!("{}: Play sound", dt_string);

            sink.append(sound.decoder().convert_samples::<f32>());
            sink.append(sound.decoder().convert_samples::<f32>());

            let Ok(_) = Notification::new()
                .appname("Vaguthu")
                .summary(&format!("Time for rest: {}", dt_string))
                .timeout(300000)
                .show()
            else {
                println!("Unable to send notification");
                return;
            };

            sink.sleep_until_end();
        },
    );

    loop {
        thread::sleep(std::time::Duration::new(60, 0));
    }
}

pub struct Sound(Arc<Vec<u8>>);

impl AsRef<[u8]> for Sound {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Sound {
    pub fn load(filename: &str) -> io::Result<Sound> {
        use std::fs::File;
        let mut buf = Vec::new();
        let mut file = File::open(filename)?;
        file.read_to_end(&mut buf)?;
        Ok(Sound(Arc::new(buf)))
    }
    pub fn cursor(&self) -> io::Cursor<Sound> {
        io::Cursor::new(Sound(self.0.clone()))
    }
    pub fn decoder(&self) -> rodio::Decoder<io::Cursor<Sound>> {
        rodio::Decoder::new(self.cursor()).unwrap()
    }
}

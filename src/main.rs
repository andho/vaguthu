extern crate chrono;
extern crate timer;

use std::io::BufReader;
use std::thread;
use std::{fs::File, str::FromStr};

use chrono::{Duration, Local, NaiveTime};
use notify_rust::Notification;
use rodio::{source::Source, Decoder, OutputStream};

struct Schedule {
    start: String,
    end: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

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
            let file = BufReader::new(File::open("assets/wrist-watch-alarm.mp3").unwrap());
            let source = Decoder::new(file).unwrap();
            let _ = stream_handle.play_raw(source.convert_samples());

            let Ok(_) = Notification::new()
                .appname("Vaguthu")
                .summary(&format!("Time for rest: {}", dt_string))
                .timeout(300000)
                .show()
            else {
                println!("Unable to send notification");
                return;
            };
        },
    );

    loop {
        thread::sleep(std::time::Duration::new(60, 0));
    }
}

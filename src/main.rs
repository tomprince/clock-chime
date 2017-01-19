extern crate clap;
extern crate chrono;
extern crate rodio;
#[macro_use]
extern crate lazy_static;

use chrono::Timelike;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

lazy_static!{
    static ref DATA: &'static Path = &Path::new("/usr/share/sounds/freedesktop/stereo/");
}

fn main() {
    let now = chrono::Local::now();

    let hour = now + chrono::Duration::minutes(30);
    let hour = hour.with_minute(0).unwrap();

    let file = format!("grandfather-clock-chime-{}", hour.format("%I"));
    let path = DATA.join(PathBuf::from(file).with_extension("wav"));
    play_file(path);
}

fn play_file<P: AsRef<Path>>(file: P) {
    let endpoint = rodio::get_default_endpoint().unwrap();
    let sink = rodio::Sink::new(&endpoint);

    let file = File::open(file).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();



    sink.append(source);
    sink.sleep_until_end();
}

extern crate clap;
extern crate chrono;
extern crate rodio;
#[macro_use]
extern crate lazy_static;

use chrono::Timelike;
use std::path::{Path, PathBuf};

lazy_static!{
    static ref DATA: &'static Path = &Path::new("/usr/share/sounds/freedesktop/stereo/");
}

mod audio;
use audio::play_file;

fn main() {
    let now = chrono::Local::now();

    let hour = now + chrono::Duration::minutes(30);
    let hour = hour.with_minute(0).unwrap();

    let file = match now.minute() {
        0...7 | 53...60 => format!("grandfather-clock-chime-{}", hour.format("%I")),
        8...22 => "grandfather-clock-chime-one-quarter".into(),
        23...37 => "grandfather-clock-chime-one-half".into(),
        38...52 => "grandfather-clock-chime-three-quarter".into(),
        _ => panic!{"Minute outside range."},
    };

    let path = DATA.join(PathBuf::from(file).with_extension("wav"));
    play_file(path);
}

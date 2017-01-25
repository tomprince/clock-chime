#[macro_use]
extern crate clap;
extern crate chrono;
extern crate rodio;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;

use chrono::Timelike;
use std::path::{Path, PathBuf};

lazy_static!{
    static ref DATA: &'static Path = &Path::new("/usr/share/sounds/freedesktop/stereo/");
}

mod error {
    error_chain!{
        errors {
            // FIXME: https://github.com/tomaka/rodio/issues/86
            Decoder(t: ::rodio::decoder::DecoderError)
        }
    }
}
use error::*;


mod audio;
use audio::play_file;

fn run() -> Result<()> {
    use clap::{App, Arg};
    let matches = App::new("Clock Chimes")
        .version(crate_version!{})
        .author(crate_authors!{})
        .about("Plays clock chimes.")
        .arg(Arg::with_name("volume")
            .short("v")
            .long("volume")
            .value_name("VOLUME")
            .help("Sets the volume")
            .takes_value(true))
        .get_matches();

    let volume = value_t!(matches, "config", f32).unwrap_or(0.4);

    play(volume)
}

fn play(volume: f32) -> Result<()> {
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
    play_file(path, volume)
}

quick_main!(run);

extern crate rodio;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn play_file<P: AsRef<Path>>(file: P) {
    let endpoint = rodio::get_default_endpoint().unwrap();
    let sink = rodio::Sink::new(&endpoint);

    let file = File::open(file).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();



    sink.append(source);
    sink.sleep_until_end();
}

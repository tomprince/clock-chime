use error::*;
use rodio;
use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn play_file<P: AsRef<Path>>(filename: P, volume: f32) -> Result<()> {
    let endpoint = rodio::get_default_endpoint().ok_or("No default audio endpoint.")?;
    let sink = rodio::Sink::new(&endpoint);

    let file = File::open(filename).chain_err(|| "Couldn't open file.")?;
    // FIXME: https://github.com/tomaka/rodio/issues/86
    let source = rodio::Decoder::new(BufReader::new(file)).map_err(ErrorKind::Decoder)?;

    sink.append(source.amplify(db_to_ratio(volume)));
    sink.sleep_until_end();

    Ok(())
}

fn db_to_ratio(db: f32) -> f32 {
    lazy_static!{ static ref EXP: f32 = (10_f32).log(2_f32); }
    db.powf(*EXP)
}

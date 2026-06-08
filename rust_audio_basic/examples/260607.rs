use std::fs::File;
use symphonia::core::errors::Error;
use symphonia::core::formats::TrackType;
use symphonia::core::io::MediaSourceStream;

fn main() {
    let file = File::open("test.wav").expect("Fail to read file");
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut format = symphonia::default::get_probe()
        .probe(
            &Default::default(),
            mss,
            Default::default(),
            Default::default(),
        )
        .expect("Fail to read format");

    let track = format
        .default_track(TrackType::Audio)
        .expect("No audio track");

    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(
            track.codec_params.as_ref().unwrap().audio().unwrap(),
            &Default::default(),
        )
        .expect("Unsupported codec");

    let trackid = track.id;

    let mut sig: Vec<f32> = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(Some(packet)) => packet,
            // end
            Ok(None) => {
                break;
            }
            // re-examined
            Err(Error::ResetRequired) => {
                unimplemented!();
            }
            // other err
            Err(err) => {
                panic!("{}", err);
            }
        };

        // filter non-audio track packet
        if packet.track_id != trackid {
            continue;
        }

        let decoded = match decoder.decode(&packet) {
            Ok(decoded) => decoded,
            Err(Error::DecodeError(_)) => {
                continue;
            }
            Err(Error::IoError(_)) => {
                continue;
            }
            Err(err) => {
                panic!("{}", err)
            }
        };

        decoded.copy_to_vec_interleaved(&mut sig);
    }
    println!("{}", sig.len());
}

use std::fs::{File, Metadata};
use symphonia::core::errors::Error;
use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::formats::{FormatOptions, TrackType};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

fn main() {
    let path = "test.wav";
    let src = File::open(path).expect("faild to open media");
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let fmt_opts: FormatOptions = Default::default();
    let meta_opts: MetadataOptions = Default::default();

    let mut format = symphonia::default::get_probe()
        .probe(&Default::default(), mss, fmt_opts, meta_opts)
        .expect("unsupported format");

    let track = format
        .default_track(TrackType::Audio)
        .expect("no audio track");
    let dec_opts: AudioDecoderOptions = Default::default();

    let mut decoder = symphonia::default::get_codecs().make_audio_decoder(
        track
            .codec_params
            .as_ref()
            .expect("codec parameter missing")
            .audio()
            .unwrap(),
        &dec_opts,
    ).expect("decoder error");

    let track_id = track.id;

    let mut out: Vec<f32> = Vec::new();
    
    loop {
        let packet = match format.next_packet() {
            Ok(Some(packet)) => packet,
            Ok(None) => {
                break;
            },
            Err(Error::ResetRequired) => {
                unimplemented!();
            },
            Err(err) => {
                panic!("{}", err);
            }
        };

        if packet.track_id != track_id { continue }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                decoded.copy_to_vec_interleaved(&mut out); 
            }
            Err(Error::IoError(_)) => {
                continue;
            }
            Err(Error::DecodeError(_)) => {
                continue;
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    println!("{}", out.len());
}

use core::panic;

// 260602 Sine Wave 產生器
use cpal::{SampleFormat, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No availible device");
    let supported_config = device
        .default_output_config()
        .expect("Error while querying configs");

    let sample_format = supported_config.sample_format();
    // channels, sample_rate, buffer_size
    let config: StreamConfig = supported_config.into();
    // dbg!("{:?}", &config);

    let err_fn = |err| eprintln!("Error: {err}");

    let sample_rate = config.sample_rate as f32;
    let freq = (440.0, 659.25);

    let stream = match sample_format {
        SampleFormat::F32 => {
            let mut clock = 0f32;

            device.build_output_stream(
                &config,
                move | data: &mut [f32], _: &cpal::OutputCallbackInfo | {
                    for sample in data.iter_mut() {
                        clock = (clock + 1.0) % sample_rate;
                        let t = clock / sample_rate;
                        *sample = ( (2.0 * std::f32::consts::PI * freq.0 * t).sin() + (2.0 * std::f32::consts::PI * freq.1 * t).sin() ) * 0.5;
                    }
                },
                err_fn,
                None
                )
            },
            other_format => panic!("Unsupported sample format device: {}", other_format),
    }.unwrap();

    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));
}

// fn sine_gen<T>(data: &mut [T], _: &cpal::OutputCallbackInfo)
// where
//     T: Sample + FromSample<f32>
// {
//     let sample_rate = 48000.0;
//     let freq = 440.0;
//
//     for (idx, sample) in data.iter_mut().enumerate() {
//         let t = idx as f32 / sample_rate;
//         let sig: T = T::from_sample((2.0 * std::f32::consts::PI * freq * t).sin());
//
//         *sample = sig;
//     }
// }

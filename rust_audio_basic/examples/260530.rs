// 260530 音訊 Clipping 與 Peak 運算
fn clipping(samples: &[f32]) -> Vec<f32> {
    samples.iter().map(|&v| { v.clamp(-1.0, 1.0) }).collect::<Vec<f32>>()
}

fn peak_finding(samples: &[f32]) -> f32 {
    samples.iter().fold(0.0, | acc, &x | x.abs().max(acc))
}

fn main() {
    let samples: Vec<f32> = vec![0.5, 1.5, -2.0, -0.8, 0.1];
    let samples_clipping = clipping(&samples);
    let samples_peak = peak_finding(&samples_clipping);

    println!("original samples: {:?}", samples);
    println!("samples_clipping: {:?}", samples_clipping);
    println!("samples_peak: {:?}", samples_peak);
}

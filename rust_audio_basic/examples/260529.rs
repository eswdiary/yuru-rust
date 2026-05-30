// 260530 簡易音訊數據讀取器
use std::fs;

fn load_audio_samples(path: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let file = fs::read_to_string(path)?;
    let mut r: Vec<f32> = vec![];

    for line in file.lines() {
        r.push(line.parse::<f32>()?);
    }
    Ok(r)
}

fn main() {
    let audio_samples = load_audio_samples("samples.txt");

    dbg!(&audio_samples);

    match audio_samples {
        Ok(v) => println!("vec: {:?}, size: {}", v, v.len()),
        Err(e) => eprintln!("error: {}", e),
    }
}

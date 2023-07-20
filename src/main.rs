extern crate sdl2;

use sdl2::audio::{AudioCVT, AudioCallback, AudioSpecDesired, AudioSpecWAV};
use std::path::Path;
use std::time::Duration;
struct Sound {
    data: Vec<f32>,
    volume: f32,
    pos: f32,
    period: f32,
}

impl AudioCallback for Sound {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for dst in out.iter_mut() {
            let intpos = self.pos.floor() as usize;
            let mut nextpos = intpos + 1;
            if nextpos > self.data.len() {
                nextpos = 0
            }
            let next_fac = self.pos - self.pos.floor();
            let inv_fac = 1.0 - next_fac;

            let sample_1 = *self.data.get(intpos).unwrap_or(&0.0);
            let sample_2 = *self.data.get(nextpos).unwrap_or(&0.0);

            let pre_scale = sample_1 * inv_fac + sample_2 * next_fac;

            let scaled_signed_float = pre_scale * self.volume;
            //let scaled = (scaled_signed_float + 128.0) as u8;
            *dst = scaled_signed_float;
            //*dst = scaled;
            self.pos += self.period;
            if self.pos.floor() as usize > self.data.len() {
                self.pos = 0.0;
            }
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio()?;

    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1), // mono
        samples: None,     // default sample size
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // Show obtained AudioSpec
        println!("{:?}", spec);

        let wav = AudioSpecWAV::load_wav(Path::new("./wav/Classique.WAV"))
            .expect("Could not load test WAV file");

        let cvt = AudioCVT::new(
            wav.format,
            wav.channels,
            wav.freq,
            spec.format,
            spec.channels,
            spec.freq,
        )
        .expect("Could not convert WAV file");

        let data = cvt.convert(wav.buffer().to_vec());

        let f32_data: Vec<f32> = data
            .chunks(4)
            .map(|c| f32::from_ne_bytes(c.try_into().unwrap()))
            .collect();

        Sound {
            data: f32_data,
            volume: 0.25,
            pos: 0.0,
            period: 0.5,
        }
        // initialize the audio callback
    })?;

    // Start playback
    device.resume();

    // Play for 2 seconds
    std::thread::sleep(Duration::from_millis(2_000));

    // Device is automatically closed when dropped

    Ok(())
}

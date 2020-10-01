use std::time::{Duration, Instant};
use std::thread::sleep;

use term_size;

use asciidf::{renderer, examples};

fn main() {
    let (w, h) = term_size::dimensions().unwrap_or((137, 28));
    let mut pixels = renderer::Pixels::new(w, h-1);

    const frame_time: f32 = 0.200;
    loop {
        let now = Instant::now();

        let (w, h) = term_size::dimensions().unwrap_or((137, 28));
        pixels.resize(w, h);
        pixels.update(examples::simple_sdf);
        pixels.draw();

        let elapsed = now.elapsed().as_secs_f32();
        if elapsed < frame_time {
            sleep(Duration::from_secs_f32(frame_time - elapsed));
        }
    }
}


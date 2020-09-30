use term_size;

use asciidf::{renderer, examples};

fn main() {
    let (w, h) = term_size::dimensions().unwrap_or((137, 28));
    let mut pixels = renderer::Pixels::new(w, h-1);

    pixels.update(examples::simple_sdf);
    pixels.draw();
}


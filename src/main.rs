// Some useful references:
// https://www.iquilezles.org/www/index.htm
// http://jamie-wong.com/2016/07/15/ray-marching-signed-distance-functions/

mod renderer;

use term_size;

fn main() {
    let (w, h) = term_size::dimensions().expect("unable to get terminal dimensions");
    let mut pixels = renderer::Pixels::new(w, h-1);

    pixels.update();
    pixels.draw();
}

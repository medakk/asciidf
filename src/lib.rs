use wasm_bindgen::prelude::*;

pub mod examples;
pub mod util;
pub mod renderer;

#[wasm_bindgen]
pub fn do_thing() -> String {
    let (w, h) = (200, 40);
    let mut pixels = renderer::Pixels::new(w, h-1);
    pixels.update(examples::simple_sdf);

    pixels.html()
}

pub use renderer::Pixels;

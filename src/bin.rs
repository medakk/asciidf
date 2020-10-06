use std::time::{Duration, Instant};
use std::thread::sleep;

use term_size;

use asciidf::{renderer, examples};
use ncurses;

//TODO argparse for direct print vs ncurses

fn main() {
    let mut w = 0;
    let mut h = 0;

    let local_conf = ncurses::LcCategory::all;
    ncurses::setlocale(local_conf, "en_US.UTF-8");

    ncurses::initscr();
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();
    ncurses::getmaxyx(ncurses::stdscr(), &mut h, &mut w);

    let mut pixels = renderer::Pixels::new(w as usize, h as usize);

    const frame_time: f32 = 0.028;
    loop {
        let now = Instant::now();

        ncurses::getmaxyx(ncurses::stdscr(), &mut h, &mut w);
        pixels.resize(w as usize, h as usize);
        pixels.update(examples::simple_sdf);
        ncurses::clear();
        pixels.ncurses_draw();

        // std::thread::sleep causes weird rendering issues, so we busy wait :(
        while now.elapsed().as_secs_f32() < frame_time {
        }
    }
}

/*
fn NOTCURSES_main() {

    let (w, h) = term_size::dimensions().unwrap_or((137, 28));
    let mut pixels = renderer::Pixels::new(w, h-1);

    const frame_time: f32 = 0.018;
    loop {
        let now = Instant::now();

        let (w, h) = term_size::dimensions().unwrap_or((137, 28));
        pixels.resize(w, h);
        pixels.update(examples::simple_sdf);
        pixels.draw();

        // std::thread::sleep causes weird rendering issues, so we busy wait :(
        while now.elapsed().as_secs_f32() < frame_time {
        }
    }
}
*/
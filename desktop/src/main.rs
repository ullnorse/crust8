use std::env;

use crust8_core::{SCREEN_HEIGHT, SCREEN_WIDTH};

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    let sld_context = sdl3::init().unwrap();
    let video_subsystem = sld_context.video().unwrap();
    let window = video_subsystem
        .window("Crust8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    canvas.clear();
    canvas.present();
}

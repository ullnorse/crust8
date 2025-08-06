use crust8_core::{Emulator, SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl3::{
    EventPump,
    event::Event,
    keyboard::Keycode,
    pixels::PixelFormatEnum,
    render::{Canvas, TextureAccess},
    video::Window,
};
use std::{
    env,
    fs::File,
    io::Read,
    time::{Duration, Instant},
};

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const FRAME_RATE: u64 = 60;
const INSTRUCTIONS_PER_SECOND: u64 = 700;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Crust8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture(
            Some(PixelFormatEnum::RGB24.into()),
            TextureAccess::Streaming,
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .unwrap();
    texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut emu = Emulator::new();
    let mut rom = File::open(&args[1]).unwrap();
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    emu.load(&buffer);

    let frame_duration = Duration::from_micros(1_000_000 / FRAME_RATE);
    let instr_duration = Duration::from_micros(1_000_000 / INSTRUCTIONS_PER_SECOND);
    let mut last_frame = Instant::now();
    let mut last_instr = Instant::now();

    loop {
        handle_input(&mut event_pump, &mut emu);
        while last_instr.elapsed() >= instr_duration {
            emu.tick();
            last_instr += instr_duration;
        }
        if last_frame.elapsed() >= frame_duration {
            emu.tick_timers();
            draw_screen(&emu, &mut canvas, &mut texture);
            last_frame += frame_duration;
        }
    }
}

fn draw_screen(emu: &Emulator, canvas: &mut Canvas<Window>, texture: &mut sdl3::render::Texture) {
    let mut pixels = [0u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3];
    let screen_buf = emu.get_display();
    for (i, &on) in screen_buf.iter().enumerate() {
        let color = if on { 0xFF } else { 0x00 };
        let offset = i * 3;
        pixels[offset] = color;
        pixels[offset + 1] = color;
        pixels[offset + 2] = color;
    }
    texture.update(None, &pixels, SCREEN_WIDTH * 3).unwrap();
    canvas.clear();
    canvas.copy(texture, None, None).unwrap();
    canvas.present();
}

fn handle_input(event_pump: &mut EventPump, emu: &mut Emulator) {
    for evt in event_pump.poll_iter() {
        match evt {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                if let Some(k) = key2btn(key) {
                    emu.keypress(k, true);
                }
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } => {
                if let Some(k) = key2btn(key) {
                    emu.keypress(k, false);
                }
            }
            _ => (),
        }
    }
}

fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::_1 => Some(0x1),
        Keycode::_2 => Some(0x2),
        Keycode::_3 => Some(0x3),
        Keycode::_4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

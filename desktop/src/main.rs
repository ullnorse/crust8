use std::{fs::File, io::Read, time::Duration};

use crust8_core::{Emulator, SCREEN_HEIGHT, SCREEN_WIDTH};
use eframe::{
    CreationContext, NativeOptions,
    egui::{
        CentralPanel, Color32, Context, ImageSource, Key, TextureHandle, TextureOptions, Vec2,
        ViewportBuilder, load::SizedTexture,
    },
};

const INSTRUCTIONS_PER_FRAME: u64 = 4;

struct App {
    emu: Emulator,
    texture: Option<TextureHandle>,
}

impl App {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        let args: Vec<_> = std::env::args().collect();
        if args.len() != 2 {
            panic!("Usage: cargo run -- path/to/rom");
        }

        let mut emu = Emulator::new();
        let mut rom = File::open(&args[1]).unwrap();
        let mut buffer = Vec::new();
        rom.read_to_end(&mut buffer).unwrap();
        emu.load(&buffer);

        Self { emu, texture: None }
    }

    fn handle_input(&mut self, ctx: &Context) {
        const KEY_MAP: [(Key, usize); 16] = [
            (Key::Num1, 0x1),
            (Key::Num2, 0x2),
            (Key::Num3, 0x3),
            (Key::Num4, 0xC),
            (Key::Q, 0x4),
            (Key::W, 0x5),
            (Key::E, 0x6),
            (Key::R, 0xD),
            (Key::A, 0x7),
            (Key::S, 0x8),
            (Key::D, 0x9),
            (Key::F, 0xE),
            (Key::Z, 0xA),
            (Key::X, 0x0),
            (Key::C, 0xB),
            (Key::V, 0xF),
        ];

        ctx.input(|i| {
            for (key, chip8_key) in KEY_MAP {
                self.emu.keypress(chip8_key, i.key_down(key));
            }
        })
    }

    fn draw_screen(&mut self, ctx: &Context) {
        let display_buffer = self.emu.get_display();

        let pixels: Vec<Color32> = display_buffer
            .iter()
            .map(|&on| if on { Color32::WHITE } else { Color32::BLACK })
            .collect();

        let image = eframe::egui::ColorImage {
            size: [SCREEN_WIDTH, SCREEN_HEIGHT],
            source_size: Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32),
            pixels,
        };

        let options = TextureOptions::NEAREST;

        let texture = self
            .texture
            .get_or_insert_with(|| ctx.load_texture("chip8_screen", image.clone(), options));

        texture.set(image, options);

        CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();

            ui.image(ImageSource::Texture(SizedTexture {
                id: texture.id(),
                size: available_size,
            }))
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.handle_input(ctx);
        self.emu.tick_timers();

        for _ in 0..INSTRUCTIONS_PER_FRAME {
            if let Err(e) = self.emu.tick() {
                eprintln!("Emulator error: {:?}", e);
                break;
            }
        }

        self.draw_screen(ctx);

        ctx.request_repaint_after(Duration::from_millis(16));
    }
}

fn main() -> eframe::Result {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(Vec2::new(
            (SCREEN_WIDTH * 15) as f32,
            (SCREEN_HEIGHT * 15) as f32,
        )),
        ..Default::default()
    };

    eframe::run_native(
        "Crust8 Emulator",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

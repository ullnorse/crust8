use anyhow::Context as _;
use clap::Parser;
use crust8_core::{Emulator, SCREEN_HEIGHT, SCREEN_WIDTH};
use eframe::{
    CreationContext, NativeOptions,
    egui::{
        CentralPanel, Color32, Context, ImageSource, Key, TextureHandle, TextureOptions, Vec2,
        ViewportBuilder, load::SizedTexture,
    },
};
use std::{path::PathBuf, time::Duration};

const INSTRUCTIONS_PER_FRAME: u64 = 4;

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

/// Crust8 — A CHIP-8 Emulator written in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the CHIP-8 ROM file
    rom_path: PathBuf,
}

struct App {
    emu: Emulator,
    texture: Option<TextureHandle>,
}

impl App {
    pub fn new(_cc: &CreationContext<'_>, emu: Emulator) -> Self {
        Self { emu, texture: None }
    }

    fn handle_input(&mut self, ctx: &Context) {
        ctx.input(|i| {
            for (key, chip8_key) in KEY_MAP {
                self.emu.keypress(chip8_key, i.key_down(key));
            }
        })
    }

    fn draw_screen(&mut self, ctx: &Context) {
        let pixels: Vec<Color32> = self
            .emu
            .get_display()
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
            ui.image(ImageSource::Texture(SizedTexture {
                id: texture.id(),
                size: ui.available_size(),
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
            }
        }

        self.draw_screen(ctx);
        ctx.request_repaint_after(Duration::from_millis(16));
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut emu = Emulator::new();

    let buffer = std::fs::read(&cli.rom_path)
        .with_context(|| format!("Failed to read ROM {:?}", cli.rom_path))?;

    emu.load(&buffer);

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
        Box::new(move |cc| Ok(Box::new(App::new(cc, emu)))),
    )
    .map_err(|e| anyhow::anyhow!("{e}"))
}

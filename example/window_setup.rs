extern crate ggez;

use ggez::event;
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode, WindowSetup, NumSamples, FullscreenType};
use std::env;
use std::path;

struct MainState {
    text: graphics::Text,
    frames: usize,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;
        let text = graphics::Text::new(ctx, "Hello world!", &font)?;

        let s = MainState { text, frames: 0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let dest_point = graphics::Point2::new(10.0, 10.0);
        graphics::draw(ctx, &self.text, dest_point, 0.0)?;
        graphics::present(ctx);

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }
}

pub fn main() {
    let ctx = &mut ContextBuilder::new("logging", "ggez")
        .window_setup(
            WindowSetup {
                title: "This is new title".to_string(),
                icon: "".to_owned(),
                resizable: false,
                allow_highdpi: true,
                samples: NumSamples::One,
            },
        )
        .window_mode(
            WindowMode {
                width: 800,
                height: 600,
                borderless: false,
                fullscreen_type: FullscreenType::Off,
                vsync: true,
                min_width: 0,
                max_width: 0,
                min_height: 0,
                max_height: 0,
            },
        )
        .build()
        .unwrap();
    
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}

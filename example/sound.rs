extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::audio;
use ggez::timer;

use std::env;
use std::path;

struct MainState {
    #[allow(dead_code)]
    sound: audio::Source,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let sound = audio::Source::new(ctx, "/sound.ogg").unwrap();
        let _ = sound.play();
        let s = MainState {
            sound,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if timer::get_ticks(ctx) % 10 == 0 {
            let _ = self.sound.play();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}

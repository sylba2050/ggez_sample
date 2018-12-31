extern crate ggez;

use ggez::graphics;
use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::filesystem;

use std::{env, path, str};
use std::io::{Read, Write};

struct MainState {
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {};
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
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

    let mut buffer = Vec::new();
    let mut test_file = ctx.filesystem.open("/test.txt").unwrap();
    test_file.read_to_end(&mut buffer).unwrap();

    println!("{}", str::from_utf8(&buffer).unwrap());

    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}

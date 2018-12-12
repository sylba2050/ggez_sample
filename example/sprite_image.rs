extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;

struct MainState {
    image: graphics::Image,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::new(ctx, "/dragon1.png")?;
        let s = MainState {
            image,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if timer::get_ticks(ctx) % 100 == 0 {
            println!("Delta frame time: {:?} ", timer::get_delta(ctx));
            println!("Average FPS: {}", timer::get_fps(ctx));
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::draw_ex(
            ctx,
            &self.image,
            graphics::DrawParam {
                src: graphics::Rect::new(0.25, 0.25, 0.5, 0.5),
                dest: graphics::Point2::new(0.0, 0.0),
                .. Default::default()
            },
        ).expect("cannot draw tile");

        graphics::present(ctx);
        Ok(())
    }
}

// Creating a gamestate depends on having an SDL context to load resources.
// Creating a context depends on loading a config file.
// Loading a config file depends on having FS (or we can just fake our way around it
// by creating an FS and then throwing it away; the costs are not huge.)
pub fn main() {
    let c = conf::Conf::new();
    println!("Starting with default config: {:#?}", c);
    let ctx = &mut Context::load_from_conf("spritebatch", "ggez", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
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

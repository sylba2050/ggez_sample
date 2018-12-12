extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

struct MainState {
    image: graphics::Image,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::new(_ctx, "/shiro.png")?;
        let s = MainState {
            image,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::draw_ex(
            ctx,
            &self.image,
            graphics::DrawParam {
                src: graphics::Rect::new(0., 0., 120. / 320., 120. / 240.),
                dest: graphics::Point2::new(0.0, 0.0),
                .. Default::default()
            },
        ).expect("cannot draw tile");
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}

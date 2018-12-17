extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::graphics::{self, DrawMode, Point2, Color};
use ggez::{Context, GameResult};

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

        // 線
        let color = Color::from((255, 0, 0, 255)); //rgba
        graphics::set_color(ctx, color)?;
        let start_point = graphics::Point2::new(5., 5.); // 開始座標
        let end_point = graphics::Point2::new(500., 5.); // 終了座標
        graphics::line(
            ctx,
            &[start_point, end_point],
            2., // 幅
        )?;

        // 円
        let color = Color::from((0, 255, 0, 255)); //rgba
        graphics::set_color(ctx, color)?;
        graphics::circle(
            ctx,
            DrawMode::Fill, // 塗りつぶしモード
            Point2::new(100., 380.), // 中心座標
            100., // 半径
            2., // 許容誤差
        )?;

        // 楕円
        let color = Color::from((0, 0, 255, 255)); //rgba
        graphics::set_color(ctx, color)?;
        graphics::ellipse(
            ctx,
            DrawMode::Fill, // 塗りつぶしモード
            Point2::new(600., 200.), //中心座標
            50., //x方向半径
            120., //y方向半径
            1. //許容誤差
        )?;

        // 長方形
        let color = Color::from((0, 0, 0, 255)); //rgba
        graphics::set_color(ctx, color)?;
        graphics::rectangle(
            ctx,
            DrawMode::Fill, // 塗りつぶしモード
            graphics::Rect::new(600., 400., 150., 100.), // x, y, w, h
        )?;


        // 多角形
        let color = Color::from((255, 255, 255, 255)); //rgba
        graphics::set_color(ctx, color)?;
        let vertices1 = graphics::Point2::new(300., 300.); // 頂点
        let vertices2 = graphics::Point2::new(250., 350.); // 頂点
        let vertices3 = graphics::Point2::new(350., 350.); // 頂点
        graphics::polygon(
            ctx,
            DrawMode::Fill, // 塗りつぶしモード
            &[vertices1, vertices2, vertices3],
        )?;

        // 点
        let color = Color::from((155, 155, 155, 255)); //rgba
        graphics::set_color(ctx, color)?;
        let point1 = graphics::Point2::new(150., 150.); // 点
        let point2 = graphics::Point2::new(125., 175.); // 点
        let point3 = graphics::Point2::new(175., 175.); // 点
        graphics::points(
            ctx,
            &[point1, point2, point3],
            5., //点のサイズ
        )?;


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

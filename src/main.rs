mod invaders;
mod background;
mod player;
mod enemy;
mod enemy_grid;
mod shot;
mod common;

// export WINIT_HIDPI_FACTOR=1

use ggez::conf::{WindowMode,FullscreenType,WindowSetup};
use ggez::event;
use invaders::Invaders;

const SCRWIDTH: u32 = 1920;
const SCRHEIGHT: u32 = 1080;

fn main() -> ggez::GameResult<()> {
    let cb = ggez::ContextBuilder::new("fire", "ggez")
        .add_resource_path("resources")
        .window_setup(WindowSetup::default().vsync(true))
        .window_mode(WindowMode::default()
                     .fullscreen_type(FullscreenType::Desktop)
                     .dimensions(SCRWIDTH as f32, SCRHEIGHT as f32)
        );
    let (ctx, events_loop) = &mut cb.build()?;

    let state = &mut Invaders::new(SCRWIDTH as f32, SCRHEIGHT as f32, ctx);
    event::run(ctx, events_loop, state)
}

use ggez::event::{EventHandler,MouseButton,KeyCode,KeyMods};
use ggez::{Context,GameResult,timer};
use ggez::graphics::{MeshBuilder,Color,DrawParam,clear,present,draw};
use crate::enemy_grid::EnemyGrid;
use crate::player::Player;
use crate::background::Background;

pub struct Invaders {
    w: f32,
    h: f32,
    score: u32,
    enemy_grid: EnemyGrid,
    player: Player,
    background: Background,
}

impl Invaders {
    pub fn new(w: f32, h: f32, ctx: &mut Context) -> Self {
        Invaders {
            w, h,
            score: 0,
            enemy_grid: EnemyGrid::new(w, h, 13, 3),
            player: Player::new(w, h, ctx),
            background: Background::new(ctx),
        }
    }
}

impl EventHandler for Invaders {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.background.update();
        self.enemy_grid.update();
        self.player.update();
        self.player.check_hits(&mut self.enemy_grid);
        if self.enemy_grid.has_won() {
            self.enemy_grid.reset();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::new(0.0,0.0,0.0,1.0));
        let canvas = self.background.draw(ctx);
        draw(ctx, &*canvas, DrawParam::default())?;
        
        let mut mb = MeshBuilder::new();
        
        self.enemy_grid.draw(ctx, &mut mb);
        self.player.draw_shots(ctx, &mut mb);

        let mesh = mb.build(ctx)?;
        
        draw(ctx, &mesh, DrawParam::default())?;
        self.player.draw(ctx, &mut mb);
        
        present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context,
                      keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Right => self.player.set_motion(4.0),
            KeyCode::Left => self.player.set_motion(-4.0),
            KeyCode::Escape => ggez::quit(ctx),
            KeyCode::Space => self.player.shoot(),
            _ => ()
        }
    }
    
    fn key_up_event(&mut self, ctx: &mut Context,
                    keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Right => self.player.set_motion(0.0),
            KeyCode::Left => self.player.set_motion(0.0),
            _ => ()
        }
    }
    

}

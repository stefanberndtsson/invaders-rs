use std::ops::Add;
use ggez::{Context};
use ggez::graphics::{MeshBuilder,DrawMode,Color};
use ggez::nalgebra::Point2;
use crate::common::{Point,Area};
use crate::shot::Shot;

const ENEMYWIDTH: f32 = 48.0;
const ENEMYHEIGHT: f32 = 48.0;

pub struct Enemy {
    pub pos: Point,
    pub size: Point,
}

impl Enemy {
    pub fn new(pos: Point) -> Self {
        let size = Point::new(ENEMYWIDTH, ENEMYHEIGHT);
        Enemy { pos, size }
    }

    pub fn is_hit(&self, shot: &Shot, grid_offset: Point) -> bool {
        Area::intersect(&(self + grid_offset), shot)
    }

    pub fn update(&mut self, motion: Point) {
        self.pos = self.pos + motion;
    }

    pub fn at_side(&self, left: f32, right: f32) -> bool {
        if self.pos.x <= left { return true; }
        if self.pos.x + self.size.x >= right { return true; }
        false
    }
    
    pub fn draw(&mut self, _ctx: &mut Context, mb: &mut MeshBuilder) {
        let _ = mb.polygon(DrawMode::fill(),
                   &[
                       Point2::new(self.pos.x, self.pos.y),
                       Point2::new(self.pos.x+self.size.x, self.pos.y),
                       Point2::new(self.pos.x+self.size.x, self.pos.y+self.size.y),
                       Point2::new(self.pos.x, self.pos.y+self.size.y),
                       Point2::new(self.pos.x, self.pos.y),
                   ],
                   Color::new(1.0,0.0,1.0,1.0));
    }
}

impl From<&Enemy> for Area {
    fn from(enemy: &Enemy) -> Area {
        Area { x: enemy.pos.x, y: enemy.pos.y, w: enemy.size.x, h: enemy.size.y }
    }
}

impl Add<Point> for &Enemy {
    type Output = Enemy;
    
    fn add(self, point: Point) -> Enemy {
        let pos = self.pos + point;
        Enemy { pos, size: self.size }
    }
}

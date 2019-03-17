use std::ops::Add;
use ggez::{Context};
use ggez::graphics::{MeshBuilder,DrawMode,Color};
use ggez::nalgebra::Point2;
use crate::common::{Point,Area};
use crate::shot::Shot;

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Enemy {
            x,
            y,
            w,
            h,
        }
    }

    pub fn is_hit(&self, shot: &Shot, grid_offset: Point) -> bool {
        Area::intersect(&(self + grid_offset), shot)
    }
    
    pub fn draw(&mut self, _ctx: &mut Context, position: Point, size: Point, mb: &mut MeshBuilder) {
        let _ = mb.polygon(DrawMode::fill(),
                   &[
                       Point2::new(position.x, position.y),
                       Point2::new(position.x+size.x-2.0, position.y),
                       Point2::new(position.x+size.x-2.0, position.y+size.y-2.0),
                       Point2::new(position.x, position.y+size.y-2.0),
                       Point2::new(position.x, position.y),
                   ],
                   Color::new(1.0,0.0,1.0,1.0));
    }
}

impl From<&Enemy> for Area {
    fn from(enemy: &Enemy) -> Area {
        Area { x: enemy.x, y: enemy.y, w: enemy.w, h: enemy.h }
    }
}

impl Add<Point> for &Enemy {
    type Output = Enemy;
    
    fn add(self, point: Point) -> Enemy {
        Enemy {
            x: self.x + point.x,
            y: self.y + point.y,
            w: self.w,
            h: self.h
        }
    }
}

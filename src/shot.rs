use ggez::{Context};
use ggez::graphics::{MeshBuilder,DrawMode,Color};
use ggez::nalgebra::Point2;
use crate::common::Area;

const SHOTWIDTH: f32 = 5.0;
const SHOTHEIGHT: f32 = 15.0;
const SHOTSPEED: f32 = 8.0;

pub struct Shot {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub s: f32,
}

pub enum ShotStatus {
    Alive,
    Offscreen,
}

impl Shot {
    pub fn new(x: f32, y: f32) -> Self {
        Shot { x, y, w: SHOTWIDTH, h: SHOTHEIGHT, s: SHOTSPEED }
    }

    pub fn update(&mut self) -> ShotStatus {
        if self.y > -self.h {
            self.y -= self.s;
        }
        if self.y < -SHOTHEIGHT {
            ShotStatus::Offscreen
        } else {
            ShotStatus::Alive
        }
    }
    
    pub fn draw(&mut self, _ctx: &mut Context, mb: &mut MeshBuilder) {
        let _ = mb.polygon(DrawMode::fill(),
                   &[
                       Point2::new(self.x, self.y),
                       Point2::new(self.x+self.w, self.y),
                       Point2::new(self.x+self.w, self.y+self.h),
                       Point2::new(self.x, self.y+self.h),
                       Point2::new(self.x, self.y),
                   ],
                   Color::new(1.0,1.0,1.0,1.0));
    }
}

impl From<&Shot> for Area {
    fn from(shot: &Shot) -> Area {
        Area { x: shot.x, y: shot.y, w: shot.w, h: shot.h }
    }
}

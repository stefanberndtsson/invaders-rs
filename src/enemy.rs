use std::ops::Add;
use ggez::{Context};
use ggez::graphics::{MeshBuilder,DrawMode,Color,Image,draw};
use ggez::nalgebra::Point2;
use crate::common::{Point,Area};
use crate::shot::Shot;

const ENEMYWIDTH: f32 = 48.0;
const ENEMYHEIGHT: f32 = 48.0;
const ENEMYANIM: usize = 8;
const ANIMSPEED: u32 = 4;

pub struct Enemy {
    pub pos: Point,
    pub size: Point,
    pub animstep: usize,
    pub animdelay: u32,
    pub images: Vec<Image>,
}

impl Enemy {
    pub fn new(pos: Point, ctx: &mut Context) -> Self {
        let size = Point::new(ENEMYWIDTH, ENEMYHEIGHT);
        let mut images = Vec::new();
        let imagetype = (rand::random::<u32>() % 2) + 1;
        let animstep = rand::random::<usize>() % ENEMYANIM;
        let animdelay = (rand::random::<u32>() % 7) + 3;
        
        for i in 0..8 {
            let filename = format!("/enemy{}-{}.png", imagetype, i);
            let image = Image::new(ctx, filename).unwrap();
            images.push(image);
        }

        Enemy { pos, size, animstep, animdelay, images }
    }

    pub fn is_hit(&self, shot: &Shot, grid_offset: Point) -> bool {
        let enemy_area: Area = self.into();
        Area::intersect(enemy_area + grid_offset, shot)
    }

    pub fn update(&mut self, motion: Point) {
        self.pos = self.pos + motion;
        if self.animdelay > 0 {
            self.animdelay -= 1;
        } else {
            self.animdelay = ANIMSPEED;
            self.animstep += 1;
            self.animstep = self.animstep % ENEMYANIM;
        }
    }

    pub fn at_side(&self, left: f32, right: f32) -> bool {
        if self.pos.x <= left { return true; }
        if self.pos.x + self.size.x >= right { return true; }
        false
    }
    
    pub fn draw(&mut self, ctx: &mut Context, _mb: &mut MeshBuilder) {
        // let _ = mb.polygon(DrawMode::fill(),
        //            &[
        //                Point2::new(self.pos.x, self.pos.y),
        //                Point2::new(self.pos.x+self.size.x, self.pos.y),
        //                Point2::new(self.pos.x+self.size.x, self.pos.y+self.size.y),
        //                Point2::new(self.pos.x, self.pos.y+self.size.y),
        //                Point2::new(self.pos.x, self.pos.y),
        //            ],
        //            Color::new(1.0,0.0,1.0,1.0));
        let pos: Point2<f32> = Point2::new(self.pos.x, self.pos.y);
        let _ = draw(ctx, &self.images[self.animstep], (pos, 0.0, Color::new(1.0, 1.0, 1.0, 1.0)));
    }
}

impl From<&Enemy> for Area {
    fn from(enemy: &Enemy) -> Area {
        Area { x: enemy.pos.x, y: enemy.pos.y, w: enemy.size.x, h: enemy.size.y }
    }
}


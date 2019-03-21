//use std::collections::BTreeSet;
use ggez::{Context};
use ggez::graphics::{MeshBuilder,DrawMode,Color,Image,draw};
use ggez::nalgebra::Point2;
use crate::shot::Shot;
use crate::enemy::Enemy;
use crate::enemy_grid::EnemyGrid;
use crate::common::Point;

const PLAYERSIZE: f32 = 81.0;
const PLAYERWIDTH: f32 = 63.0;

pub struct Player {
    w: f32,
    h: f32,
    x: f32,
    cooldown: i32,
    motion: f32,
    shots: Vec<Shot>,
    image: Image,
}

impl Player {
    pub fn new(w: f32, h: f32, ctx: &mut Context) -> Self {
        let image = Image::new(ctx, "/ship-center.png").unwrap();

        Player {
            w, h, x: w/2.0, cooldown: 0, motion: 0.0, shots: Vec::new(), image
        }
    }

    pub fn set_motion(&mut self, motion: f32) {
        self.motion = motion;
    }

    pub fn shoot(&mut self) {
        if self.cooldown == 0 {
            self.shots.push(Shot::new(self.x + PLAYERWIDTH/2.0, self.h-PLAYERSIZE));
            self.cooldown = 10;
        }
    }
    
    pub fn draw(&mut self, ctx: &mut Context, mb: &mut MeshBuilder) {
        // let _ = mb.polygon(DrawMode::fill(),
        //            &[
        //                Point2::new(self.x, self.h-PLAYERSIZE),
        //                Point2::new(self.x+PLAYERSIZE/3.0, self.h),
        //                Point2::new(self.x-PLAYERSIZE/3.0, self.h),
        //                Point2::new(self.x, self.h-PLAYERSIZE),
        //            ],
        //                    Color::new(1.0,1.0,1.0,1.0));
        //
        let pos: Point2<f32> = Point2::new(self.x, self.h-PLAYERSIZE);
        let _ = draw(ctx, &self.image, (pos, 0.0, Color::new(1.0, 1.0, 1.0, 1.0)));
    }

    pub fn draw_shots(&mut self, ctx: &mut Context, mb: &mut MeshBuilder) {
        for shot in &mut self.shots {
            shot.draw(ctx, mb);
        }
    }
    
    pub fn check_hits(&mut self, enemy_grid: &mut EnemyGrid) {
        let mut enemies_to_remove = Vec::new();
        let mut shots_to_remove = Vec::new();
        
        for (i, enemy) in enemy_grid.enemies.iter().enumerate() {
            for (j, shot) in self.shots.iter().enumerate() {
                if enemy.is_hit(shot, enemy_grid.position) {
                    enemies_to_remove.push(i);
                    shots_to_remove.push(j);
                }
            }
        }

        for i in enemies_to_remove.iter().rev() {
            enemy_grid.enemies.remove(*i);
        }
        for i in shots_to_remove.iter().rev() {
            self.shots.remove(*i);
        }
    }
    
    pub fn update(&mut self) {
        self.x += self.motion;
        if self.x <= PLAYERSIZE {
            self.x = PLAYERSIZE;
            self.motion = 0.0;
        } else if self.x >= self.w - PLAYERSIZE {
            self.x = self.w - PLAYERSIZE;
            self.motion = 0.0;
        }
        if self.cooldown > 0 {
            self.cooldown -= 1;
        }
        for shot in &mut self.shots {
            shot.update();
        }
    }
}

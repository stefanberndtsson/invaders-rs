use ggez::{Context};
use ggez::graphics::{MeshBuilder};
use crate::enemy::Enemy;
use crate::common::Point;

const STEP_COUNTER: u32 = 10;

pub struct EnemyGrid {
    w: f32,
    h: f32,
    pub enemies: Vec<Enemy>,
    max_width: i32,
    max_height: i32,
    enemy_size: Point,
    pub position: Point,
    leftmost_column: i32,
    rightmost_column: i32,
    step_counter: u32,
    step_direction: f32,
    step_amount: f32,
}

impl EnemyGrid {
    pub fn new(w: f32, h: f32, max_width: i32, max_height: i32) -> Self {
        let mut enemies = Vec::new();
        let enemy_size = Point::new(64.0, 64.0);
        Self::generate_enemies(max_width, max_height, enemy_size, &mut enemies);
        
        let position = Point::new(0.0, 0.0);

        let leftmost_column = 0;
        let rightmost_column = max_width - 1;

        let step_counter = STEP_COUNTER;
        let step_direction = 1.0;
        let step_amount = 16.0;
        
        EnemyGrid {
            w,
            h,
            enemies,
            max_width,
            max_height,
            enemy_size,
            position,
            leftmost_column,
            rightmost_column,
            step_counter,
            step_direction,
            step_amount,
        }
    }

    fn generate_enemies(width: i32, height: i32, size: Point, vec: &mut Vec<Enemy>) {
        for y in 0..height {
            for x in 0..width {
                vec.push(Enemy::new(x as f32*size.x, y as f32*size.y, size.x, size.y));
            }
        }
    }

    pub fn update(&mut self) {
        if self.step_counter > 0 {
            self.step_counter -= 1;
        } else {
            if self.at_side() {
                self.position = self.position + Point::new(0.0, self.step_amount);
                self.step_direction = -self.step_direction;
            } else {
                self.position = self.position + Point::new(self.step_direction * self.step_amount, 0.0);
            }
            self.step_counter = STEP_COUNTER;
        }
    }

    fn at_side(&self) -> bool {
        if self.step_direction > 0.0 &&
            self.position.x >= self.w - (self.max_width as f32 * self.enemy_size.x) {
                return true;
            }
        if self.step_direction < 0.0 &&
            self.position.x <= 0.0 {
                return true;
            }
        false
    }
    
    pub fn draw(&mut self, ctx: &mut Context, mb: &mut MeshBuilder) {
        for enemy in &mut self.enemies {
            let relative_position = Point::new(enemy.x, enemy.y);
            let enemy_position = self.position + relative_position;
            enemy.draw(ctx, enemy_position, self.enemy_size, mb);
        }
        
    }
}

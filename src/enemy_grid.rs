use ggez::{Context};
use ggez::graphics::{MeshBuilder};
use crate::enemy::Enemy;
use crate::common::Point;

const STEP_COUNTER: f32 = 10.0;
const STEP_AMOUNT: f32 = 16.0;

pub struct EnemyGrid {
    w: f32,
    h: f32,
    pub enemies: Vec<Enemy>,
    max_enemies: f32,
    max_width: i32,
    max_height: i32,
    enemy_size: Point,
    pub position: Point,
    leftmost_column: i32,
    rightmost_column: i32,
    step_counter: f32,
    step_direction: f32,
    moving_down: bool,
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

        let max_enemies = (max_width * max_height) as f32;
        let step_counter = STEP_COUNTER;
        let step_amount = 16.0 / STEP_COUNTER;
        
        EnemyGrid {
            w,
            h,
            enemies,
            max_enemies,
            max_width,
            max_height,
            enemy_size,
            position,
            leftmost_column,
            rightmost_column,
            step_counter,
            moving_down: false,
            step_direction: -1.0,
            step_amount,
        }
    }

    fn generate_enemies(width: i32, height: i32, size: Point, vec: &mut Vec<Enemy>) {
        for y in 0..height {
            for x in 0..width {
                let pos = Point::new(x as f32 * size.x, y as f32 * size.y);
                vec.push(Enemy::new(pos));
            }
        }
    }

    pub fn update(&mut self) {
        let remaining_enemies = self.enemies.len() as f32;

        self.step_amount = 16.0 / STEP_COUNTER;
        self.step_amount = self.step_amount * (self.max_enemies / remaining_enemies);
        
        let motion = match (self.moving_down, self.at_side()) {
            (false, true) => {
                self.step_direction = -self.step_direction;
                self.moving_down = true;
                Point::new(0.0, 4.0)
            },
            _ => {
                self.moving_down = false;
                Point::new(self.step_amount * self.step_direction, 0.0)
            }
        };

        for enemy in &mut self.enemies {
            enemy.update(motion);
        }
    }

    fn at_side(&self) -> bool {
        for enemy in &self.enemies {
            if enemy.at_side(0.0, self.w) { return true }
        }
        false
    }
    
    pub fn draw(&mut self, ctx: &mut Context, mb: &mut MeshBuilder) {
        for enemy in &mut self.enemies {
            enemy.draw(ctx, mb);
        }
    }
}

use ggez::Context;
use ggez::graphics::{Canvas,MeshBuilder,DrawMode,DrawParam,Rect,Color,set_canvas,draw};
use ggez::nalgebra::Point2;
use crate::common::{Point,randrange};

pub struct Background {
    canvas: Canvas,
    stars: Vec<Star>,
}

struct Star {
    pos: Point,
    speed: f32,
}

impl Star {
    fn new(pos: Point, speed: f32) -> Self {
        Star {
            pos, speed
        }
    }
}

impl Background {
    pub fn new(ctx: &mut Context) -> Self {
        let canvas = Canvas::with_window_size(ctx).unwrap();
        let mut stars = Vec::new();

        for _ in 0..500 {
            stars.push(Star::new(Point::new(randrange(0.0,1919.0),
                                            randrange(0.0, 1079.0)), randrange(0.2, 6.5)));
        }
        
        Background {
            canvas, stars
        }
    }

    pub fn update(&mut self) {
        for star in &mut self.stars {
            star.pos = star.pos + Point::new(0.0, star.speed);
            if star.pos.y > 1080.0 {
                star.pos = Point::new(randrange(0.0,1919.0), 0.0);
            }
        }
    }
    
    pub fn draw(&mut self, ctx: &mut Context) -> &Canvas {
        let mut canvas_mb = MeshBuilder::new();
        for star in &mut self.stars {
            canvas_mb.circle(DrawMode::fill(),
                             Point2::new(star.pos.x, star.pos.y), 1.0, 1.0,
                             Color::new(0.5, 0.5, 0.5, 1.0));
        }
        canvas_mb.rectangle(DrawMode::fill(),
                            Rect::new(0.0, 0.0, 1920.0, 1080.0),
                            Color::new(0.0, 0.0, 0.0, 0.7));

        let mesh = canvas_mb.build(ctx).unwrap();
        set_canvas(ctx, Some(&self.canvas));
        let _ = draw(ctx, &mesh, DrawParam::default());
        set_canvas(ctx, None);
        &self.canvas
    }
}

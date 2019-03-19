use std::ops::{Add,AddAssign};
use rand::random;

pub fn randrange(start: f32, stop: f32) -> f32 {
    let random = random::<f32>();
    (random * (stop-start))+start
}

#[derive(Copy,Clone,Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new<T: Into<f64>>(x: T, y: T) -> Self {
        Point { x: x.into() as f32, y: y.into() as f32 }
    }

    pub fn grid_multiply(&self, grid: Point) -> Point {
        Point { x: self.x * grid.x, y: self.y * grid.y }
    }
}

impl Add<Point> for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Add<Point> for &Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Area {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Area {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Area { x, y, w, h }
    }

    pub fn intersect<T: Into<Area>, U: Into<Area>>(area1: T, area2: U) -> bool {
        let area1 = area1.into();
        let area2 = area2.into();

        if  area1.x+area1.w < area2.x ||
            area2.x+area2.w < area1.x ||
            area1.y+area1.h < area2.y ||
            area2.y+area2.h < area1.y {
                false
            } else {
                true
            }
    }
}

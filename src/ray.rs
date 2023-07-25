use crate::vec::{Point, Vec3};

pub struct Ray {
    origin: Point,
    dir: Vec3
}

impl Ray {
    pub fn new(origin: Point, dir: Vec3) -> Ray {
        Ray {
            origin, dir
        }
    }
    pub fn origin(&self) -> Point { self.origin }
    pub fn direction(&self) -> Vec3 { self.dir }
    pub fn at(&self, t: f32) -> Point { 
        self.origin.clone() + (self.dir.clone()*t) 
    }
}

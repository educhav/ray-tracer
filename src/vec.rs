use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub v: [f32; 3]
}

pub type Color = Vec3;
pub type Point = Vec3;

impl Vec3 {
    pub fn new(v1: f32, v2: f32, v3: f32) -> Vec3 {
        Vec3 { v: [v1, v2, v3] }
    }
    pub fn x(&self) -> f32 { self.v[0] }
    pub fn y(&self) -> f32 { self.v[1] }
    pub fn z(&self) -> f32 { self.v[2] }

    pub fn length(&self) -> f32 {
        ((self.v[0]*self.v[0])+(self.v[1]*self.v[1])+(self.v[2]*self.v[2])).sqrt()
    }

    pub fn unit_vector(vec: Vec3) -> Vec3 {
        return vec / (vec.length());
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
        return v1[0]*v2[0] + v1[1]*v2[1] + v1[2]*v2[2];
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, _rhs: Self) -> Self {
        Vec3 {
            v: [self.v[0] + _rhs.v[0], 
                self.v[1] + _rhs.v[1], 
                self.v[2] + _rhs.v[2]
            ]
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] - _rhs.v[0], 
                self.v[1] - _rhs.v[1], 
                self.v[2] - _rhs.v[2]
            ]
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3 {
            v: [self.v[0] * _rhs, 
                self.v[1] * _rhs, 
                self.v[2] * _rhs
            ]
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: f32) -> Vec3 {
        Vec3 {
            v: [self.v[0] * (1.0/_rhs), 
                self.v[1] * (1.0/_rhs), 
                self.v[2] * (1.0/_rhs)
            ]
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            v: [-self.v[0], -self.v[1], -self.v[2]]
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, _rhs: usize) -> &f32 {
        return &self.v[_rhs as usize];
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, _rhs: usize) -> &mut f32 {
        return &mut self.v[_rhs as usize];
    }
}

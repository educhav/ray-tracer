use std::fs;
use std::ops;

struct Vec3 {
    v: Vec<f32>
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            v: vec![x, y, z]
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(mut self, _rhs: Vec3) -> Vec3 {
        self.v[0] += _rhs.v[0];
        self.v[1] += _rhs.v[1];
        self.v[2] += _rhs.v[2];
        return self;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(mut self, _rhs: f32) -> Vec3 {
        self.v[0] /= _rhs;
        self.v[1] /= _rhs;
        self.v[2] /= _rhs;
        return self;
    }
}
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(mut self, _rhs: f32) -> Vec3 {
        self.v[0] *= _rhs;
        self.v[1] *= _rhs;
        self.v[2] *= _rhs;
        return self;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(mut self) -> Vec3 {
        self.v[0] = -self.v[0];
        self.v[1] = -self.v[1];
        self.v[2] = -self.v[2];
        return self;
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


fn main() {

    let width = 256;
    let height = 256;

    let mut contents = String::new();
    contents.push_str(&format!("P3\n{} {}\n255\n", width, height));

    for j in (0..height).rev() {
        // eprintln!("Scanlines remaining: {}", j);
        for i in 0..width {
            contents.push_str(&format!("{} {} {}\n", i, j, 65));
        }
    }

    match fs::write("image.ppm", contents) {
        Ok(it) => it,
        Err(_) => panic!("Couldn't write to file"),
    };

    let vec1 = Vec3::new(1.5, 1.0, 1.0);
    let vec2 = Vec3::new(1.0, 1.0, 1.0);
    let sum = vec1 + vec2;
    println!("{}", sum[0]);

} 

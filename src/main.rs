use std::fs;
pub mod vec;
pub mod ray;

use crate::vec::{Color, Point, Vec3};
use crate::ray::Ray;

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(Point::new(0.0,0.0,-1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = Vec3::unit_vector(ray.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    return Color::new(1.0,1.0,1.0)*(1.0-t) + Color::new(0.5,0.7,1.0)*t;
}

fn format_color(color: &Color) -> String {
    return format!("{} {} {}\n", 
                       (color.x() * 255.0) as i32, 
                       (color.y() * 255.0) as i32, 
                       (color.z() * 255.0) as i32);
}

fn hit_sphere(center: Point, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = Vec3::dot(ray.direction(), ray.direction());
    let b = 2.0 * Vec3::dot(oc, ray.direction());
    let c = Vec3::dot(oc, oc) - (radius*radius);
    let discriminant = b*b - 4.0*a*c;
    return discriminant > 0.0;
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = ((width as f32) / aspect_ratio) as i32;
    println!("height: {}", height);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    println!("viewport width: {}", viewport_width);
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - Vec3::new(0.0, 0.0, focal_length);

    let mut contents = String::new();
    contents.push_str(&format!("P3\n{} {}\n255\n", width, height));
    

    for j in (0..height).rev() {
        // eprintln!("Scanlines remaining: {}", j);
        for i in 0..width {
            let u = (i as f32) / ((width as f32) - 1.0);
            let v = (j as f32) / ((height as f32) - 1.0);
            // eprintln!("u:{}\nv:{}\n", u, v);
            let ray = Ray::new(origin, lower_left_corner + (horizontal*u) + (vertical*v) - origin);
            // let color = Color::new(i as f32, j as f32, 65.0);
            let color = ray_color(&ray);
            contents.push_str(&format_color(&color));
        }
    }
    match fs::write("image.ppm", contents) {
        Ok(it) => it,
        Err(_) => panic!("Couldn't write to file"),
    };
} 

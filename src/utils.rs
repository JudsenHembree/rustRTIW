// ignore unused func
#![allow(dead_code)]

// Imports
use std::io::Write;

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new_zero() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }
    pub fn write_pixel(&self, writer: &mut std::io::BufWriter<std::fs::File>) {
        let ir = (255.999 * self.x) as u32;
        let ig = (255.999 * self.y) as u32;
        let ib = (255.999 * self.z) as u32;

        let pixel = format!("{} {} {} ", ir, ig, ib);
        let err = writer.write(pixel.as_bytes());
        match err {
            Ok(_) => (),
            Err(error) => panic!("Error writing pixel: {}", error),
        }
    }
    pub fn unit_vector(v: Vec3) -> Vec3 {
        let k = 1.0 / v.length();
        Vec3::new(v.x * k, v.y * k, v.z * k)
    }
    pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin: origin, direction: direction }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}

pub struct Camera {
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(viewport_height: f32, aspect_ratio: f32, focal_length: f32, origin: Vec3) -> Camera {
        let viewport_width = aspect_ratio * viewport_height;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        Camera {
            viewport_height: viewport_height,
            viewport_width: viewport_width,
            focal_length: focal_length,
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}

// pixel struct aka the actual image
pub struct Pixels {
    pub pixels: Vec<Vec<Vec3>>,
}

impl Pixels {
    pub fn new(width: usize, height: usize) -> Pixels {
        let mut pixels = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Vec3::new_zero());
            }
            pixels.push(row);
        }
        Pixels { pixels: pixels }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
        self.pixels[y][x] = color;
    }

    pub fn write(&self, writer: &mut std::io::BufWriter<std::fs::File>) {
        let header = format!("P3 {} {} {}\n", self.pixels[0].len(), self.pixels.len(), 255);
        let err = writer.write(header.as_bytes());
        match err {
            Ok(_) => (),
            Err(error) => panic!("Error writing header: {}", error),
        }

        eprintln!("Writing pixels to file...");
        for row in self.pixels.iter() {
            for pixel in row.iter() {
                pixel.write_pixel(writer);
            }
        }
    }
}

//ops
use std::ops::{Add, Sub, Mul, Div};

// Add
impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, other: f32) -> Vec3 {
        Vec3::new(self.x + other, self.y + other, self.z + other)
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self + other.x, self + other.y, self + other.z)
    }
}

// Sub
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: f32) -> Vec3 {
        Vec3::new(self.x - other, self.y - other, self.z - other)
    }
}

impl Sub<Vec3> for f32 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self - other.x, self - other.y, self - other.z)
    }
}

// Mul
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

// Div
impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
impl Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self / rhs.x, self / rhs.y, self / rhs.z)
    }
}
impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

//copy and clone

use std::clone::Clone;
use std::marker::Copy;

// clones
impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        *self
    }
}

impl Clone for Ray {
    fn clone(&self) -> Ray {
        *self
    }
}

impl Clone for Camera {
    fn clone(&self) -> Camera {
        *self
    }
}

// copies
impl Copy for Vec3 {}
impl Copy for Ray {}
impl Copy for Camera {}

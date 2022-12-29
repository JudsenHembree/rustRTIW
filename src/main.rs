// ignore unused func
#![allow(dead_code)]
// Imports
use indicatif::*;
use std::thread::*;

// modules
mod utils;

// Constants
const ASPECT_RATIO: f32 = 16.0 / 9.0;
// make it bigger for parallel to actually matter
const IMAGE_WIDTH: u32 = 100;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

fn set_pixel(x: usize, y: usize, color: utils::Vec3, pixels: &mut utils::Pixels) {
    pixels.pixels[x][y] = color;
}

fn hit_sphere(center: utils::Vec3, radius: f32, ray: utils::Ray) -> f32 {
    let oc = ray.origin() - center;
    let a = utils::Vec3::dot(ray.direction(), ray.direction());
    let b = 2.0 * utils::Vec3::dot(oc, ray.direction());
    let c = utils::Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

fn ray_color(ray: utils::Ray) -> utils::Vec3 {
    // if the ray hits the sphere, return the color
    let t = hit_sphere(utils::Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = utils::Vec3::unit_vector(ray.at(t) - utils::Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * utils::Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }
    let unit_direction = utils::Vec3::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = utils::Vec3::new(1.0, 1.0, 1.0);
    let blue = utils::Vec3::new(0.5, 0.7, 1.0);
    let color = white * (1.0 - t) + blue * t;
    color
}

fn render_pixels_in_row(x: u32, y: u32, pixels: &mut Vec<utils::Vec3>, camera: &utils::Camera) {
    for i in 0..IMAGE_WIDTH {
        let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
        let v = y as f32 / (IMAGE_HEIGHT - 1) as f32;
        let ray = camera.get_ray(u, v);
        let pixel = ray_color(ray);
        pixels[i as usize] = pixel;
    }
}
// Main
fn main() {
    // relative path starts from the root of the project???
    let err = std::fs::File::create("images/image.ppm");
    let ppm_file = match err {
        Ok(file) => file,
        Err(error) => panic!("Error opening file: {}", error),
    };

    // writer for ppm File
    let mut writer = std::io::BufWriter::new(ppm_file);

    // create a camera
    let camera = utils::Camera::new(2.0, ASPECT_RATIO, 1.0, utils::Vec3::new(0.0, 0.0, 0.0));

    // pixel struct aka img
    let mut img = utils::Pixels::new(IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize);

    // Loop through the pixels a couple times for warmup
    eprintln!("Performing warmup");
    for _ in (0..3).progress() {
        for j in (0..IMAGE_HEIGHT).rev() {
            for i in 0..IMAGE_WIDTH {
                let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
                let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;
                let ray = camera.get_ray(u, v);
                let pixel = ray_color(ray);
                img.set_pixel(i as usize, j as usize, pixel);
            }
        }
    }

    let mut sequential_runs: Vec<std::time::Duration> = vec![];
    let mut parallel_runs: Vec<std::time::Duration> = vec![];
    eprintln!("Collecting times for sequential");
    for _ in (0..10).progress() {
        let start = std::time::Instant::now();
        for j in (0..IMAGE_HEIGHT).rev() {
            for i in 0..IMAGE_WIDTH {
                let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
                let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;
                let ray = camera.get_ray(u, v);
                let pixel = ray_color(ray);
                img.set_pixel(i as usize, j as usize, pixel);
            }
        }
        let end = std::time::Instant::now();
        sequential_runs.push(end - start);

    }
    // render in parallel
    eprintln!("Collecting times for parallel");
    for _ in (0..10).progress() {
        let start = std::time::Instant::now();
        let mut joins = Vec::new();
        for j in (0..IMAGE_HEIGHT).rev() {
            let mut jclone = img.pixels[j as usize].clone();
            joins.push(spawn(move || {
                for i in 0..IMAGE_WIDTH {
                    let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
                    let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;
                    let ray = camera.get_ray(u, v);
                    let pixel = ray_color(ray);
                    jclone[i as usize] = pixel;
                }
            }));
        }
        for join in joins{
            join.join();
        }
        let end = std::time::Instant::now();
        parallel_runs.push(end - start);
    }

    // report the results
    println!("Sequential runs: {:?}", sequential_runs);
    println!("Parallel runs: {:?}", parallel_runs);
    let sequential_avg = sequential_runs.iter().sum::<std::time::Duration>() / sequential_runs.len() as u32;
    let parallel_avg = parallel_runs.iter().sum::<std::time::Duration>() / parallel_runs.len() as u32;
    println!("Speedup: {:?}", sequential_avg.as_nanos() / parallel_avg.as_nanos());

    // write the pixels to the ppm File
    img.write(&mut writer);

}

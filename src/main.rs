// This will be used to compare the speed of Rust, to that of Golang.
// I will compare them when both have the same features, and both are multithreaded.
extern crate image;
use image::png::PNGEncoder;
use image::ColorType;

extern crate crossbeam;

extern crate num_cpus;
extern crate time;

mod math;
mod object;
mod ray;
use math::Vec3;
use object::{Hitable, Sphere};
mod camera;
mod material;

use std::f32;
use std::fs::File;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 500;
const SAMPLES: u32 = 100;

fn color(r: &ray::Ray, scn: &Vec<Sphere>, depth: i32, rng: &mut math::RandomSeries) -> Vec3 {
    let hr = scn.hit(r, 0.001, f32::MAX);
    match hr {
        Some(rec) => {
            // TODO: Temp is not a good name for this variable.
            let (atten, scat, temp) = rec.mat.scatter(r, &rec, rng);
            if (depth < 50) && (temp) {
                return atten * color(&scat, scn, depth + 1, rng);
            } else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }

        None => {
            let unit_dir = r.direction.normalize();
            let t: f32 = 0.5 * (unit_dir.y + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn render_band(
    scn: Vec<Sphere>,
    cam: camera::Camera,
    img: &mut [u8],
    bounds: (u32, u32),
    start_height: u32,
) {
    let mut rand_series: math::RandomSeries = math::RandomSeries { state: 485468 };

    // Iterate over all the pixels in the image.
    for x in 0..bounds.0 {
        for y in 0..bounds.1 {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES {
                let u: f32 = (x as f32 + rand_series.random_bilateral()) / WIDTH as f32;
                let v: f32 = ((HEIGHT - (y + start_height) - 1) as f32
                    + rand_series.random_bilateral()) / HEIGHT as f32;

                let r = cam.get_ray(u, v);
                col = col + color(&r, &scn, 0, &mut rand_series);
            }

            col = col / SAMPLES as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            let mut current = (bounds.0 * y + x) as usize * 3;

            img[current] = (col.x * 255.9) as u8;
            img[current + 1] = (col.y * 255.9) as u8;
            img[current + 2] = (col.z * 255.9) as u8;
        }
    }
}

fn main() {
    // Just some basic settings for the picture.
    let cores = num_cpus::get() as u32;
    /*
    assert!(
        HEIGHT % num_cpus::get() as u32 == 0,
        "please choose different image dimensions"
    );
*/

    println!("Image dimensions: {}x{}", WIDTH, HEIGHT);
    println!("Number of samples: {}", SAMPLES);

    // Create a scene to be rendered.
    let cam = camera::new();
    let mut scn = Vec::new();

    // This shit looks ugly as fuck, there must be a better way.
    scn.push(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material::Metal::new(Vec3::new(1.0, 0.2, 0.2), 0.0),
    ));
    scn.push(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material::Diffuse::new(0.5, 0.5, 0.5),
    ));

    // Time the program, so I can see if there are any performance issues.
    let begin_time = time::precise_time_ns();
    let mut pixels = vec![0; (WIDTH * HEIGHT * 3) as usize];

    let rows_per_band: u32 = (HEIGHT as f32 / cores as f32).round() as u32;
    {
        let bands: Vec<&mut [u8]> = pixels
            .chunks_mut((rows_per_band * WIDTH * 3) as usize)
            .collect();

        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let thread_scene = scn.clone();

                let bound_y = rows_per_band.min(HEIGHT - i as u32 * rows_per_band);
                let band_bounds = (WIDTH, bound_y);
                let start_height = i as u32 * (rows_per_band);
                println!("Start height {}: {}", i, start_height);
                println!("Bound Y {}: {}", i, bound_y);

                spawner
                    .spawn(move || render_band(thread_scene, cam, band, band_bounds, start_height));
            }
        });
    }

    /*
    let mut rand_series: math::RandomSeries = math::RandomSeries { state: 478927 };

    // Iterate over all the pixels in the image.
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut col = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..samples {
            let u: f32 = (x as f32 + rand_series.random_bilateral()) / width as f32;
            let v: f32 = ((height - y - 1) as f32 + rand_series.random_bilateral()) / height as f32;

            let r = cam.get_ray(u, v);
            col = col + color(&r, &scn, 0, &mut rand_series);
        }

        col = col / samples as f32;
        col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

        *pixel = image::Rgb([
            (col.x * 255.99) as u8,
            (col.y * 255.9) as u8,
            (col.z * 255.9) as u8,
        ]);
    }

    */
    // Print the final time.
    let end_time = time::precise_time_ns();
    println!(
        "Raytracing took: {} seconds!",
        ((end_time - begin_time) / 1000000) as f32 / 1000.0
    );

    // Save the output, the file to save to should become and argument later.
    let output = File::create("test.png").expect("Couldn't create file.");
    let encoder = PNGEncoder::new(output);
    encoder
        .encode(&pixels, WIDTH, HEIGHT, ColorType::RGB(8))
        .expect("Failed to encode image.");
}

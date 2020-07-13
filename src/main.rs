mod camera;
mod hitrecord;
mod material;
mod ray;
mod sphere;
mod traits;
mod vec3;

type Color = Vec3<vec3::Color>;
type Point3 = Vec3<vec3::Point3>;

use camera::*;
use hitrecord::*;
use material::*;
use rand::{random, Rng};
use ray::Ray;
use rayon::prelude::*;
use sphere::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::sync::Arc;
use traits::*;
use vec3::Vec3;

fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}

fn random_scene() -> Vec<Sphere> {
    let mut world = Vec::new();

    let ground_material = Arc::new(Lambertian::new((0.5, 0.5, 0.5).into()));
    world.push(Sphere::new((0, -1000, 0).into(), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );
            let dielectric = Arc::new(Dielectric::new(1.5));

            if (center - (4.0, 0.2, 0.0).into()).len() > 0.9 {
                let sphere_material: Arc<dyn Material + Send + Sync> = if choose_mat < 0.8 {
                    Arc::new(Lambertian::new(Color::random_unit() * Color::random_unit()))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    dielectric.clone()
                };
                world.push(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.push(Sphere::new((0, 1, 0).into(), 1.0, material1));
    let material2 = Arc::new(Lambertian::new((0.4, 0.2, 0.1).into()));
    world.push(Sphere::new((-4, 1, 0).into(), 1.0, material2));
    let material3 = Arc::new(Metal::new((0.7, 0.6, 0.5).into(), 0.0));
    world.push(Sphere::new((4, 1, 0).into(), 1.0, material3));

    world
}

fn main() {
    let mut file = BufWriter::with_capacity(8 * 1024 * 1024, File::create("image.ppm").unwrap());
    const MAX_DEPTH: u32 = 50;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const width: usize = 1024;
    const height: usize = (width as f64 / ASPECT_RATIO) as usize;
    const samples_per_pixel: u32 = 100;
    file.write_fmt(format_args!("P3\n{} {}\n255\n", width, height))
        .unwrap();

    //let glass = Arc::new(Dielectric::new(1.5));
    let mut world = random_scene();

    // world.push(Sphere::new(
    //     (0., -100.5, -1.).into(),
    //     100.0,
    //     Arc::new(Lambertian::new((0.8, 0.8, 0.0).into())),
    // ));

    // world.push(Sphere::new(
    //     (1, 0, -1).into(),
    //     0.5,
    //     Arc::new(Metal::new((0.8, 0.6, 0.2).into(), 0.3)),
    // ));

    // world.push(Sphere::new(
    //     (-1, 0, -1).into(),
    //     0.5,
    //     Arc::new(Dielectric::new(1.5)),
    // ));
    // world.push(Sphere::new(
    //     (-1, 0, -1).into(),
    //     -0.45,
    //     Arc::new(Dielectric::new(1.5)),
    // ));

    // world.push(Sphere::new(
    //     (0.0, 0.0, -1.0).into(),
    //     0.5,
    //     Arc::new(Lambertian::new((0.1, 0.2, 0.5).into())),
    // ));

    let lookfrom: Point3 = (13, 2, 3).into();
    let lookat: Point3 = (0, 0, 0).into();
    let vup: Point3 = (0, 1, 0).into();
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        Degrees::new(20.0),
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );
    // for j in (0..height).into_iter().rev() {
    //     if j % 50 == 0 {
    //         println!("Scanlines remaining: {:03}", j);
    //     }
    //     for i in 0..width {
    //         let mut pixel_color = Color::zeroed();
    //         for s in 0..samples_per_pixel {
    //             let u = (i as f64 + random::<f64>()) / (width - 1) as f64;
    //             let v = (j as f64 + random::<f64>()) / (height - 1) as f64;
    //             let ray = cam.ray(u, v);

    //             pixel_color += ray_color(&ray, &world, MAX_DEPTH);
    //         }

    //         write_color(&mut file, &pixel_color, samples_per_pixel);
    //     }
    // }
    let results = (0..height)
        .into_par_iter()
        .rev()
        .map(|j| {
            //eprintln!("Scanlines remaining: {:03}", j);
            let mut rng = rand::thread_rng();
            let mut v = Vec::with_capacity(samples_per_pixel as usize * width);
            for i in 0..width {
                let mut pixel_color = Color::zeroed();
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (height - 1) as f64;
                    let ray = cam.ray(u, v);

                    pixel_color += ray_color(&ray, &world, MAX_DEPTH);
                }
                v.push(process_color(pixel_color, samples_per_pixel));
            }
            v
        })
        .flatten()
        .collect::<Vec<(_, _, _)>>();
    for p in results.iter() {
        file.write_fmt(format_args!("{} {} {}\n", p.0, p.1, p.2))
            .unwrap();
    }
}

fn process_color(pixel: Color, samples_per_pixel: u32) -> (u32, u32, u32) {
    let mut r = pixel.x;
    let mut g = pixel.y;
    let mut b = pixel.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let ir = (clamp(r, 0.0, 0.999) * 256.0) as u32;
    let ig = (clamp(g, 0.0, 0.999) * 256.0) as u32;
    let ib = (clamp(b, 0.0, 0.999) * 256.0) as u32;
    (ir, ig, ib)
}

fn write_color(stream: &mut impl Write, pixel: &Color, samples_per_pixel: u32) {
    let mut r = pixel.x;
    let mut g = pixel.y;
    let mut b = pixel.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let ir = (clamp(r, 0.0, 0.999) * 256.0) as u32;
    let ig = (clamp(g, 0.0, 0.999) * 256.0) as u32;
    let ib = (clamp(b, 0.0, 0.999) * 256.0) as u32;

    stream
        .write_fmt(format_args!("{} {} {}\n", ir, ig, ib))
        .unwrap();
}

fn ray_color(ray: &Ray, world: &impl Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    let mut rec = HitRecord::default();
    if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let m = rec.material.as_ref().map(|x| x.clone()).unwrap();
        if m.scatter(ray, &mut rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::zeroed();
    }
    let unit_dir = ray.direction().unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    ((1.0 - t) * Color::new(1.0, 1.0, 1.0)) + (t * Color::new(0.5, 0.7, 1.0))
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + ((1.0 - r0) * (1.0 - cosine).powf(5.0))
}

fn clamp<T>(val: T, min: T, max: T) -> T
where
    T: Copy + std::cmp::PartialOrd,
{
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

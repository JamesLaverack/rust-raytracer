extern crate raytracer;
extern crate rand;
extern crate rayon;
extern crate indicatif;

use std::f64;

use raytracer::structures::vec3::Vec3;
use raytracer::structures::ray::Ray;

use raytracer::materials::Material;
use raytracer::materials::Scatterable;
use raytracer::materials::lambertian::Lambertian;
use raytracer::materials::metal::Metal;
use raytracer::materials::dielectric::Dielectric;

use raytracer::objects::sphere::Sphere;
use raytracer::objects::camera::Camera;
use raytracer::objects::lensing_world::LensingWorld;
use raytracer::objects::{Hittable, HitRecord};

use raytracer::io::write::gen_ppm;

use rand::Rng;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use indicatif::{ProgressBar, ProgressStyle};

fn color (r: &Ray, world: &Hittable, depth: u64) -> Vec3 {
    let mut rec: HitRecord = HitRecord::new();
    if world.intersect(&r, 0.0001, f64::MAX, &mut rec)  {
        let mut scattered: Ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        if depth > 0 && rec.material().scatter(r, &mut rec, &mut attenuation, &mut scattered) {
            return attenuation * color(&scattered, world, depth-1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction: Vec3 = r.direction().unit_vector();
        let t: f64 = 0.5*(unit_direction.y() + 1.0);
        return (1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0);
        // return Vec3::new(0.5, 0.5, 0.5);
    }
}

fn random_scene() -> LensingWorld {
    let mut rng = rand::thread_rng();

    let mut world: LensingWorld = LensingWorld::new();
    world.add_sphere(Sphere::new(Vec3::new(0.0, -1000.0, 0.0),
        1000.0, Material::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))));

    for a in -4..4 {
        for b in -4..4 {
            let choose_mat: f64 = rng.gen::<f64>();
            let center: Vec3 = Vec3::new(a as f64 + 0.9 * rng.gen::<f64>(),
                0.2, b as f64 + 0.9 * rng.gen::<f64>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {  // diffuse
                    world.add_sphere(Sphere::new(center, 0.2,
                        Material::Lambertian(Lambertian::new(Vec3::new(
                            rng.gen::<f64>()*rng.gen::<f64>(),
                            rng.gen::<f64>()*rng.gen::<f64>(),
                            rng.gen::<f64>()*rng.gen::<f64>())))));
                } else if choose_mat < 0.95 {  //metal
                    world.add_sphere(Sphere::new(center, 0.2,
                        Material::Metal(Metal::new(Vec3::new(
                            0.5 * (1.0 + rng.gen::<f64>()),
                            0.5 * (1.0 + rng.gen::<f64>()),
                            0.5 * (1.0 + rng.gen::<f64>())),
                            0.5 * rng.gen::<f64>()))));
                } else {  // dielectric
                    world.add_sphere(Sphere::new(center, 0.2,
                        Material::Dielectric(Dielectric::new(1.5))));
                }
            }

//            world.add_sphere(Sphere::new(Vec3::new(0.0, -2.0, 0.0), 1.0,
//                Material::Dielectric(Dielectric::new(1.5))));
            world.add_sphere(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0,
                Material::Lambertian(Lambertian::new(Vec3::new(0.4, 0.4, 0.1)))));
            world.add_sphere(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0,
                Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))));
        }
    }

    world
}

fn main() {
    let filename = "outputs/one_weekend.png".to_string();

    let nx: u64 = 1200;
    let ny: u64 = 800;
    let ns: u64 = 16;

    let cam: Camera = Camera::new(Vec3::new(13.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0), 20.0, (nx as f64)/(ny as f64), 0.1, 10.0);

    let world: LensingWorld = random_scene();

    let bar = ProgressBar::new(ny);
    bar.set_style(ProgressStyle::default_bar().template("[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]"));

    let scene: Vec<Vec<Vec3>> = (0..ny).into_par_iter().map(|y_rev| {
        let y: f64 = ny as f64 - y_rev as f64 - 1.0;
        let row: Vec<Vec3> = (0..nx).into_par_iter().map(|x| {
            let mut color_vector: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u: f64 = (x as f64 + rand::random::<f64>()) / nx as f64;
                let v: f64 = (y as f64 + rand::random::<f64>()) / ny as f64;
                let r: Ray = cam.get_ray(u, v);
                color_vector = color_vector + color(&r, &world, 10);
            }
            color_vector = color_vector/ns as f64;
            color_vector = 255.99*Vec3::new(color_vector.r().sqrt(), color_vector.g().sqrt(), color_vector.b().sqrt());
            color_vector.colorize();
            color_vector
        }).collect();
        bar.inc(1);
        row
    }).collect();

    bar.finish();

    gen_ppm(scene, filename);
}
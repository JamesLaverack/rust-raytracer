extern crate raytracer;
extern crate rand;

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
use raytracer::objects::{Hittable, HittableList, HitRecord};

use raytracer::io::write::gen_ppm;

use rand::Rng;

fn color (r: &Ray, world: &Hittable, depth: u64) -> Vec3 {
    let mut rec: HitRecord = HitRecord::new();
    if world.intersect(&r, 0.0, f64::MAX, &mut rec)  {
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

fn random_scene() -> HittableList {
    let n = 500;
    let mut rng = rand::thread_rng();
    
    let mut list: HittableList = HittableList::new();
    list.add_sphere(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 
        1000.0, Material::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen::<f64>();
            let center: Vec3 = Vec3::new(a as f64 + 0.9 * rng.gen::<f64>(), 
                0.2, b as f64 + 0.9 * rng.gen::<f64>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {  // diffuse
                    list.add_sphere(Sphere::new(center, 0.2, 
                        Material::Lambertian(Lambertian::new(Vec3::new(
                            rng.gen::<f64>()*rng.gen::<f64>(), 
                            rng.gen::<f64>()*rng.gen::<f64>(), 
                            rng.gen::<f64>()*rng.gen::<f64>())))));
                } else if choose_mat < 0.95 {  //metal
                    list.add_sphere(Sphere::new(center, 0.2, 
                        Material::Metal(Metal::new(Vec3::new(
                            0.5 * (1.0 + rng.gen::<f64>()), 
                            0.5 * (1.0 + rng.gen::<f64>()), 
                            0.5 * (1.0 + rng.gen::<f64>())),
                            0.5 * rng.gen::<f64>()))));
                } else {  // dielectric
                    list.add_sphere(Sphere::new(center, 0.2, 
                        Material::Dielectric(Dielectric::new(1.5))));
                }
            }

            list.add_sphere(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, 
                Material::Dielectric(Dielectric::new(1.5))));
            list.add_sphere(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, 
                Material::Lambertian(Lambertian::new(Vec3::new(0.4, 0.4, 0.1)))));
            list.add_sphere(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, 
                Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))));
        }
    }

    list
}

fn main() {
    let filename = "outputs/basic_sphere.ppm".to_string();

    let nx: u64 = 480;
    let ny: u64 = 320;
    let ns: u64 = 10;

    let cam: Camera = Camera::new(Vec3::new(13.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 0.0), 
    Vec3::new(0.0, 1.0, 0.0), 20.0, (nx as f64)/(ny as f64), 0.1, 10.0);

    let mut world: HittableList = random_scene();

    let mut scene: Vec<Vec<Vec3>> = Vec::new();

    let mut rng = rand::thread_rng();

    for y in (0..ny).rev() {
        let mut row: Vec<Vec3> = Vec::new();
        for x in 0..nx {
            let mut color_vector: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u: f64 = (x as f64 + rng.gen::<f64>()) / nx as f64;
                let v: f64 = (y as f64 + rng.gen::<f64>()) / ny as f64;
                let r: Ray = cam.get_ray(u, v);
                color_vector = color_vector + color(&r, &world, 10);
            }
            color_vector = color_vector/ns as f64;
            color_vector = 255.99*Vec3::new(color_vector.r().sqrt(), color_vector.g().sqrt(), color_vector.b().sqrt());
            color_vector.colorize();
            row.push(color_vector);
        }
        scene.push(row);
    }
    gen_ppm(scene, filename);
}
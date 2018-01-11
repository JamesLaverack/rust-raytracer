use structures::vec3::Vec3;
use structures::ray::Ray;

use materials::Material;

use super::Hittable;
use super::HitRecord;
use super::sphere::Sphere;

use std::f64;

pub struct LensingWorld {
    spheres: Vec<Sphere>,
    gravity_point: Vec3,
    gravity_strength: f64,
}

impl LensingWorld {
    pub fn new(gravity_point: Vec3, gravity_strength: f64) -> LensingWorld {
        let spheres: Vec<Sphere> = Vec::new();
        LensingWorld { spheres, gravity_point, gravity_strength }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn size(&self) -> usize {
        self.spheres.len()
    }
}

impl Hittable for LensingWorld {
    fn intersect(&self, start_ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let max_step: f64 = 0.1;
        let max_iter: u32 = 500;
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        let mut ray: Ray = start_ray.clone();
        for j in 0..max_iter {
            for i in 0..self.size() {
                if self.spheres[i].intersect(&ray, t_min, closest_so_far, &mut temp_rec) && temp_rec.t < max_step {
                    hit_anything = true;
                    closest_so_far = temp_rec.t;
                    rec.p = temp_rec.p;
                    rec.t = temp_rec.t;
                    rec.normal = temp_rec.normal;
                    rec.material = temp_rec.material;
                }
            }
            if hit_anything {
                return hit_anything;
            }

            // March ray forwards
            ray = ray.apply_gravity_and_march(&self.gravity_point, max_step * self.gravity_strength, max_step);

            // We hit the black hole, eat the ray
            if ray.direction() == Vec3::new(0.0, 0.0, 0.0) {
                return false
            }
        }
        false
    }
}
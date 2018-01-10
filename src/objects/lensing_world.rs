use structures::vec3::Vec3;
use structures::ray::Ray;

use materials::Material;

use super::Hittable;
use super::HitRecord;
use super::sphere::Sphere;

use std::f64;

pub struct LensingWorld {
    spheres: Vec<Sphere>,
    weights: Vec<Vec3>,
}

impl LensingWorld {
    pub fn new() -> LensingWorld {
        let spheres_list: Vec<Sphere> = Vec::new();
        let weights_list: Vec<Vec3> = Vec::new();
        LensingWorld { spheres: spheres_list, weights: weights_list }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn add_weight(&mut self, weight: Vec3) {
        self.weights.push(weight);
    }

    pub fn size(&self) -> usize {
        self.spheres.len()
    }
}

impl Hittable for LensingWorld {
    fn intersect(&self, start_ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let max_step: f64 = 0.1;
        let max_iter: u32 = 1000;
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

            ray = ray.apply_gravity_and_march(&Vec3::new(-1.0, 1.0, 0.0), max_step * 0.15, max_step);
            //ray = ray.march(max_step);
            if ray.direction() == Vec3::new(0.0, 0.0, 0.0) {
                return false
            }
        }
        false
    }
}
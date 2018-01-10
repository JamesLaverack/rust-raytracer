use structures::vec3::Vec3;
use structures::ray::Ray;

use materials::Material;

use super::Hittable;
use super::HitRecord;
use super::sphere::Sphere;

use std::f64;

pub struct LensingWorld {
    spheres: Vec<Sphere>,
}

impl LensingWorld {
    pub fn new() -> LensingWorld {
        let spheres_list: Vec<Sphere> = Vec::new();
        LensingWorld{spheres: spheres_list}
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn size(&self) -> usize {
        self.spheres.len()
    }
}

impl Hittable for LensingWorld {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        for i in 0..self.size() {
            if self.spheres[i].intersect(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.normal = temp_rec.normal;
                rec.material = temp_rec.material;
            }
        }
        hit_anything
    }
}
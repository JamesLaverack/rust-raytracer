use structures::vec3::Vec3;

#[derive(Clone, Debug)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray {a: a, b: b}
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.a + t * self.b 
    }

    pub fn march(&self, m: f64) -> Ray {
        Ray::new(self.point_at_parameter(m), self.direction())
    }

    pub fn apply_gravity_and_march(self, gravity_point: &Vec3, strength: f64, m: f64) -> Ray  {
        let direction_of_force = (*gravity_point - self.origin()).unit_vector();
        let distance = gravity_point.distance(&self.origin()) - 0.5;
        if distance >= 0.0 {
            let magnitude = (1.0 / (distance * distance)) * strength;
            let new_ray_direction = ((direction_of_force * magnitude) + (self.direction() * (1.0 - magnitude))).unit_vector();
            return Ray::new(self.point_at_parameter(m), new_ray_direction);
        } else {
            // we're inside the black hole
            return Ray::new(Vec3::new(1000.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        }
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Ray) -> bool {
        self.origin() == other.origin() && self.direction() == other.direction()
    }
}

#[test]
fn test_origin() {
    let origin: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let direction: Vec3 = Vec3::new(3.0, 4.0, 1.0);
    let ray: Ray = Ray::new(origin, direction);
    assert_eq!(ray.origin(), origin);
}

#[test]
fn test_march() {
    let origin: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let direction: Vec3 = Vec3::new(3.0, 4.0, 1.0);
    let mut ray: Ray = Ray::new(origin, direction);
    ray = ray.march(1.0);
    assert_ne!(ray.origin(), origin);
}

#[test]
fn test_direction() {
    let origin: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let direction: Vec3 = Vec3::new(3.0, 4.0, 1.0);
    let ray: Ray = Ray::new(origin, direction);
    assert_eq!(ray.direction(), direction);
}

#[test]
fn test_param() {
    let origin: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let direction: Vec3 = Vec3::new(3.0, 4.0, 1.0);
    let ray: Ray = Ray::new(origin, direction);
    let point: Vec3 = Vec3::new(7.0, 10.0, 5.0);
    assert_eq!(ray.point_at_parameter(2.0), point);
}
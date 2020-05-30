use crate::ray::Ray;
use crate::types::Vec3;
use crate::types::VecValueType;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: VecValueType,
}

impl Sphere {
    pub fn new(x: VecValueType, y: VecValueType, z: VecValueType, r: VecValueType) -> Self {
        Sphere {
            center: Vec3::new(x, y, z),
            radius: r,
        }
    }

    pub fn hit(&self, ray: Ray) -> bool {
        let oc = ray.origin() - self.center;
        let b = 2. * oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4. * c; // direction is unit vector => a = 1.0
        discriminant > 0.
    }
}

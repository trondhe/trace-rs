use crate::ray::{Ray, RayHit};
use crate::types::Vec3;
use crate::types::VecValueType;

pub trait Hitable {
    fn hit(&self, ray: Ray) -> Option<RayHit>;
}
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
}

impl Hitable for Sphere {
    fn hit(&self, ray: Ray) -> Option<RayHit> {
        let oc = ray.origin() - self.center;
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - c;
        if discriminant < 0.0 {
            return None; // No real solutions, ray did not hit
        }
        let d_sqrt = discriminant.sqrt();
        let t = {
            let t1 = -1.0 * b - d_sqrt;
            if t1 > 0.0 {
                // t1 is always the shorter/closer distance, return if larger than 0.0
                t1
            } else {
                -1.0 * b + d_sqrt
            }
        };
        if t < 0.0 {
            return None;
        }
        let p = ray.p(t);
        let normal = ((p - self.center) / self.radius).normalize();
        Some(RayHit { t, p, normal })
    }
}

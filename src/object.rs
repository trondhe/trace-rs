use crate::ray::{Ray, RayHit};
use crate::types::Vec3;

pub trait Hitable {
    fn hit(&self, ray: Ray) -> Option<RayHit>;
}

pub struct HitableList {
    data: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push<T: Hitable + 'static>(&mut self, hitable: T) {
        self.data.push(Box::new(hitable));
    }

    pub fn find_foreground_hit(&self, ray: Ray) -> Option<RayHit> {
        let mut nearest_hit: Option<RayHit> = None;
        for obj in &self.data {
            if let Some(hit) = obj.hit(ray) {
                if nearest_hit.is_none() {
                    nearest_hit = Some(hit);
                    continue;
                }
                if nearest_hit.unwrap().t > hit.t {
                    nearest_hit = Some(hit);
                }
            }
        }
        nearest_hit
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, r: f32) -> Self {
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
        let normal = (p - self.center).normalize();
        Some(RayHit { t, p, normal })
    }
}

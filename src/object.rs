use crate::ray::Ray;
use crate::types::{TraceValueType, Vec3};

pub trait Hitable {
    fn hit(&self, ray: Ray) -> Option<(RayHit, ObjectType)>;
}

pub struct HitableList {
    data: Vec<Box<dyn Hitable + Sync>>,
}

impl HitableList {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, hitable: Box<dyn Hitable + Sync>) {
        self.data.push(hitable);
    }

    pub fn list(&self) -> &Vec<Box<dyn Hitable + Sync>> {
        &self.data
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: TraceValueType,
    object_type: ObjectType,
}

impl Sphere {
    pub fn new(
        x: TraceValueType,
        y: TraceValueType,
        z: TraceValueType,
        r: TraceValueType,
        object_type: ObjectType,
    ) -> Self {
        Sphere {
            center: Vec3::new(x, y, z),
            radius: r,
            object_type,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: Ray) -> Option<(RayHit, ObjectType)> {
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

        let hit = RayHit { t, p, normal };
        Some((hit, self.object_type))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Surface {
    pub roughness: TraceValueType,
    pub attenuation: Vec3,
}

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub intensity: Vec3,
}

#[derive(Debug, Copy, Clone)]
pub enum ObjectType {
    Surface(Surface),
    LightSource(Light),
}
#[derive(Debug, Copy, Clone)]
pub struct RayHit {
    pub t: TraceValueType,
    pub p: Vec3,
    pub normal: Vec3,
}

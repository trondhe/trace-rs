use crate::object::Hitable;
use crate::ray::Ray;
use crate::ray::RayHit;

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

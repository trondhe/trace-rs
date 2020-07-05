use rand::Rng;

use crate::object::{HitableList, ObjectType, RayHit, Surface};

use crate::ray::Ray;
use crate::types::{TraceValueType, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Tracer {
    bounces_max: usize,
}

impl Tracer {
    pub fn new(bounces_max: usize) -> Tracer {
        Tracer { bounces_max }
    }

    pub fn trace(&self, ray: &Ray, world: &HitableList) -> Vec3 {
        let mut bounces: usize = 0;
        let mut light_intensity = Vec3::new(1., 1., 1.);
        let mut current_ray = *ray;
        while bounces < self.bounces_max {
            let maybe_hit = self.find_foreground_hit(&current_ray, world);
            match maybe_hit {
                None => {
                    // Background
                    let v = (current_ray.direction().y.sin() + 1.0) / 2.;
                    let background_r = 1.0 - v + v * 0.5;
                    let background_g = 1.0 - v + v * 0.7;
                    let background_b = 1.0 - v + v * 1.0;
                    let background = Vec3::new(background_r, background_g, background_b);
                    light_intensity.x *= background.x;
                    light_intensity.y *= background.y;
                    light_intensity.z *= background.z;
                    break;
                }
                Some((hit, hit_type)) => {
                    self.mutate_color_from(&hit_type, &mut light_intensity);
                    match hit_type {
                        ObjectType::LightSource(_) => break,
                        ObjectType::Surface(surface) => {
                            current_ray =
                                self.calculate_bounced_ray_from(&current_ray, hit, surface);
                            bounces += 1;
                        }
                    }
                }
            }
        }
        light_intensity
    }

    fn mutate_color_from(&self, hit_type: &ObjectType, colour: &mut Vec3) {
        match hit_type {
            ObjectType::Surface(surface) => {
                colour.x *= surface.attenuation.x;
                colour.y *= surface.attenuation.y;
                colour.z *= surface.attenuation.z;
            }
            ObjectType::LightSource(light) => {
                colour.x *= light.intensity.x;
                colour.y *= light.intensity.y;
                colour.z *= light.intensity.z;
            }
        }
    }

    fn calculate_bounced_ray_from(&self, ray_incident: &Ray, hit: RayHit, surface: Surface) -> Ray {
        let mut rng = rand::thread_rng();
        let rand = rng.gen::<TraceValueType>() * 2. - 1.; // 0-1

        // Comparing [0, 1] random to [0, 1] roughness number.
        // A roughness of 1 will always result in a diffuse bounce
        if surface.roughness >= rand {
            self.diffuse(hit.p, hit.normal)
        } else {
            self.reflect(&ray_incident, hit.p, hit.normal)
        }
    }

    fn reflect(&self, ray: &Ray, hitpoint: Vec3, normal: Vec3) -> Ray {
        // Reflected = vector_incoming - 2 * (vector_incoming * normal) * normal
        Ray::new(
            hitpoint,
            ray.direction() - 2. * (ray.direction().dot(&normal)) * normal,
        )
    }

    fn diffuse(&self, hitpoint: Vec3, normal: Vec3) -> Ray {
        let mut rng = rand::thread_rng();
        let j_scale = rng.gen::<TraceValueType>() * 2. - 1.; // 0 - 1 range
        let k_scale = rng.gen::<TraceValueType>() * 2. - 1.; // 0 - 1 range
        let i = normal;
        let j = if (i.x, i.y, i.z) == (0., 0., 1.) {
            // i == [x,y,z] => j == [-y, x, 0] gives [0, 0, 0] in this given case
            Vec3::new(1., 0., 0.)
        } else {
            // i == [x,y,z] => j == [-y, x, 0] as a simple solution
            Vec3::new(-i.y, i.x, 0.)
        };
        let k = i.cross(&j);
        let direction = i + j_scale * j + k_scale * k;
        Ray::new(hitpoint, direction)
    }

    fn find_foreground_hit(&self, ray: &Ray, world: &HitableList) -> Option<(RayHit, ObjectType)> {
        let mut nearest_hit = None;
        for obj in world.list() {
            if let Some((hit, hit_type)) = obj.hit(*ray) {
                if nearest_hit.is_none() {
                    nearest_hit = Some((hit, hit_type));
                    continue;
                }
                if nearest_hit.unwrap().0.t > hit.t {
                    nearest_hit = Some((hit, hit_type));
                }
            }
        }
        nearest_hit
    }
}

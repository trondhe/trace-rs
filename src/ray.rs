use na::Vector3;

pub type PointValueType = f32;
pub type Point = Vector3<PointValueType>;
pub struct Ray {
    pub a: Point,
    pub b: Point,
}
#[allow(dead_code)]
impl Ray {
    pub fn new() -> Self {
        Ray {
            a: Vector3::new(0., 0., 0.),
            b: Vector3::new(0., 0., 0.),
        }
    }

    pub fn with_value(origin: Point, direction: Point) -> Self {
        Ray {
            a: origin,
            b: direction,
        }
    }

    pub fn point_at_parameter(&self, t: PointValueType) -> Point {
        self.a + t * self.b
    }

    pub fn origin(&self) -> Point {
        self.a
    }

    pub fn direction(&self) -> Point {
        self.b
    }
}

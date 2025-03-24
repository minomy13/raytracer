use uuid::Uuid;

use crate::ray::{intersection::Intersection, Ray};

pub mod sphere;

pub trait Body {
    fn new() -> Self;
    fn intersect(&self, ray: Ray) -> Option<[Intersection; 2]>;
    fn get_id(&self) -> Uuid;
}

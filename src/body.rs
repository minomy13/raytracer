use uuid::Uuid;

use crate::{
    matrix::Matrix,
    ray::{intersection::Intersection, Ray},
};

pub mod sphere;

pub trait Body {
    fn new() -> Self;
    fn intersect(&self, ray: Ray) -> Option<[Intersection; 2]>;
    fn transform(&self, by: Matrix<4, 4>) -> Self;
    fn get_id(&self) -> Uuid;
    fn get_transformation(&self) -> Matrix<4, 4>;
}

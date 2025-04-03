use uuid::Uuid;

use crate::{
    material::Material,
    matrix::Matrix,
    ray::{intersection::Intersection, Ray},
    tuple::Tuple,
};

pub mod sphere;

pub trait Body {
    fn new() -> Self;
    fn intersect(&self, ray: &Ray) -> Option<[Intersection; 2]>;
    fn transform(&self, by: Matrix<4, 4>) -> Self;
    fn normal_at(&self, point: Tuple) -> Tuple;
    fn get_id(&self) -> Uuid;
    fn get_transformation(&self) -> Matrix<4, 4>;
    fn get_material(&self) -> Material;
    fn set_material(&self, material: Material) -> Self;
}

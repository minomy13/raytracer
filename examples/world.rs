use std::f64::consts::PI;

use raytracer::{body::{sphere::Sphere, Body}, camera::Camera, color::Color, light::{Light, PointLight}, material::Material, matrix::{transformation::Axis, Matrix}, point, vector, world::World, tuple::Tuple};

fn main() {
    let floor = Sphere::new().transform(Matrix::scaling_matrix(10.0, 0.01, 10.0)).set_material(Material::default().set_color(Color::new(1.0, 0.9, 0.9)).set_specular(0.0));
    let left_wall = Sphere::new().transform(Matrix::translation_matrix(0.0, 0.0, 5.0).rotate(Axis::Y, -PI/4.0).rotate(Axis::X, PI/2.0).scale(10.0, 0.01, 10.0)).set_material(floor.get_material());
    let right_wall = Sphere::new().transform(Matrix::translation_matrix(0.0, 0.0, 5.0).rotate(Axis::Y, PI/4.0).rotate(Axis::X, PI/2.0).scale(10.0, 0.01, 10.0)).set_material(floor.get_material());

    let middle = Sphere::new().transform(Matrix::translation_matrix(-0.5, 1.0, 0.5)).set_material(Material::default().set_color(Color::new(0.1, 1.0, 0.5)).set_diffuse(0.7).set_specular(0.3));
    let right = Sphere::new().transform(Matrix::translation_matrix(1.5, 0.5, -0.5).scale(0.5, 0.5, 0.5)).set_material(Material::default().set_color(Color::new(0.5, 1.0, 0.1)).set_diffuse(0.7).set_specular(0.3));
    let left = Sphere::new().transform(Matrix::translation_matrix(-1.5, 0.33, -0.75).scale(0.33, 0.33, 0.33)).set_material(Material::default().set_color(Color::new(1.0, 0.8, 0.1)).set_diffuse(0.7).set_specular(0.3));

    // TODO: find better style!
    let mut world = World::new();
    let world = world.add_light(PointLight::new(point!(-10, 10, -10), Color::white()));
    let world = world.add_object(floor).add_object(left_wall).add_object(right_wall).add_object(middle).add_object(right).add_object(left);

    let mut c = Camera::new(1000, 500, PI/3.0);
    let c = c.transform(Matrix::view_transform_matrix(point!(0, 1.5, -5), point!(0, 1, 0), vector!(0, 1, 0)));
    let _ = c.render(world).save();
}
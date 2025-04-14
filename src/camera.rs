use crate::{point, 
    canvas::Canvas,
    matrix::Matrix,
    ray::Ray,
    tuple::Tuple,
    world::World,
};

pub struct Camera {
    hsize: usize,
    vsize: usize,
    half_width: f64,
    half_height: f64,
    field_of_view: f64,
    pixel_size: f64,
    transformation: Matrix<4, 4>,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2f64).tan();
        let aspect = (hsize as f64) / (vsize as f64);
        let half_width;
        let half_height;

        if aspect >= 1f64 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2f64) / (hsize as f64);

        Self {
            hsize,
            vsize,
            half_width,
            half_height,
            field_of_view,
            pixel_size,
            transformation: Matrix::identity_matrix(),
        }
    }

    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let inverse_transformation = self.transformation.inverse();
        let pixel = inverse_transformation * point!(world_x, world_y, -1);
        let origin = inverse_transformation * Tuple::point_origin();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn transform(&mut self, transformation: Matrix<4, 4>) -> &mut Self {
        self.transformation = self.transformation * transformation;
        self
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                image.write_pixel(x, y, color);
            }
        }

        image
    }

    pub fn set_transformation(&mut self, transformation: Matrix<4, 4>) -> &mut Self {
        self.transformation = transformation;
        self
    }

    pub fn get_hsize(&self) -> usize {
        self.hsize
    }

    pub fn get_vsize(&self) -> usize {
        self.vsize
    }

    pub fn get_field_of_view(&self) -> f64 {
        self.field_of_view
    }

    pub fn get_pixel_size(&self) -> f64 {
        self.pixel_size
    }

    pub fn get_transform(&self) -> Matrix<4, 4> {
        self.transformation
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{vector, point, 
        color::Color,
        matrix::{transformation::Axis, Matrix},
        tuple::Tuple,
        utils::assert_f64_eq,
        world::World,
    };

    use super::Camera;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let c = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(c.get_hsize(), hsize);
        assert_eq!(c.get_vsize(), vsize);
        assert_f64_eq!(c.get_field_of_view(), PI / 2.0);
        assert_eq!(c.get_transform(), Matrix::identity_matrix())
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert_f64_eq!(c.get_pixel_size(), 0.01)
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert_f64_eq!(c.get_pixel_size(), 0.01)
    }

    #[test]
    fn constructing_ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.get_origin(), Tuple::point_origin());
        assert_eq!(r.get_direction(), vector!(0, 0, -1))
    }

    #[test]
    fn constructing_ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.get_origin(), Tuple::point_origin());
        assert_eq!(r.get_direction(), vector!(0.66519, 0.33259, -0.66851))
    }

    #[test]
    fn constructing_ray_when_camera_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        let c = c.transform(Matrix::rotation_matrix(Axis::Y, PI / 4.0).translate(0.0, -2.0, 5.0));
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.get_origin(), point!(0, 2, -5));
        assert_eq!(
            r.get_direction(),
            vector!(2f64.sqrt() / 2.0, 0, -2f64.sqrt() / 2.0)
        )
    }

    #[test]
    fn rendering_world_with_camera() {
        let w = World::default();
        let from = point!(0, 0, -5);
        let to = Tuple::point_origin();
        let up = vector!(0, 1, 0);
        // TODO: find better style
        let mut c = Camera::new(11, 11, PI / 2.0);
        let c = c.transform(Matrix::view_transform_matrix(from, to, up));
        let image = c.render(&w);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855))
    }
}

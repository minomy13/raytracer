use crate::{color::Color, tuple::Tuple};

pub trait Light {
    fn new(position: Tuple, intensity: Color) -> Self;
    fn get_intensity(&self) -> Color;
    fn get_position(&self) -> Tuple;
}

pub struct PointLight {
    intensity: Color,
    position: Tuple,
}

impl Light for PointLight {
    fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            intensity,
            position,
        }
    }

    fn get_intensity(&self) -> Color {
        self.intensity
    }

    fn get_position(&self) -> Tuple {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, light::Light, tuple::Tuple};

    use super::PointLight;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Tuple::new_point(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);
        assert_eq!(light.intensity, intensity);
        assert_eq!(light.position, position)
    }
}

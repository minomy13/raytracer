use crate::{color::Color, light::Light, tuple::Tuple};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shinyness: f64,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shinyness: f64) -> Self {
        if !(0f64..=1f64).contains(&ambient) {
            panic!("Ambient out of range (0..=1)");
        }
        if !(0f64..=1f64).contains(&diffuse) {
            panic!("Diffuse out of range (0..=1)");
        }
        if !(0f64..=1f64).contains(&specular) {
            panic!("Specular out of range (0..=1)");
        }
        if !(10f64 <= shinyness) {
            panic!("Shinyness out of range (>=10)");
        }

        Self {
            color,
            ambient,
            diffuse,
            specular,
            shinyness,
        }
    }

    pub fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shinyness: 200.0,
        }
    }

    // TODO: optimize
    pub fn lighting(
        &self,
        light: &dyn Light,
        position: Tuple,
        eyevector: Tuple,
        normalvector: Tuple,
    ) -> Color {
        let effective_color = self.color * light.get_intensity();
        let lightv = (light.get_position() - position).normalize();
        let ambient = effective_color * self.ambient;
        let diffuse: Color;
        let specular: Color;

        // determine if light is behind surface
        let light_dot_normal = lightv.dot(normalvector);
        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // determine if reflects towards eye
            let reflectv = -lightv.reflect_at(normalvector);
            let reflect_dot_eye = reflectv.dot(eyevector);
            if reflect_dot_eye < 0.0 {
                specular = Color::black()
            } else {
                let factor = reflect_dot_eye.powf(self.shinyness);
                specular = light.get_intensity() * self.specular * factor
            }
        }

        ambient + diffuse + specular
    }

    pub fn set_color(&self, color: Color) -> Self {
        Self { color, ..*self }
    }

    pub fn set_ambient(&self, ambient: f64) -> Self {
        if !(0f64..=1f64).contains(&ambient) {
            panic!("Ambient out of range (0..=1)");
        }
        Self { ambient, ..*self }
    }

    pub fn set_diffuse(&self, diffuse: f64) -> Self {
        if !(0f64..=1f64).contains(&diffuse) {
            panic!("Diffuse out of range (0..=1)");
        }
        Self { diffuse, ..*self }
    }

    pub fn set_specular(&self, specular: f64) -> Self {
        if !(0f64..=1f64).contains(&specular) {
            panic!("Specular out of range (0..=1)");
        }
        Self { specular, ..*self }
    }

    pub fn set_shinyness(&self, shinyness: f64) -> Self {
        if !(10f64 <= shinyness) {
            panic!("Shinyness out of range (>=10)");
        }
        Self { shinyness, ..*self }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_ambient(&self) -> f64 {
        self.ambient
    }

    pub fn get_diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn get_specular(&self) -> f64 {
        self.specular
    }

    pub fn get_shinyness(&self) -> f64 {
        self.shinyness
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color,
        light::{Light, PointLight},
        tuple::Tuple,
    };

    use super::Material;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(m.get_color(), Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.get_ambient(), 0.1);
        assert_eq!(m.get_diffuse(), 0.9);
        assert_eq!(m.get_specular(), 0.9);
        assert_eq!(m.get_shinyness(), 200.0)
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::default();
        let position = Tuple::point_origin();
        let eyev = Tuple::new_vec(0.0, 0.0, -1.0);
        let normalv = eyev.clone();
        let light = PointLight::new(Tuple::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9))
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45deg() {
        let m = Material::default();
        let position = Tuple::point_origin();
        let eyev = Tuple::new_vec(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = Tuple::new_vec(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0))
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_light_offset_45deg() {
        let m = Material::default();
        let position = Tuple::point_origin();
        let eyev = Tuple::new_vec(0.0, 0.0, -1.0);
        let normalv = eyev.clone();
        let light = PointLight::new(
            Tuple::new_point(0.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );
        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364))
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection() {
        let m = Material::default();
        let position = Tuple::point_origin();
        let eyev = Tuple::new_vec(0.0, -2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = Tuple::new_vec(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Tuple::new_point(0.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );
        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364))
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = Material::default();
        let position = Tuple::point_origin();
        let eyev = Tuple::new_vec(0.0, 0.0, -1.0);
        let normalv = eyev.clone();
        let light = PointLight::new(Tuple::new_point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1))
    }
}

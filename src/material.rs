use crate::color::Color;
use crate::float_eq;
use crate::light::PointLight;
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lighting(&self, light: PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
        let black = Color::new(0.0, 0.0, 0.0);
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv * normalv;
        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (black, black)
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv * eyev;
            let specular = if reflect_dot_eye <= 0.0 {
                black
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity * self.specular * factor
            };
            (diffuse, specular)
        };
        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::new()
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && float_eq(self.ambient, other.ambient)
            && float_eq(self.diffuse, other.diffuse)
            && float_eq(self.specular, other.specular)
            && float_eq(self.shininess, other.shininess)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::light::PointLight;
    use crate::material::Material;
    use crate::tuple::Tuple;

    #[test]
    fn the_default_material() {
        let m = Material::new();

        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::new();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(light, position, eyev, normalv);
        let expected = Color::new(1.9, 1.9, 1.9);

        assert_eq!(result, expected);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degrees() {
        let m = Material::new();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(light, position, eyev, normalv);
        let expected = Color::new(1.0, 1.0, 1.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::new();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Tuple::new_point(0.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );
        let result = m.lighting(light, position, eyev, normalv);
        let expected = Color::new(0.7364, 0.7364, 0.7364);

        assert_eq!(result, expected);
    }

    #[test]
    fn lighting_with_the_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Tuple::new_point(0.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );
        let result = m.lighting(light, position, eyev, normalv);
        let expected = Color::new(1.6364, 1.6364, 1.6364);

        assert_eq!(result, expected);
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::new();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::new_point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(light, position, eyev, normalv);
        let expected = Color::new(0.1, 0.1, 0.1);

        assert_eq!(result, expected);
    }
}

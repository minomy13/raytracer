use crate::{body::Body, tuple::Tuple};

use super::Ray;

#[derive(Clone, Copy)]
pub(crate) struct Intersection<'a> {
    t: f64,
    object: &'a dyn Body,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a dyn Body) -> Self {
        Self { t, object }
    }

    pub fn prepare_computations(&self, ray: &Ray) -> Computations {
        let point = ray.position(self.t);
        let normalv = self.object.normal_at(point);
        let eyev = -ray.get_direction();
        let inside = normalv.dot(eyev) < 0f64;
        Computations {
            // TODO: copy just for convenience, consider ref
            t: self.t,
            inside,
            object: self.object,
            point,
            eyev,
            normalv: if inside { -normalv } else { normalv },
        }
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }

    pub fn get_object(&self) -> &dyn Body {
        self.object
    }

    pub fn find_hit(intersections: &'a mut Vec<Intersection>) -> Option<&'a Intersection<'a>> {
        // TODO: sort necessary here? should already be sorted
        intersections.sort_by(|a, b| a.get_t().total_cmp(&b.get_t()));
        intersections.iter().find(|elm| elm.get_t() >= 0.0)
    }
}

// TODO: is public right approach here?
pub(crate) struct Computations<'a> {
    pub t: f64,
    pub inside: bool,
    pub object: &'a dyn Body,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
}

// TODO: check page 64 - aggregating; substitute with `vec![]` at the moment

#[cfg(test)]
mod tests {
    use crate::{vector, point, 
        body::{sphere::Sphere, Body},
        ray::Ray,
        tuple::Tuple,
        utils::assert_f64_eq,
    };

    use super::Intersection;

    // TESTME
    macro_rules! intersection_eq {
        ($int1:expr, $int2:expr) => {
            ($int1.get_t() == $int2.get_t())
                && ($int1.get_object().get_id() == $int2.get_object().get_id())
        };
    }

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.get_t(), 3.5);
        assert_eq!(i.get_object().get_id(), s.get_id())
    }

    #[test]
    fn hit_when_all_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let mut xs = vec![i1, i2];
        let i = Intersection::find_hit(&mut xs);
        assert!(intersection_eq!(i.unwrap(), i1))
    }

    #[test]
    fn hit_when_some_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let mut xs = vec![i1, i2];
        let i = Intersection::find_hit(&mut xs);
        assert!(intersection_eq!(i.unwrap(), i2))
    }

    #[test]
    fn hit_when_all_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let mut xs = vec![i1, i2];
        let i = Intersection::find_hit(&mut xs);
        assert!(i.is_none())
    }

    #[test]
    fn hit_always_lowest_nonnegative() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let mut xs = vec![i1, i2, i3, i4];
        let i = Intersection::find_hit(&mut xs);
        assert!(intersection_eq!(i.unwrap(), i4))
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(&r);
        assert_f64_eq!(comps.t, i.get_t());
        assert_eq!(comps.point, point!(0, 0, -1));
        assert_eq!(comps.eyev, vector!(0, 0, -1));
        assert_eq!(comps.normalv, vector!(0, 0, -1))
    }

    #[test]
    fn hit_when_intersection_occurs_outside() {
        let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(&r);
        assert!(!comps.inside)
    }

    #[test]
    fn hit_when_intersection_occurs_inside() {
        let r = Ray::new(Tuple::point_origin(), vector!(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.point, point!(0, 0, 1));
        assert_eq!(comps.eyev, vector!(0, 0, -1));
        assert!(comps.inside);
        // inverted because inside
        assert_eq!(comps.normalv, vector!(0, 0, -1))
    }
}

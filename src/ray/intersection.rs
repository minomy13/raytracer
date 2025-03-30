use uuid::Uuid;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Intersection {
    t: f64,
    // TODO: consider switching to references
    object_id: Uuid,
}

impl Intersection {
    pub fn new(t: f64, object_id: Uuid) -> Self {
        Self { t, object_id }
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }

    // TODO: consider better return type
    pub fn get_object_id(&self) -> Uuid {
        self.object_id
    }
}

pub fn find_hit<'a>(intersections: &'a mut Vec<Intersection>) -> Option<&'a Intersection> {
    intersections.sort_by(|a, b| a.get_t().total_cmp(&b.get_t()));
    intersections.iter().find(|elm| elm.get_t() >= 0.0)
}

// TODO: check page 64 - aggregating; substitute with `vec![]` at the moment

#[cfg(test)]
mod tests {
    use crate::body::{sphere::Sphere, Body};

    use super::{find_hit, Intersection};

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, s.get_id());
        assert_eq!(i.get_t(), 3.5);
        assert_eq!(i.get_object_id(), s.get_id())
    }

    #[test]
    fn hit_when_all_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s.get_id());
        let i2 = Intersection::new(2.0, s.get_id());
        let mut xs = vec![i1, i2];
        let i = find_hit(&mut xs);
        assert_eq!(*i.unwrap(), i1)
    }

    #[test]
    fn hit_when_some_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, s.get_id());
        let i2 = Intersection::new(1.0, s.get_id());
        let mut xs = vec![i1, i2];
        let i = find_hit(&mut xs);
        assert_eq!(*i.unwrap(), i2)
    }

    #[test]
    fn hit_when_all_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, s.get_id());
        let i2 = Intersection::new(-1.0, s.get_id());
        let mut xs = vec![i1, i2];
        let i = find_hit(&mut xs);
        assert!(i.is_none())
    }

    #[test]
    fn hit_always_lowest_nonnegative() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, s.get_id());
        let i2 = Intersection::new(7.0, s.get_id());
        let i3 = Intersection::new(-3.0, s.get_id());
        let i4 = Intersection::new(2.0, s.get_id());
        let mut xs = vec![i1, i2, i3, i4];
        let i = find_hit(&mut xs);
        assert_eq!(*i.unwrap(), i4)
    }
}

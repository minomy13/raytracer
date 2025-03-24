use uuid::Uuid;

pub struct Intersection {
    t: f64,
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

pub fn filter_hits(intersections: Vec<Intersection>) -> Vec<Intersection> {
    todo!()
}

// TODO: check page 64; aggregating

#[cfg(test)]
mod tests {
    use crate::body::{sphere::Sphere, Body};

    use super::Intersection;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, s.get_id());
        assert_eq!(i.get_t(), 3.5);
        assert_eq!(i.get_object_id(), s.get_id())
    }
}

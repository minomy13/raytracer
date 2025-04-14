pub(crate) const EPSILON: f64 = 0.00001;

pub fn float_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[allow(unused_macros)]
macro_rules! assert_f64_eq {
    ($a:expr, $b:expr) => {
        assert!(crate::utils::float_eq($a, $b))
    };
}
#[allow(unused_imports)]
pub(crate) use assert_f64_eq;

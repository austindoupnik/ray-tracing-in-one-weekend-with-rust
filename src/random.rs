use rand::{Rng, thread_rng};

pub fn random() -> f64 {
    thread_rng().gen()
}

#[allow(dead_code)]
pub fn random_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min, max)
}
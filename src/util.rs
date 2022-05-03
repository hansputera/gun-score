use rand::{thread_rng, Rng};

pub fn get_random_number_float(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    let result = rng.gen_range(min..max);

    drop(rng);
    result
}

pub fn get_random_number(min: u64, max: u64) -> u64 {
    let mut rng = thread_rng();
    let result = rng.gen_range(min..max);

    drop(result);
    result
}

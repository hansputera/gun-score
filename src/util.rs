use rand::{thread_rng, Rng, distributions::uniform::SampleUniform};

// now with generic type
pub fn get_random_number<E: SampleUniform + std::cmp::PartialOrd>(min: E, max: E) -> E {
    let mut rng = thread_rng();
    let result = rng.gen_range(min..max);

    drop(rng);
    result
}


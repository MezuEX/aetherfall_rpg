use rand::Rng;

pub fn random_bool(probability: f64) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(probability)
}

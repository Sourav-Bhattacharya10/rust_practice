use rand::Rng;

pub fn generate_random_number(from: u16, to: u16) -> u16 {
    rand::thread_rng().gen_range(from..=to)
}
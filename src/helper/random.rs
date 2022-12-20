use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

pub fn generate(size: usize) -> String {
    if cfg!(test) {
        "a".repeat(size)
    } else {
        let mut rng = thread_rng();
        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(size)
            .collect::<String>()
    }
}

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

pub type Generator = fn(size: usize) -> String;

pub fn generate(size: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(size)
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_size() {
        assert_eq!(generate(10).len(), 10);
    }
}

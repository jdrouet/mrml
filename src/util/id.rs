use std::iter;

#[cfg(not(test))]
pub fn generate(size: usize) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(rand::distributions::Alphanumeric))
        .take(size)
        .collect::<String>()
}

#[cfg(test)]
pub fn generate(size: usize) -> String {
    iter::repeat(())
        .map(|()| 'x')
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

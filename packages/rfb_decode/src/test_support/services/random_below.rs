use super::next_random;

pub fn random_below(state: &mut u64, limit: usize) -> usize {
    let limit = u64::try_from(limit).expect("a test limit fits a u64");
    usize::try_from(next_random(state) % limit).expect("a value below a usize limit fits a usize")
}

use std::collections::HashSet;

pub fn solution_a(input: &[u8], window: usize) -> usize {
    input
        .windows(window)
        .position(|window| HashSet::<_>::from_iter(window.iter()).len() == window.len())
        .unwrap()
        + window
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &[u8] = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes();
    const INPUT: &[u8] = include_bytes!("input.txt");

    #[test]
    fn it_works() {
        assert_eq!(solution_a(TEST_INPUT, 4), 7);
        println!("{}", solution_a(INPUT, 4));
        assert_eq!(solution_a(TEST_INPUT, 14), 19);
        println!("{}", solution_a(INPUT, 14));
    }
}

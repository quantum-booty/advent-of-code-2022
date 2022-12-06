use std::collections::HashSet;

pub fn solution_a(input: &[u8], window: usize) -> usize {
    input
        .windows(window)
        .take_while(|window| {
            let set = HashSet::<_>::from_iter(window.iter());
            // println!("{:?} {:?}", set, window);
            set.len() != window.len()
        })
        .count()
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
        let result = solution_a(INPUT, 4);
        println!("{}", result);
        assert_eq!(solution_a(TEST_INPUT, 14), 19);
        let result = solution_a(INPUT, 14);
        println!("{}", result);
    }
}

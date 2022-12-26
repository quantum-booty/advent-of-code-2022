use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

pub fn solution(input: &str, decryption_key: i64, n_mixes: usize) -> i64 {
    let (_, numbers) = parse(input).unwrap();
    let numbers: Vec<_> = numbers.iter().map(|n| n * decryption_key).collect();
    // mix_index index encodes the ordering/position of number
    // mix_index element encodes which the index in numbers it correspond to
    let mut mix_index: Vec<_> = (0..numbers.len()).collect();
    for _ in 0..n_mixes {
        mix(&numbers, &mut mix_index);
    }
    let mixed_numbers: Vec<_> = mix_index.iter().map(|&idx| numbers[idx]).collect();

    let zero_idx = mixed_numbers.iter().position(|n| *n == 0).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|steps| (zero_idx + steps) % mixed_numbers.len())
        .map(|idx| mixed_numbers[idx])
        .sum()
}

fn mix(numbers: &[i64], mix_index: &mut Vec<usize>) {
    for (idx, &number) in numbers.iter().enumerate() {
        let number_position = mix_index
            .iter()
            .position(|mix_idx| *mix_idx == idx)
            .unwrap();
        mix_index.remove(number_position);
        let new_number_position =
            (number_position as i64 + number).rem_euclid(mix_index.len() as i64) as usize;
        mix_index.insert(new_number_position, idx);
    }
}

fn parse(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(newline, complete::i64)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1
2
-3
3
-2
0
4";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn it_works() {
        let decryption_key = 811589153;
        assert_eq!(solution(TEST_INPUT, 1, 1), 3);
        println!("{}", solution(INPUT, 1, 1));
        assert_eq!(solution(TEST_INPUT, decryption_key, 10), 1623178306);
        println!("{}", solution(INPUT, decryption_key, 10));
    }
}

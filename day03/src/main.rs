#![feature(iter_array_chunks)]
use std::collections::HashSet;

// a = 97
// z = 122
// A = 65
// Z = 90
fn main() {
    let test_input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    let input = include_str!("inputs.txt");

    println!("{}", solution_a(test_input));
    println!("{}", solution_a(input));
    println!("{}", solution_b(test_input));
    println!("{}", solution_b(input));
}

fn solution_b(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .map(|lines| {
            let mut iter = lines
                .iter()
                .map(|line| HashSet::<_>::from_iter(line.as_bytes()));
            let intersection = iter
                .next()
                .map(|set| {
                    iter.fold(set, |set1, set2| {
                        set1.intersection(&set2).copied().collect()
                    })
                })
                .unwrap();
            char_to_priority(**intersection.iter().next().unwrap())
        })
        .sum()
}

fn solution_a(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let letters = line.as_bytes();
            let (left, right) = letters.split_at(letters.len() / 2);
            let left = HashSet::<_>::from_iter(left);
            let right = HashSet::<_>::from_iter(right);
            let shared = **left.intersection(&right).into_iter().next().unwrap();
            char_to_priority(shared)
        })
        .sum()
}

fn char_to_priority(c: u8) -> u32 {
    if (97..=122).contains(&c) {
        (c - 96).into()
    } else {
        (c - 38).into()
    }
}

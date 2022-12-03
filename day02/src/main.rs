fn main() {
    println!("Hello, world!");

    let test_input = "A Y
B X
C Z";
    println!("{}", solution_a(test_input));
    println!("{}", solution_a(include_str!("inputs.txt")));
    println!("{}", solution_b(test_input));
    println!("{}", solution_b(include_str!("inputs.txt")));
}

fn solution_a(inputs: &str) -> u32 {
    parse(inputs)
        .into_iter()
        .map(|(opponent, mine)| calculate_score_a(opponent, mine))
        .sum()
}

fn solution_b(inputs: &str) -> u32 {
    parse(inputs)
        .into_iter()
        .map(|(opponent, mine)| calculate_score_b(opponent, mine))
        .sum()
}

fn parse(inputs: &str) -> Vec<(&str, &str)> {
    inputs
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .collect()
}

fn calculate_score_b(opponent: &str, mine: &str) -> u32 {
    match (opponent, mine) {
        // lost
        ("A", "X") => 3,
        ("B", "X") => 1,
        ("C", "X") => 2,
        // draw
        ("A", "Y") => 4,
        ("B", "Y") => 5,
        ("C", "Y") => 6,
        // won
        ("A", "Z") => 8,
        ("B", "Z") => 9,
        ("C", "Z") => 7,
        _ => unreachable!(),
    }
}

fn calculate_score_a(opponent: &str, mine: &str) -> u32 {
    match (opponent, mine) {
        // lost
        ("B", "X") => 1,
        ("C", "Y") => 2,
        ("A", "Z") => 3,
        // draw
        ("A", "X") => 4,
        ("B", "Y") => 5,
        ("C", "Z") => 6,
        // won
        ("C", "X") => 7,
        ("A", "Y") => 8,
        ("B", "Z") => 9,
        _ => unreachable!(),
    }
}

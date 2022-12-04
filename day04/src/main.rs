fn main() {
    let test_input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    let input = include_str!("input.txt");
    println!("{}", solution_a(test_input));
    println!("{}", solution_a(input));
    println!("{}", solution_b(test_input));
    println!("{}", solution_b(input));
}

fn solution_a(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(a, b)| (a.1 >= b.1 && a.0 <= b.0) || (b.1 >= a.1 && b.0 <= a.0))
        .count()
}

fn solution_b(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(a, b)| b.1 >= a.0 && a.1 >= b.0)
        .count()
}

fn parse(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    input
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(a, b)| (a.split_once('-').unwrap(), b.split_once('-').unwrap()))
                .unwrap()
        })
        .map(|(a, b)| {
            (
                (a.0.parse::<usize>().unwrap(), a.1.parse::<usize>().unwrap()),
                (b.0.parse::<usize>().unwrap(), b.1.parse::<usize>().unwrap()),
            )
        })
        .collect()
}

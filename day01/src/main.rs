fn main() {
    let test_input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    println!("{}", solution_a(test_input));
    println!("{}", solution_a(include_str!("input.txt")));
    println!("{}", solution_b(include_str!("input.txt")));
}

fn solution_a(input: &str) -> usize {
    parse_inventories(input).into_iter().max().unwrap()
}

fn solution_b(input: &str) -> usize {
    let mut inventories = parse_inventories(input);
    inventories.sort_unstable();
    inventories.iter().rev().take(3).sum()
}

fn parse_inventories(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|inventories| {
            inventories
                .lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum()
        })
        .collect()
}

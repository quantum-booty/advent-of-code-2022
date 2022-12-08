use itertools::Itertools;

fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn get_up_down_left_right(grid: &[Vec<u32>], x: usize, y: usize) -> Vec<Vec<u32>> {
    let column = grid.iter().map(|row| row[x]).collect::<Vec<u32>>();
    let (up, down) = column.split_at(y);
    let (left, right) = grid[y].split_at(x);
    vec![
        up.iter().rev().cloned().collect(),
        down[1..].to_vec(),
        left.iter().rev().cloned().collect(),
        right[1..].to_vec(),
    ]
}

pub fn solution_a(input: &str) -> usize {
    let grid = parse_grid(input);
    let len = grid.len();

    (1..len - 1)
        .cartesian_product(1..len - 1)
        .map(|(y, x)| {
            let height = grid[y][x];
            get_up_down_left_right(&grid, x, y)
                .iter()
                .map(|direction| direction.iter().all(|h| *h < height))
                .any(|direction_visible| direction_visible)
        })
        .filter(|direction_visible| *direction_visible)
        .count()
        + (len - 1) * 4
}

pub fn solution_b(input: &str) -> u32 {
    let grid = parse_grid(input);
    let len = grid.len();
    (1..len - 1)
        .cartesian_product(1..len - 1)
        .map(|(y, x)| {
            let height = grid[y][x];
            get_up_down_left_right(&grid, x, y)
                .iter()
                .map(|direction| {
                    direction
                        .iter()
                        .position(|h| *h >= height)
                        .map(|p| p + 1)
                        .unwrap_or_else(|| direction.len())
                })
                .product::<usize>()
        })
        .max()
        .unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn it_works() {
        assert_eq!(solution_a(TEST_INPUT), 21);
        println!("{}", solution_a(INPUT));
        assert_eq!(solution_b(TEST_INPUT), 8);
        println!("{}", solution_b(INPUT));
    }
}

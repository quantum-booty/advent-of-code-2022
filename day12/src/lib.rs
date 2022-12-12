use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

pub fn solution_a(input: &str) -> usize {
    let grid = parse_grid(input);
    let (start, end) = parse_start_end(input);
    breadth_first_search(&grid, start, end)
}

pub fn solution_b(input: &str) -> usize {
    let grid = parse_grid(input);
    let (_, end) = parse_start_end(input);
    breadth_first_search_from_end(&grid, end)
}

fn breadth_first_search(grid: &Vec<Vec<u8>>, start: Coord, end: Coord) -> usize {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::new();

    while !queue.is_empty() {
        let (current_node, steps) = queue.pop_back().unwrap();
        if current_node == end {
            return steps;
        }
        if visited.contains(&current_node) {
            continue;
        }
        visited.insert(current_node);
        for neighbour in get_neighbours(grid, &current_node) {
            if grid[neighbour.y][neighbour.x] <= grid[current_node.y][current_node.x] + 1 {
                queue.push_front((neighbour, steps + 1));
            }
        }
    }
    0
}

fn breadth_first_search_from_end(grid: &Vec<Vec<u8>>, end: Coord) -> usize {
    let mut queue = VecDeque::from([(end, 0)]);
    let mut visited = HashSet::new();

    while !queue.is_empty() {
        let (current_node, steps) = queue.pop_back().unwrap();
        if grid[current_node.y][current_node.x] == b'a' {
            return steps;
        }
        if visited.contains(&current_node) {
            continue;
        }
        visited.insert(current_node);
        for neighbour in get_neighbours(grid, &current_node) {
            if grid[current_node.y][current_node.x] <= grid[neighbour.y][neighbour.x] + 1 {
                queue.push_front((neighbour, steps + 1));
            }
        }
    }
    0
}

fn get_neighbours(grid: &Vec<Vec<u8>>, coord: &Coord) -> Vec<Coord> {
    let height = grid.len();
    let width = grid[0].len();

    let mut neighbours = vec![];
    if 0 < coord.x {
        neighbours.push(Coord {
            x: coord.x - 1,
            y: coord.y,
        });
    }
    if coord.x < width - 1 {
        neighbours.push(Coord {
            x: coord.x + 1,
            y: coord.y,
        });
    }
    if 0 < coord.y {
        neighbours.push(Coord {
            x: coord.x,
            y: coord.y - 1,
        });
    }
    if coord.y < height - 1 {
        neighbours.push(Coord {
            x: coord.x,
            y: coord.y + 1,
        });
    }
    neighbours
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .replace('S', "a")
        .replace('E', "z")
        .lines()
        .map(|line| line.bytes().collect::<Vec<u8>>())
        .collect()
}

fn parse_start_end(input: &str) -> (Coord, Coord) {
    let s = input
        .lines()
        .enumerate()
        .map(|(y, line)| (y, line.find('S')))
        .find(|(_, s)| s.is_some())
        .map(|(y, s)| Coord { y, x: s.unwrap() })
        .unwrap();
    let e = input
        .lines()
        .enumerate()
        .map(|(y, line)| (y, line.find('E')))
        .find(|(_, e)| e.is_some())
        .map(|(y, e)| Coord { y, x: e.unwrap() })
        .unwrap();
    (s, e)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn it_works() {
        assert_eq!(solution_a(TEST_INPUT), 31);
        println!("{}", solution_a(INPUT));
        assert_eq!(solution_b(TEST_INPUT), 29);
        println!("{}", solution_b(INPUT));
    }
}

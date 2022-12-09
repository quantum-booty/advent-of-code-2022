use std::collections::HashSet;

struct Cmd {
    direction: Direction,
    amount: i32,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

pub fn solution_a(input: &str, n_knots: usize) -> usize {
    let cmds = parse(input);
    let mut p = vec![Coord { x: 0, y: 0 }; n_knots];
    let mut visited = HashSet::<Coord>::new();
    visited.insert(Coord { x: 0, y: 0 });

    for cmd in cmds {
        for _ in 0..cmd.amount {
            match cmd.direction {
                Direction::Up => p[0].y += 1,
                Direction::Down => p[0].y -= 1,
                Direction::Left => p[0].x -= 1,
                Direction::Right => p[0].x += 1,
            }

            for i in 0..p.len() - 1 {
                let dx = p[i].x - p[i + 1].x;
                let dy = p[i].y - p[i + 1].y;

                let not_touching = dx.abs() > 1 || dy.abs() > 1;
                if not_touching {
                    p[i + 1].x += dx.signum();
                    p[i + 1].y += dy.signum();
                }
            }

            visited.insert(p[p.len() - 1]);
        }
    }
    visited.len()
}

fn parse(input: &str) -> Vec<Cmd> {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, amt)| {
            let dir = match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            };
            Cmd {
                direction: dir,
                amount: amt.parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const TEST_INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn it_works() {
        assert_eq!(solution_a(TEST_INPUT1, 2), 13);
        println!("{}", solution_a(INPUT, 2));
        assert_eq!(solution_a(TEST_INPUT2, 10), 36);
        println!("{}", solution_a(INPUT, 10));
    }
}

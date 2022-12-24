use hashbrown::{HashMap, HashSet};

use Dir::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Elf {
    x: i32,
    y: i32,
}

impl Elf {
    fn neighbour(&self, dir: &Dir) -> Self {
        match dir {
            N => Self {
                y: self.y - 1,
                x: self.x,
            },
            S => Self {
                y: self.y + 1,
                x: self.x,
            },
            W => Self {
                y: self.y,
                x: self.x - 1,
            },
            E => Self {
                y: self.y,
                x: self.x + 1,
            },
            NE => Self {
                y: self.y - 1,
                x: self.x + 1,
            },
            SE => Self {
                y: self.y + 1,
                x: self.x + 1,
            },
            NW => Self {
                y: self.y - 1,
                x: self.x - 1,
            },
            SW => Self {
                y: self.y + 1,
                x: self.x - 1,
            },
        }
    }

    fn has_neighbour(&self, elves: &HashSet<Elf>) -> bool {
        [N, S, W, E, NE, SE, NW, SW]
            .iter()
            .any(|dir| elves.contains(&self.neighbour(dir)))
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    S,
    W,
    E,
    NE,
    SE,
    NW,
    SW,
}

pub fn solution_a(input: &str) -> usize {
    let mut elves = parse_elves(input);
    let y_dim = input.lines().count();
    let x_dim = input.lines().next().unwrap().len();
    // visualise(&elves, x_dim, y_dim);
    let mut proposal = vec![
        ([N, NE, NW], N),
        ([S, SE, SW], S),
        ([W, NW, SW], W),
        ([E, NE, SE], E),
    ];

    for _ in 0..10 {
        elves = simulate_step(&elves, &mut proposal);
    }

    score(elves)
}

pub fn solution_b(input: &str) -> usize {
    let mut elves = parse_elves(input);
    // visualise(&elves, x_dim, y_dim);
    let mut proposal = vec![
        ([N, NE, NW], N),
        ([S, SE, SW], S),
        ([W, NW, SW], W),
        ([E, NE, SE], E),
    ];

    for step in 1.. {
        let new_elves = simulate_step(&elves, &mut proposal);

        if elves.difference(&new_elves).count() == 0 {
            return step;
        }
        elves = new_elves;
    }
    0
}

fn simulate_step(elves: &HashSet<Elf>, proposal: &mut Vec<([Dir; 3], Dir)>) -> HashSet<Elf> {
    // HashMap<coord to move to, elfs that would like to move to that coord>
    let mut proposed_move_counts = HashMap::<Elf, HashSet<Elf>>::new();
    let mut new_elves = HashSet::new();
    for elf in elves {
        if !elf.has_neighbour(elves) {
            new_elves.insert(*elf);
            continue;
        }
        if let Some(proposed_dir) = propose_move(&*proposal, elves, elf) {
            proposed_move_counts
                .entry(elf.neighbour(&proposed_dir))
                .and_modify(|elves| {
                    elves.insert(*elf);
                })
                .or_insert(HashSet::from([*elf]));
        } else {
            new_elves.insert(*elf);
        }
    }
    for (coord_to_move_to, elves_looking_to_move) in proposed_move_counts {
        if elves_looking_to_move.len() == 1 {
            new_elves.insert(coord_to_move_to);
        } else {
            for elf in elves_looking_to_move {
                new_elves.insert(elf);
            }
        }
    }
    proposal.rotate_left(1);
    // visualise(&new_elves, x_dim, y_dim);
    new_elves
}

fn score(elves: HashSet<Elf>) -> usize {
    let y_max = elves.iter().map(|e| e.y).max().unwrap();
    let y_min = elves.iter().map(|e| e.y).min().unwrap();
    let y_dim = (y_max - y_min + 1) as usize;
    let x_max = elves.iter().map(|e| e.x).max().unwrap();
    let x_min = elves.iter().map(|e| e.x).min().unwrap();
    let x_dim = (x_max - x_min + 1) as usize;
    x_dim * y_dim - elves.len()
}

fn propose_move(proposal: &Vec<([Dir; 3], Dir)>, elves: &HashSet<Elf>, elf: &Elf) -> Option<Dir> {
    for (adj_dirs, proposed_dir) in proposal {
        if adj_dirs
            .iter()
            .all(|dir| !elves.contains(&elf.neighbour(dir)))
        {
            return Some(*proposed_dir);
        }
    }
    None
}

fn parse_elves(input: &str) -> HashSet<Elf> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '.' {
                    None
                } else {
                    Some(Elf {
                        x: x as i32,
                        y: y as i32,
                    })
                }
            })
        })
        .collect()
}

fn visualise(elves: &HashSet<Elf>, x_dim: usize, y_dim: usize) {
    let mut grid = vec![vec!['.'; x_dim]; y_dim];
    for elf in elves {
        // if elf.y as usize >= y_dim || elf.x as usize >= x_dim {
        //     continue;
        // }
        grid[elf.y as usize][elf.x as usize] = '#';
    }
    let grid_str = grid
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{grid_str}");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        solution_a(TEST_INPUT);
        assert_eq!(solution_a(TEST_INPUT_2), 110);
        println!("{}", solution_a(INPUT));
        assert_eq!(solution_b(TEST_INPUT_2), 20);
        println!("{}", solution_b(INPUT));
    }

    const TEST_INPUT: &str = ".....
..##.
..#..
.....
..##.
.....";

    const TEST_INPUT_2: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    const INPUT: &str = include_str!("input.txt");
}

use std::collections::HashSet;

use itertools::Itertools;
use Movement::*;

fn visualise(rocks: &HashSet<(u64, u64)>, tetris: &[(u64, u64)]) {
    // println!("{:?} {:?}", rocks, tetris);
    let y_max = rocks
        .iter()
        .chain(tetris.iter())
        .map(|(_, y)| y)
        .max()
        .unwrap();

    let mut grid = vec![vec!['.'; 9]; *y_max as usize + 1];

    for y in 0..y_max + 1 {
        grid[y as usize][0] = '|';
        grid[y as usize][8] = '|';
    }

    for x in 0..9 {
        grid[0][x] = '-';
    }

    for rock in rocks.iter() {
        // println!("{rock:?}");
        grid[rock.1 as usize][rock.0 as usize] = '#';
    }
    for rock in tetris.iter() {
        // println!("{rock:?}");
        grid[rock.1 as usize][rock.0 as usize] = '@';
    }

    let plot = grid
        .iter()
        .rev()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{plot}\n");
}

pub fn solution(input: &str, simulation_steps: usize, total_steps: usize) -> u64 {
    let movements = parse(input);
    let mut rocks = HashSet::<(u64, u64)>::new();
    let mut spawn_funcs = [
        spawn_horizontal,
        spawn_cross,
        spawn_l,
        spawn_vertical,
        spawn_square,
    ]
    .iter()
    .cycle();
    let mut movements = movements.iter().cycle();
    let mut y_max = 0;
    let mut tetris_stack = Vec::new();
    for _ in 0..simulation_steps {
        let spawn_func = spawn_funcs.next().unwrap();
        let mut tetris = spawn_func(y_max);
        // visualise(&rocks, &tetris);
        loop {
            let movement = movements.next().unwrap();
            // move left or right
            // println!("Push to {movement:?}");
            match movement {
                Left => {
                    let moved: Vec<_> = tetris.iter().map(|(x, y)| (*x - 1, *y)).collect();
                    if moved
                        .iter()
                        .all(|rock| !rocks.contains(rock) && rock.0 != 0)
                    {
                        tetris = moved;
                    }
                }
                Right => {
                    let moved: Vec<_> = tetris.iter().map(|(x, y)| (*x + 1, *y)).collect();
                    if moved
                        .iter()
                        .all(|rock| !rocks.contains(rock) && rock.0 != 8)
                    {
                        tetris = moved;
                    }
                }
            }
            // visualise(&rocks, &tetris);
            // fall down
            let moved: Vec<_> = tetris.iter().map(|(x, y)| (*x, *y - 1)).collect();
            if moved
                .iter()
                .all(|rock| !rocks.contains(rock) && rock.1 != 0)
            {
                // println!("Rock falls 1 unit");
                tetris = moved;
            } else {
                // println!("Rock cant fall");
                break;
            }
            // visualise(&rocks, &tetris);
        }
        for rock in &tetris {
            rocks.insert(*rock);
        }
        tetris_stack.push(tetris);
        y_max = *rocks.iter().map(|(_, y)| y).max().unwrap();
    }

    let repeat_segment_length = 30;
    let repeat_segment = &tetris_stack[tetris_stack.len() - repeat_segment_length..];

    let repeat_segment_positions: Vec<_> = tetris_stack
        .windows(repeat_segment_length)
        .positions(|window| {
            window
                .iter()
                .zip(repeat_segment.iter())
                .all(|(l, r)| l.iter().zip(r.iter()).all(|(l, r)| l.0 == r.0))
        })
        .take(2)
        .collect();

    let cummax: Vec<_> = tetris_stack
        .iter()
        .map(|tetris| tetris.iter().map(|(_, y)| y).max().unwrap())
        .scan(0u64, |max, e| {
            *max = (*max).max(*e);
            Some(*max)
        })
        .collect();

    let first_repeat = repeat_segment_positions[0];
    let second_repeat = repeat_segment_positions[1];

    let cummax = &cummax[first_repeat - 1..second_repeat];

    let cumdiff: Vec<_> = cummax.iter().map(|m| m - cummax[0]).collect();
    let cumdiff = &cumdiff[1..];

    let initial_height = cummax[0];
    let number_repeats = (total_steps - first_repeat - 1) / cumdiff.len();
    let final_height = initial_height + number_repeats as u64 * cumdiff[cumdiff.len() - 1];
    let left_over_steps = (total_steps - first_repeat - 1) % cumdiff.len();
    final_height + cumdiff[left_over_steps]
}

// 0 1 2 3 4 5 6 7 8
// 0 left wall 8 right wall
fn spawn_horizontal(y_max: u64) -> Vec<(u64, u64)> {
    vec![
        (3, y_max + 4),
        (4, y_max + 4),
        (5, y_max + 4),
        (6, y_max + 4),
    ]
}

fn spawn_cross(y_max: u64) -> Vec<(u64, u64)> {
    vec![
        (3, y_max + 5),
        (4, y_max + 5),
        (5, y_max + 5),
        (4, y_max + 6),
        (4, y_max + 4),
    ]
}

fn spawn_l(y_max: u64) -> Vec<(u64, u64)> {
    vec![
        (3, y_max + 4),
        (4, y_max + 4),
        (5, y_max + 4),
        (5, y_max + 5),
        (5, y_max + 6),
    ]
}

fn spawn_vertical(y_max: u64) -> Vec<(u64, u64)> {
    vec![
        (3, y_max + 4),
        (3, y_max + 5),
        (3, y_max + 6),
        (3, y_max + 7),
    ]
}

fn spawn_square(y_max: u64) -> Vec<(u64, u64)> {
    vec![
        (3, y_max + 4),
        (4, y_max + 4),
        (3, y_max + 5),
        (4, y_max + 5),
    ]
}

#[derive(Debug)]
enum Movement {
    Left,
    Right,
}

fn parse(input: &str) -> Vec<Movement> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Right,
            '<' => Left,
            _ => unreachable!(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn it_works() {
        assert_eq!(solution(TEST_INPUT, 80, 2022), 3068);
        println!("{}", solution(INPUT, 1971, 2022));
        assert_eq!(solution(TEST_INPUT, 80, 1000000000000), 1514285714288);
        println!("{}", solution(INPUT, 1971, 1000000000000));
    }
}

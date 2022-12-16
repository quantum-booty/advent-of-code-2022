#![feature(iter_array_chunks)]
use std::collections::HashSet;

use itertools::Itertools;

use regex::Regex;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn manhat_dist(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

pub fn solution_a(input: &str, y: i32) -> usize {
    let pairs = parse(input);
    pairs
        .iter()
        .filter_map(|(sensor, beacon)| {
            let manhat = sensor.manhat_dist(beacon);
            let y_dist = sensor.y.abs_diff(y);
            if manhat >= y_dist {
                let radius = manhat - y_dist;
                let mut positions =
                    HashSet::<i32>::from_iter(sensor.x - radius as i32..=sensor.x + radius as i32);
                if beacon.y == y {
                    positions.remove(&beacon.x);
                }
                println!("{:?}", positions.len());
                Some(positions)
            } else {
                None
            }
        })
        .reduce(|acc, e| acc.union(&e).copied().collect())
        .unwrap()
        .len()
}

pub fn solution_b(input: &str, max: i32) -> i128 {
    let pairs = parse(input);
    for y in 0..max {
        let mut ranges: Vec<_> = pairs
            .iter()
            .filter_map(|(sensor, beacon)| {
                let manhat = sensor.manhat_dist(beacon);
                let y_dist = sensor.y.abs_diff(y);
                if manhat >= y_dist {
                    let radius = manhat - y_dist;
                    Some(sensor.x - radius as i32..=sensor.x + radius as i32)
                } else {
                    None
                }
            })
            .collect();
        ranges.sort_by_key(|r| *r.start());

        // println!("{:?}", ranges);
        let mut end = 0;
        for r in ranges {
            if r.contains(&end) {
                end = *r.end();
                continue;
            }
            if end + 2 == *r.start() {
                println!(
                    "{:?} {} {}",
                    end + 1,
                    y,
                    (end as i128 + 1) * 4000000 + y as i128
                );
                // return (end as i128 + 1) * 4000000 + y as i128;
            }
        }
    }
    1
}

fn parse(input: &str) -> Vec<(Coord, Coord)> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(input)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .array_chunks::<4>()
        .map(|chunk| {
            (
                Coord {
                    x: chunk[0],
                    y: chunk[1],
                },
                Coord {
                    x: chunk[2],
                    y: chunk[3],
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");
    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn it_works() {
        assert_eq!(solution_a(TEST_INPUT, 10), 26);
        println!("{}", solution_a(INPUT, 2000000));
        println!("{}", solution_b(TEST_INPUT, 20));
        println!("{}", solution_b(INPUT, 4000000));
    }
}

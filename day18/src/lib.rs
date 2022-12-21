use std::collections::{HashSet, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub fn solution_a(input: &str) -> usize {
    let (_, coords) = parse(input).unwrap();
    let coords: HashSet<(i32, i32, i32)> = HashSet::from_iter(coords.into_iter());
    coords
        .iter()
        .flat_map(|c| get_neighbours(*c))
        // for every coord, count neighbours that aren't in coords, i.e. is air
        .filter(|c| !coords.contains(c))
        .count()
}

pub fn solution_b(input: &str) -> usize {
    let (_, coords) = parse(input).unwrap();
    let coords: HashSet<(i32, i32, i32)> = HashSet::from_iter(coords.into_iter());
    let x_min = *coords.iter().map(|(x, _, _)| x).min().unwrap() - 1;
    let x_max = *coords.iter().map(|(x, _, _)| x).max().unwrap() + 1;
    let y_min = *coords.iter().map(|(_, y, _)| y).min().unwrap() - 1;
    let y_max = *coords.iter().map(|(_, y, _)| y).max().unwrap() + 1;
    let z_min = *coords.iter().map(|(_, _, z)| z).min().unwrap() - 1;
    let z_max = *coords.iter().map(|(_, _, z)| z).max().unwrap() + 1;

    let mut surface = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((x_min, y_min, z_min));
    while let Some(c) = queue.pop_front() {
        if c.0 < x_min
            || c.0 > x_max
            || c.1 < y_min
            || c.1 > y_max
            || c.2 < z_min
            || c.2 > z_max
        {
            continue;
        }

        if !visited.insert(c) {
            continue;
        }

        for neighbour in get_neighbours(c) {
            if coords.contains(&neighbour) {
                surface.insert(c);
                continue;
            }
            queue.push_back(neighbour);
        }
    }

    surface.iter()
        .flat_map(|c| get_neighbours(*c))
        // for every coord in surface, count neighbours that are in coord
        .filter(|c| coords.contains(c))
        .count()
}

fn get_neighbours(coord: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    vec![
        (coord.0 + 1, coord.1, coord.2),
        (coord.0 - 1, coord.1, coord.2),
        (coord.0, coord.1 + 1, coord.2),
        (coord.0, coord.1 - 1, coord.2),
        (coord.0, coord.1, coord.2 + 1),
        (coord.0, coord.1, coord.2 - 1),
    ]
}

fn parse(input: &str) -> IResult<&str, Vec<(i32, i32, i32)>> {
    separated_list1(
        newline,
        map(
            tuple((
                complete::i32,
                tag(","),
                complete::i32,
                tag(","),
                complete::i32,
            )),
            |(x, _, y, _, z)| (x, y, z),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn it_works() {
        assert_eq!(solution_a(TEST_INPUT), 64);
        println!("{}", solution_a(INPUT));
        assert_eq!(solution_b(TEST_INPUT), 58);
        println!("{}", solution_b(INPUT));
    }
}

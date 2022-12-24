use std::collections::HashMap;

use nom::{branch::alt, character::complete, combinator::map, multi::many1, IResult};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    coord: Coord,
    is_wall: bool,
    up: Coord,
    down: Coord,
    left: Coord,
    right: Coord,
}

impl Node {
    fn get_neighbour(&self, direction: &Dir) -> Coord {
        match direction {
            Dir::U => self.up,
            Dir::D => self.down,
            Dir::L => self.left,
            Dir::R => self.right,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::U => Dir::D,
            Dir::D => Dir::U,
            Dir::L => Dir::R,
            Dir::R => Dir::L,
        }
    }
}

#[derive(Debug)]
enum Rotation {
    R,
    L,
}

impl Rotation {
    fn rotate(&self, dir: Dir) -> Dir {
        match (dir, self) {
            (Dir::U, Rotation::R) => Dir::R,
            (Dir::U, Rotation::L) => Dir::L,
            (Dir::D, Rotation::R) => Dir::L,
            (Dir::D, Rotation::L) => Dir::R,
            (Dir::L, Rotation::R) => Dir::U,
            (Dir::L, Rotation::L) => Dir::D,
            (Dir::R, Rotation::R) => Dir::D,
            (Dir::R, Rotation::L) => Dir::U,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Rotate(Rotation),
    Distance(u32),
}

pub fn solution(input: &str, is_test: bool, is_part_a: bool) -> u32 {
    let (graph_str, inst_str) = input.split_once("\n\n").unwrap();
    let edge_map = get_edge_map(is_test, is_part_a);
    let grid = parse_grid(graph_str);
    let start_coord = Coord {
        x: grid[0].iter().position(|c| c == &'.').unwrap() as u32 + 1,
        y: 1,
    };
    let graph = parse_graph(&edge_map, &grid);
    // for n in graph.keys().filter(|n| n.y == 8) {
    //     println!("{n:?}");
    // }
    // panic!("");
    let (_, instructions) = parse_instructions(inst_str).unwrap();
    let mut cur_node = &graph[&start_coord];
    let mut cur_dir = Dir::R;
    let mut path = vec![];
    for inst in instructions {
        match inst {
            Instruction::Rotate(rot) => cur_dir = rot.rotate(cur_dir),
            Instruction::Distance(dist) => {
                for _ in 0..dist {
                    path.push((cur_dir, *cur_node));
                    // visualise(&start_coord, &path, &grid);
                    let neighbour = &graph[&cur_node.get_neighbour(&cur_dir)];
                    if neighbour.is_wall {
                        break;
                    }
                    if let Some((_, new_dir)) = &edge_map.get(&(cur_node.coord, cur_dir)) {
                        cur_dir = *new_dir;
                    }
                    cur_node = neighbour;
                }
            }
        }
    }
    cur_node.coord.y * 1000
        + cur_node.coord.x * 4
        + match cur_dir {
            Dir::U => 3,
            Dir::D => 1,
            Dir::L => 2,
            Dir::R => 0,
        }
}

fn get_edge_pairs(
    from: (Coord, Coord),
    to: (Coord, Coord),
    is_same_direction: bool,
) -> Vec<(Coord, Coord)> {
    let is_from_horizontal = from.0.y == from.1.y;
    let is_to_horizontal = to.0.y == to.1.y;
    let from_coords = get_edge_coords(is_from_horizontal, from);
    let to_coords = get_edge_coords(is_to_horizontal, to);
    if is_same_direction {
        from_coords.into_iter().zip(to_coords.into_iter()).collect()
    } else {
        from_coords
            .into_iter()
            .zip(to_coords.into_iter().rev())
            .collect()
    }
}

fn get_edge_coords(is_edge_horizontal: bool, edge: (Coord, Coord)) -> Vec<Coord> {
    let mut edge_coords = vec![];
    if is_edge_horizontal {
        let y = edge.0.y;
        for x in edge.0.x..=edge.1.x {
            edge_coords.push(Coord { x, y });
        }
    } else {
        let x = edge.0.x;
        for y in edge.0.y..=edge.1.y {
            edge_coords.push(Coord { x, y });
        }
    }
    edge_coords
}

fn get_edge_map(is_test: bool, is_part_a: bool) -> HashMap<(Coord, Dir), (Coord, Dir)> {
    let edges = if is_test {
        HashMap::from([
            ("1a", (Coord { x: 5, y: 5 }, Coord { x: 8, y: 5 })),
            ("1b", (Coord { x: 9, y: 1 }, Coord { x: 9, y: 4 })),
            ("2a", (Coord { x: 5, y: 8 }, Coord { x: 8, y: 8 })),
            ("2b", (Coord { x: 9, y: 9 }, Coord { x: 9, y: 12 })),
            ("3a", (Coord { x: 1, y: 8 }, Coord { x: 4, y: 8 })),
            ("3b", (Coord { x: 9, y: 12 }, Coord { x: 12, y: 12 })),
            ("4a", (Coord { x: 1, y: 5 }, Coord { x: 4, y: 5 })),
            ("4b", (Coord { x: 9, y: 1 }, Coord { x: 12, y: 1 })),
            ("5a", (Coord { x: 1, y: 5 }, Coord { x: 1, y: 8 })),
            ("5b", (Coord { x: 13, y: 12 }, Coord { x: 16, y: 12 })),
            ("6a", (Coord { x: 12, y: 5 }, Coord { x: 12, y: 8 })),
            ("6b", (Coord { x: 13, y: 9 }, Coord { x: 16, y: 9 })),
            ("7a", (Coord { x: 13, y: 1 }, Coord { x: 13, y: 4 })),
            ("7b", (Coord { x: 16, y: 9 }, Coord { x: 16, y: 12 })),
        ])
    } else {
        HashMap::from([
            ("1a", (Coord { x: 50, y: 151 }, Coord { x: 50, y: 200 })),
            ("1b", (Coord { x: 51, y: 150 }, Coord { x: 100, y: 150 })),
            ("2a", (Coord { x: 100, y: 51 }, Coord { x: 100, y: 100 })),
            ("2b", (Coord { x: 101, y: 50 }, Coord { x: 150, y: 50 })),
            ("3a", (Coord { x: 1, y: 101 }, Coord { x: 1, y: 150 })),
            ("3b", (Coord { x: 51, y: 1 }, Coord { x: 51, y: 50 })),
            ("4a", (Coord { x: 1, y: 101 }, Coord { x: 50, y: 101 })),
            ("4b", (Coord { x: 51, y: 51 }, Coord { x: 51, y: 100 })),
            ("5a", (Coord { x: 1, y: 151 }, Coord { x: 1, y: 200 })),
            ("5b", (Coord { x: 51, y: 1 }, Coord { x: 100, y: 1 })),
            ("6a", (Coord { x: 101, y: 1 }, Coord { x: 150, y: 1 })),
            ("6b", (Coord { x: 1, y: 200 }, Coord { x: 50, y: 200 })),
            ("7a", (Coord { x: 100, y: 101 }, Coord { x: 100, y: 150 })),
            ("7b", (Coord { x: 150, y: 1 }, Coord { x: 150, y: 50 })),
        ])
    };
    // from_edge, to_edge, is_same_direction
    let fold = if is_test {
        if is_part_a {
            vec![
                (("1a", Dir::U), ("2a", Dir::U), true),
                (("4a", Dir::U), ("3a", Dir::U), true),
                (("4b", Dir::U), ("3b", Dir::U), true),
                (("6b", Dir::U), ("5b", Dir::U), true),
                (("1b", Dir::L), ("7a", Dir::L), true),
                (("5a", Dir::L), ("6a", Dir::L), true),
                (("2b", Dir::L), ("7b", Dir::L), true),
            ]
        } else {
            vec![
                (("1a", Dir::U), ("1b", Dir::R), true),
                (("2a", Dir::D), ("2b", Dir::R), false),
                (("3a", Dir::D), ("3b", Dir::U), false),
                (("4a", Dir::U), ("4b", Dir::D), false),
                (("5a", Dir::L), ("5b", Dir::U), false),
                (("6a", Dir::R), ("6b", Dir::D), false),
                (("7a", Dir::D), ("7b", Dir::L), false),
            ]
        }
    } else {
        if is_part_a {
            vec![
                (("4a", Dir::U), ("6b", Dir::U), true),
                (("5b", Dir::U), ("1b", Dir::U), true),
                (("6a", Dir::U), ("2b", Dir::U), true),
                (("3b", Dir::L), ("7b", Dir::L), true),
                (("4b", Dir::L), ("2a", Dir::L), true),
                (("3a", Dir::L), ("7a", Dir::L), true),
                (("5a", Dir::L), ("1a", Dir::L), true),
            ]
        } else {
            vec![
                (("1a", Dir::R), ("1b", Dir::U), true),
                (("2a", Dir::R), ("2b", Dir::U), true),
                (("3a", Dir::L), ("3b", Dir::R), false),
                (("4a", Dir::U), ("4b", Dir::R), true),
                (("5a", Dir::L), ("5b", Dir::D), true),
                (("6a", Dir::U), ("6b", Dir::U), true),
                (("7a", Dir::R), ("7b", Dir::L), false),
            ]
        }
    };

    let mut map = HashMap::new();
    for ((from_edge, from_dir), (to_edge, to_dir), is_same_direction) in fold {
        let edge_pairs = get_edge_pairs(edges[&from_edge], edges[&to_edge], is_same_direction);
        for (from, to) in edge_pairs {
            map.insert((from, from_dir), (to, to_dir));
            map.insert((to, to_dir.opposite()), (from, from_dir.opposite()));
        }
    }
    map
}

fn visualise(start_coord: &Coord, path: &Vec<(Dir, Node)>, grid: &Vec<Vec<char>>) {
    let mut grid = grid.clone();
    for (dir, n) in path {
        match dir {
            Dir::U => grid[n.coord.y as usize - 1][n.coord.x as usize - 1] = '^',
            Dir::D => grid[n.coord.y as usize - 1][n.coord.x as usize - 1] = 'v',
            Dir::L => grid[n.coord.y as usize - 1][n.coord.x as usize - 1] = '<',
            Dir::R => grid[n.coord.y as usize - 1][n.coord.x as usize - 1] = '>',
        }
    }
    grid[start_coord.y as usize - 1][start_coord.x as usize - 1] = '>';

    let grid_str: Vec<String> = grid
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect();
    println!("{}", grid_str.join("\n"));
    println!();
}

fn parse_graph(
    edge_map: &HashMap<(Coord, Dir), (Coord, Dir)>,
    grid: &Vec<Vec<char>>,
) -> HashMap<Coord, Node> {
    let mut graph = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let char = grid[y][x];
            if char == ' ' {
                continue;
            }
            let coord = Coord {
                x: x as u32 + 1,
                y: y as u32 + 1,
            };
            let node = Node {
                coord,
                is_wall: char == '#',
                up: get_neighbour(edge_map, &coord, &Dir::U),
                down: get_neighbour(edge_map, &coord, &Dir::D),
                left: get_neighbour(edge_map, &coord, &Dir::L),
                right: get_neighbour(edge_map, &coord, &Dir::R),
            };
            graph.insert(coord, node);
        }
    }
    graph
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let x_max = input.lines().map(|line| line.len()).max().unwrap();
    let y_max = input.lines().count();
    let mut grid = vec![vec![' '; x_max]; y_max];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c;
        }
    }
    grid
}

fn get_neighbour(
    map: &HashMap<(Coord, Dir), (Coord, Dir)>,
    coord: &Coord,
    direction: &Dir,
) -> Coord {
    if let Some((destination, _)) = map.get(&(*coord, *direction)) {
        return *destination;
    }
    match direction {
        Dir::U => Coord {
            x: coord.x,
            y: coord.y - 1,
        },
        Dir::D => Coord {
            x: coord.x,
            y: coord.y + 1,
        },
        Dir::L => Coord {
            x: coord.x - 1,
            y: coord.y,
        },
        Dir::R => Coord {
            x: coord.x + 1,
            y: coord.y,
        },
    }
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(alt((
        map(complete::u32, Instruction::Distance),
        map(complete::alpha1, |c| match c {
            "L" => Instruction::Rotate(Rotation::L),
            "R" => Instruction::Rotate(Rotation::R),
            _ => unreachable!(),
        }),
    )))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn it_works() {
        assert_eq!(solution(TEST_INPUT, true, true), 6032);
        println!("{}", solution(INPUT, false, true));
        assert_eq!(solution(TEST_INPUT, true, false), 5031);
        println!("{}", solution(INPUT, false, false));
    }
}

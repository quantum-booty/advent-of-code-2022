use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt,
};

use regex::Regex;

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
struct Valve {
    name: Name,
    flow_rate: u32,
    lead_to: Vec<Name>,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Name(u8, u8);

impl Name {
    fn new(name: &str) -> Self {
        let mut bytes = name.bytes();
        Name(bytes.next().unwrap(), bytes.next().unwrap())
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&[self.0, self.1]).unwrap())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct TimeState {
    cost: u32,
    valve: Name,
}

impl Ord for TimeState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for TimeState {
    // This is a min-heap, so we want the smallest element on top
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct FlowState {
    flow: u32,
    flow_rate: u32,
    time_remain: u32,
    valve: Name,
    visited: HashSet<Name>,
}

impl Ord for FlowState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.flow.cmp(&other.flow)
    }
}

impl PartialOrd for FlowState {
    // This is a max-heap, so we want the biggest element on top
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(self))
    }
}

fn shortest_time_path(
    graph: &HashMap<Name, Valve>,
    start: Name,
    end: Name,
    remaining_time: u32,
    visited: &HashSet<Name>,
) -> Option<(u32, Vec<Name>)> {
    let mut dist: HashMap<Name, u32> =
        HashMap::from_iter(graph.keys().map(|valve| (*valve, std::u32::MAX)));
    let mut heap = BinaryHeap::<TimeState>::new();
    let mut prev = HashMap::<Name, Name>::new();
    // we're at start, with a zero cost
    dist.entry(start).and_modify(|d| *d = 0);
    heap.push(TimeState {
        cost: 0,
        valve: start,
    });

    // examine the frontier with lower cost first (min-heap)
    while let Some(TimeState { cost, valve }) = heap.pop() {
        if valve == end {
            if cost > remaining_time {
                return None;
            } else {
                let mut path = vec![];
                let mut cur = end;
                path.push(cur);
                loop {
                    if cur == start {
                        break;
                    }
                    cur = prev[&cur];
                    path.push(cur);
                }
                path.reverse();
                return Some((cost, path));
            }
        }
        if cost > dist[&valve] {
            continue;
        }

        for edge in &graph[&valve].lead_to {
            let is_visited = visited.contains(edge);
            let next = TimeState {
                cost: cost + if is_visited { 1 } else { 2 },
                valve: *edge,
            };
            if next.cost < dist[&next.valve] {
                heap.push(next);
                prev.entry(next.valve)
                    .and_modify(|p| *p = *edge)
                    .or_insert(valve);
                dist.entry(next.valve).and_modify(|d| *d = next.cost);
            }
        }
    }
    // Goal not reachable
    None
}

// at a point
// find shortest path to all valve with positive flow rates
// for each valve, calculate how much time we would need to move to it, and how much flow we would
// gain with the remaining time we have
// the higher the flow gain, the lower the cost
// do the dijkstra thing
// update the graph to reflect which valve is open
//
pub fn solution_a(input: &str) -> u32 {
    let valves = parse(input);

    let graph: HashMap<Name, Valve> =
        HashMap::from_iter(valves.iter().map(|v| (v.name, v.clone())));

    let mut flows: HashMap<Name, u32> =
        HashMap::from_iter(graph.keys().map(|valve| (*valve, std::u32::MIN)));
    let mut heap = BinaryHeap::<FlowState>::new();

    let start = Name::new("AA");
    // we're at start, with a zero cost
    flows.entry(start).and_modify(|d| *d = 0);
    heap.push(FlowState {
        flow: 0,
        flow_rate: 0,
        time_remain: 30,
        valve: start,
        visited: HashSet::new(),
    });
    let mut results = vec![];
    // get frontier with highest flow (max heap)
    while let Some(FlowState {
        flow,
        time_remain,
        flow_rate,
        valve: from,
        visited,
    }) = heap.pop()
    {
        if time_remain == 0 {
            results.push(flow);
            continue;
        }

        // we want flow to be as high as possible, so skip if flow is lower
        if flow < flows[&from] {
            continue;
        }

        for to in &graph[&from].lead_to {
            let shit = shortest_time_path(&graph, from, *to, time_remain, &visited);
            if shit.is_none() {
                continue;
            }
            let (time_spent, _) = shit.unwrap();
            let mut visited = visited.clone();
            visited.insert(*to);
            let next = FlowState {
                flow: flow + time_spent * flow_rate,
                time_remain: time_remain - time_spent,
                flow_rate: graph
                    .iter()
                    .filter(|(k, _)| visited.contains(k))
                    .map(|(_, v)| v.flow_rate)
                    .sum(),
                valve: *to,
                visited,
            };
            if next.flow >= flows[&next.valve] {
                heap.push(next.clone());
                flows.entry(next.valve).and_modify(|d| *d = next.flow);
            }
        }
    }
    // dbg!(heap);
    println!("{:?}", results.iter().max());
    0
}

fn parse(input: &str) -> Vec<Valve> {
    let re = Regex::new(r"Valve ([A-Z]{2}) .*=(\d+).*valves? (.*)").unwrap();
    re.captures_iter(input)
        .map(|cap| Valve {
            name: Name::new(&cap[1]),
            flow_rate: cap[2].parse().unwrap(),
            lead_to: cap[3].split(", ").map(Name::new).collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn it_works() {
        println!("{}", solution_a(TEST_INPUT));
    }
}

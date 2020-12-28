use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Connection {
    start: String,
    end: String,
    distance: i32,
}

impl Connection {
    fn from_str(s: &str) -> Self {
        if let Ok((start, end, distance)) = scan_fmt!(s, "{} to {} = {d}", String, String, i32) {
            return Connection {
                start,
                end,
                distance,
            };
        }
        panic!("bad format")
    }
}

struct TSP {
    cities: HashSet<String>,
    edges: HashMap<(String, String), i32>,
}

impl TSP {
    fn from_connections(connections: &[Connection]) -> Self {
        let mut cities = HashSet::new();
        let mut edges = HashMap::new();

        for conn in connections {
            cities.insert(conn.start.clone());
            cities.insert(conn.end.clone());

            edges.insert((conn.start.clone(), conn.end.clone()), conn.distance);
            edges.insert((conn.end.clone(), conn.start.clone()), conn.distance);
        }

        TSP { cities, edges }
    }

    fn solve(&self) -> i32 {
        self.cities
            .iter()
            .permutations(self.cities.len())
            .map(|r| self.calculate_route_cost(r))
            .min()
            .unwrap()
    }

    fn solve_max(&self) -> i32 {
        self.cities
            .iter()
            .permutations(self.cities.len())
            .map(|r| self.calculate_route_cost(r))
            .max()
            .unwrap()
    }

    fn calculate_route_cost(&self, route: Vec<&String>) -> i32 {
        route
            .windows(2)
            .map(|e| self.edges.get(&(e[0].clone(), e[1].clone())).unwrap())
            .sum()
    }
}

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<Connection> {
    input.lines().map(Connection::from_str).collect()
}

#[aoc(day9, part1)]
pub fn part1(connections: &[Connection]) -> i32 {
    let tsp = TSP::from_connections(connections);
    tsp.solve()
}

#[aoc(day9, part2)]
pub fn part2(connections: &[Connection]) -> i32 {
    let tsp = TSP::from_connections(connections);
    tsp.solve_max()
}

use regex::Regex;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, Eq, Ord, Hash, PartialEq, PartialOrd)]
struct ValveID(u32);

impl ValveID {
    fn from(s: &str) -> Self {
        fn parse_char(c: char) -> u32 {
            match c {
                'A'..='Z' => (c as u32) - ('A' as u32),
                _ => unreachable!(),
            }
        }

        assert_eq!(s.len(), 2);
        let mut chars = s.chars();
        let a = parse_char(chars.next().unwrap());
        let b = parse_char(chars.next().unwrap());
        ValveID(a * 26 + b)
    }
}

impl fmt::Display for ValveID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn format_char(c: u32) -> char {
            (('A' as u32 as u8) + (c as u8)) as char
        }
        let a = format_char(self.0 / 26);
        let b = format_char(self.0 % 26);
        write!(f, "{a}{b}")
    }
}

struct Valve {
    id: ValveID,
    flow_rate: u32,
    neighbors: Vec<ValveID>,
}

struct Cave {
    valves: HashMap<ValveID, Valve>,
}

#[derive(Eq, Ord, Hash, PartialEq, PartialOrd)]
struct State {
    released_pressure: u32,
    remaining_time: u32,
    current_valve: ValveID,
    current_flow: u32,
    open_valves: Vec<ValveID>,
}

impl Cave {
    fn read(filename: &str) -> Self {
        let contents = std::fs::read_to_string(filename).unwrap();
        let mut tunnels = HashMap::new();
        let r = Regex::new(r"Valve (\S*) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
            .unwrap();
        for line in contents.lines() {
            let caps = r.captures(line).unwrap();
            let id = ValveID::from(&caps[1]);
            let flow_rate = caps[2].parse().unwrap();
            let neighbors = caps[3].split(", ").map(ValveID::from).collect();
            tunnels.insert(
                id,
                Valve {
                    id,
                    flow_rate,
                    neighbors,
                },
            );
        }
        Cave { valves: tunnels }
    }

    fn valve_distances(&self) -> HashMap<(ValveID, ValveID), u32> {
        // Floyd-Warshall
        let mut distances = HashMap::new();
        for valve in self.valves.values() {
            for &other_id in &valve.neighbors {
                distances.insert((valve.id, other_id), 1);
            }
        }
        for k in self.valves.values() {
            for i in self.valves.values() {
                for j in self.valves.values() {
                    // D[i, j] = min(D[i, j], D[i, k] + D[k, j])
                    if let Some(&a) = distances.get(&(i.id, k.id)) {
                        if let Some(&b) = distances.get(&(k.id, j.id)) {
                            distances
                                .entry((i.id, j.id))
                                .and_modify(|e| *e = (*e).min(a + b))
                                .or_insert(a + b);
                        }
                    }
                }
            }
        }
        distances
    }

    fn max_release(&self) -> u32 {
        // only consider the valves with a positive flow rate
        let valves: HashMap<&ValveID, &Valve> = self
            .valves
            .iter()
            .filter(|&(_, valve)| valve.flow_rate > 0)
            .collect();

        let distances = self.valve_distances();

        // try every ordering of the valves
        fn aux(
            valves: &HashMap<&ValveID, &Valve>,
            distances: &HashMap<(ValveID, ValveID), u32>,
            open_valves: usize,
            location: ValveID,
            released_pressure: u32,
            remaining_time: u32,
        ) -> u32 {
            if remaining_time == 0 {
                return released_pressure;
            }
            let mut best_released_pressure = released_pressure;
            for (i, &&valve_id) in valves.keys().enumerate() {
                if open_valves & (1 << i) != 0 {
                    continue;
                }
                let d = 1 + distances[&(location, valve_id)];
                if d > remaining_time {
                    continue;
                }
                // immediately account for all the pressure that will be released by this valve
                let remaining_time = remaining_time - d;
                let released_pressure =
                    released_pressure + valves[&valve_id].flow_rate * remaining_time;
                // recurse
                let released_pressure = aux(
                    valves,
                    distances,
                    open_valves | (1 << i),
                    valve_id,
                    released_pressure,
                    remaining_time,
                );
                best_released_pressure = best_released_pressure.max(released_pressure);
            }
            best_released_pressure
        }
        aux(&valves, &distances, 0, ValveID::from("AA"), 0, 30)
    }
}

fn single(filename: &str) -> u32 {
    let cave = Cave::read(filename);
    cave.max_release()
}

fn puzzle1() {
    assert_eq!(single("example"), 1651);
    assert_eq!(single("input"), 1940);
}

fn puzzle2() {
    // println!("{}", single("example"));
    // println!("{}", single("input"));
}

fn main() {
    puzzle1();
    puzzle2();
}

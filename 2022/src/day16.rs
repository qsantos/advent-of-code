use regex::Regex;
use std::collections::{HashMap, VecDeque};
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

#[derive(Eq, Hash, PartialEq)]
struct State {
    open_valves: usize,
    location: ValveID,
    remaining_time: u32,
}

impl Cave {
    fn read(input: &str) -> Self {
        let mut tunnels = HashMap::new();
        let r = Regex::new(r"Valve (\S*) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
            .unwrap();
        for line in input.lines() {
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

    fn end_states(&self, timeout: u32) -> HashMap<State, u32> {
        // only consider the valves with a positive flow rate
        let valves: HashMap<&ValveID, &Valve> = self
            .valves
            .iter()
            .filter(|&(_, valve)| valve.flow_rate > 0)
            .collect();

        let distances = self.valve_distances();

        let mut end_states = HashMap::new();

        let mut q = VecDeque::new();
        q.push_back((
            State {
                open_valves: 0,
                location: ValveID::from("AA"),
                remaining_time: timeout,
            },
            0,
        ));

        while let Some((state, released_pressure)) = q.pop_front() {
            let State {
                open_valves,
                location,
                remaining_time,
            } = state;
            if released_pressure < end_states.get(&state).copied().unwrap_or(0) {
                continue;
            }
            end_states.insert(state, released_pressure);
            if remaining_time == 0 {
                continue;
            }

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
                q.push_back((
                    State {
                        open_valves: open_valves | (1 << i),
                        location: valve_id,
                        remaining_time,
                    },
                    released_pressure,
                ));
            }
        }

        end_states
    }
}

pub fn part1(input: &str) -> u32 {
    let cave = Cave::read(input);
    let end_states = cave.end_states(30);
    *end_states.values().max().unwrap()
}

pub fn part2(input: &str) -> u32 {
    let cave = Cave::read(input);
    let end_states = cave.end_states(26);

    let mut best = 0;
    for (s1, v1) in &end_states {
        for (s2, v2) in &end_states {
            if s1.open_valves & s2.open_valves == 0 {
                best = best.max(v1 + v2)
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day16.txt");
    const INPUT: &str = include_str!("../inputs/day16.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 1651);
        assert_eq!(part1(INPUT), 1940);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 1707);
        assert_eq!(part2(INPUT), 2469);
    }
}

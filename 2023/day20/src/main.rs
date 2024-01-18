use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FlipFlopModule {
    is_on: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ConjunctionModule<'a> {
    last_pulse: HashMap<&'a str, Pulse>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ModuleExt<'a> {
    Broadcaster,
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule<'a>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Module<'a> {
    destinations: Vec<&'a str>,
    ext: ModuleExt<'a>,
}

impl<'a> Module<'a> {
    fn new_broadcaster(destinations: Vec<&'a str>) -> Self {
        let ext = ModuleExt::Broadcaster;
        Module { destinations, ext }
    }

    fn new_flipflop(destinations: Vec<&'a str>) -> Self {
        let ext = ModuleExt::FlipFlop(FlipFlopModule { is_on: false });
        Module { destinations, ext }
    }

    fn new_conjunction(destinations: Vec<&'a str>) -> Self {
        let ext = ModuleExt::Conjunction(ConjunctionModule {
            last_pulse: HashMap::new(),
        });
        Module { destinations, ext }
    }

    fn apply(&mut self, from: &str, pulse: Pulse) -> Option<Pulse> {
        match &mut self.ext {
            ModuleExt::Broadcaster => Some(pulse),
            ModuleExt::FlipFlop(ext) => {
                if pulse == Pulse::High {
                    None
                } else if ext.is_on {
                    ext.is_on = false;
                    Some(Pulse::Low)
                } else {
                    ext.is_on = true;
                    Some(Pulse::High)
                }
            }
            ModuleExt::Conjunction(ext) => {
                *ext.last_pulse.get_mut(from).unwrap() = pulse;
                if ext.last_pulse.values().all(|pulse| pulse == &Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }

    fn is_start(&self) -> bool {
        match &self.ext {
            ModuleExt::Broadcaster => true,
            ModuleExt::FlipFlop(ext) => !ext.is_on,
            ModuleExt::Conjunction(ext) => {
                ext.last_pulse.values().all(|pulse| pulse == &Pulse::Low)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Modules<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

impl<'a> Modules<'a> {
    fn from(s: &'a str) -> Self {
        // parse input
        let mut modules = HashMap::new();
        let mut last_pulses = Vec::new();
        for line in s.lines() {
            let (source, destinations) = line.split_once(" -> ").unwrap();
            let destinations: Vec<&str> = destinations.split(", ").collect();
            let type_ = source.as_bytes()[0];

            let name = if type_ == b'%' || type_ == b'&' {
                &source[1..]
            } else {
                assert_eq!(source, "broadcaster");
                source
            };

            for destination in destinations.iter().copied() {
                last_pulses.push((destination, name));
            }

            let module = if type_ == b'%' {
                Module::new_flipflop(destinations)
            } else if type_ == b'&' {
                Module::new_conjunction(destinations)
            } else {
                Module::new_broadcaster(destinations)
            };

            modules.insert(name, module);
        }

        // initialize inputs of conjunction modules
        for (destination, name) in last_pulses {
            if let Some(other) = modules.get_mut(destination) {
                if let ModuleExt::Conjunction(other) = &mut other.ext {
                    other.last_pulse.insert(name, Pulse::Low);
                }
            }
        }

        Modules { modules }
    }

    fn press_button(&mut self) -> (usize, usize) {
        let mut low_pulses = 0;
        let mut high_pulses = 0;
        let mut q = VecDeque::new();
        q.push_back(("broadcaster", "button", Pulse::Low));
        while let Some((name, from, pulse)) = q.pop_front() {
            if pulse == Pulse::Low {
                low_pulses += 1;
            } else {
                high_pulses += 1;
            }
            if let Some(module) = self.modules.get_mut(name) {
                if let Some(pulse) = module.apply(from, pulse) {
                    for destination in &module.destinations {
                        q.push_back((destination, name, pulse));
                    }
                }
            }
        }
        (low_pulses, high_pulses)
    }

    fn is_start(&self) -> bool {
        self.modules.values().all(Module::is_start)
    }
}

fn part1(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut modules = Modules::from(&data);

    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        let (low, high) = modules.press_button();
        low_pulses += low;
        high_pulses += high;
    }
    low_pulses * high_pulses
}

fn main() {
    assert_eq!(part1("example1"), 32000000);
    assert_eq!(part1("example2"), 11687500);
    assert_eq!(part1("input"), 747304011);
}

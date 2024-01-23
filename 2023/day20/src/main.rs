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

    fn press_button1(&mut self) -> (usize, usize) {
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

    fn press_button2(&mut self) -> bool {
        let mut q = VecDeque::new();
        q.push_back(("broadcaster", "button", Pulse::Low));
        while let Some((name, from, pulse)) = q.pop_front() {
            if pulse == Pulse::Low && name == "rx" {
                return true;
            }
            if let Some(module) = self.modules.get_mut(name) {
                if let Some(pulse) = module.apply(from, pulse) {
                    for destination in &module.destinations {
                        q.push_back((destination, name, pulse));
                    }
                }
            }
        }
        false
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
        let (low, high) = modules.press_button1();
        low_pulses += low;
        high_pulses += high;
    }
    low_pulses * high_pulses
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a.rem_euclid(b))
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(21, 6), 3);
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(21, 6), 42);
}

fn lcm_many<I: Iterator<Item = i64>>(it: I) -> i64 {
    it.reduce(lcm).unwrap()
}

fn main() {
    assert_eq!(part1("example1"), 32000000);
    assert_eq!(part1("example2"), 11687500);
    assert_eq!(part1("input"), 747304011);

    // See circuit.dia, and circuit.svg
    //
    // To turn rx to high, vr needs to turn to high
    // To turn vr to high, bm, cl, tn and vr need to turn to high
    // To turn bm, cl, tn and vr to high, respectively ds, dt, bd and cs need to turn to high
    // So, we need to find the first time when ds, dt, bd and cs are first set to high
    // I do not know why they are not directly connected to rx
    //
    // The DAG of dependency for each of ds, dt, bd and cs is independent for the other, save for
    // the broadcast node
    // In each, there is a chain of 12 flip-flop modules connected in series.
    // The broadcast node is connected to the first node in the series.
    // The state of the nodes changes with each button press:
    //
    // state: 0000 0000 0000
    //
    // broadcast sends LOW pulse to flip-flop 1
    // flip-flop 1 changes its state to ON and sends a HIGH pulse to flip-flop 2
    // flip-flop 2 ignores the HIGH pulse
    // state: 1000 0000 0000
    //
    // broadcast sends low pulse to flip-flop 1
    // flip-flop 1 changes its state to OFF and sends a LOW pulse to flip-flop 2
    // flip-flop 2 changes its state to ON, and sends a HIGH pulse to flip-flop 3
    // flip-flop 3 ignores the HIGH pulse
    // state: 0100 0000 0000
    //
    // broadcast sends low pulse to flip-flop 1
    // flip-flop 1 changes its state to ON and sends a HIGH pulse to flip-flop 2
    // flip-flop 2 ignores the HIGH pulse
    // state: 1100 0000 0000
    //
    // broadcast sends low pulse to flip-flop 1
    // flip-flop 1 changes its state to OFF and sends a LOW pulse to flip-flop 2
    // flip-flop 2 changes its state to OFF, and sends a LOW pulse to flip-flop 3
    // flip-flop 3 changes its state to ON, and sends a HIGH pulse to flip-flop 3
    // flip-flop 4 ignores the HIGH pulse
    // state: 0010 0000 0000
    //
    // In short, the chain counts the number of times the button was pressed.
    //
    // Certain flip-flop modules are connected to ds/dt/bd/cs.
    // The first flip-flop is always part of these.
    // The first time ds/dt/bd/cs turns to HIGH, these flip-flop modules are HIGH, and all the
    // other are LOW.
    // This happen after a number of button presses corresponding to the binary number represented
    // by the HIGH flip-flops.
    // When this happen, the ds/dt/bd/cs conjunction module sends a LOW pulse to:
    // - rx (through two modules); which turns HIGH and all four of ds, dt, bd and cs do so
    // - all of the LOW flip-flops, turning them HIGH as well, making the number 1111 1111 1111
    // - the first flip-flop, incrementing that number, and thus resetting all 12 flip-flops
    //
    // In short, ds/dt/bd/cs turns HIGH when the number of button presses is a multiple of a
    // certain number.
    // This number is:
    // ds: 3889
    // dt: 3943
    // bd: 3761
    // cs: 3821
    //
    // The first time rx turns to HIGH corresponds to the first time all four of ds/dt/bd/cs turn to HHIGH.
    // So it first turns HIGH when the number of button presses is the a smallest number which is a multiple of these four numbers.
    assert_eq!(lcm_many([3889, 3943, 3761, 3821].into_iter()), 0);
}

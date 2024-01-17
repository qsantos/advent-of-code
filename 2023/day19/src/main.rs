use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Category {
    /// Extremely cool looking
    X = 0,
    /// Musical
    M,
    /// Aerodynamic
    A,
    /// Shiny
    S,
    /// Number of categories
    SIZE,
}

impl Category {
    fn from(s: u8) -> Self {
        match s {
            b'x' => Category::X,
            b'm' => Category::M,
            b'a' => Category::A,
            b's' => Category::S,
            _ => panic!("unexpected category {s}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Comparison {
    LessThan,
    GreaterThan,
}

impl Comparison {
    fn from(s: u8) -> Self {
        match s {
            b'<' => Comparison::LessThan,
            b'>' => Comparison::GreaterThan,
            _ => panic!("unexpected comparison {s}"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ConditionalRule<'a> {
    category: Category,
    comparison: Comparison,
    threshold: u64,
    destination: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct UnconditionalRule<'a> {
    destination: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Rule<'a> {
    Conditional(ConditionalRule<'a>),
    Unconditional(UnconditionalRule<'a>),
}

impl<'a> Rule<'a> {
    fn from(s: &'a str) -> Self {
        if let Some((condition, destination)) = s.split_once(':') {
            let category = Category::from(condition.as_bytes()[0]);
            let comparison = Comparison::from(condition.as_bytes()[1]);
            let threshold = condition[2..].parse().unwrap();
            Rule::Conditional(ConditionalRule {
                category,
                comparison,
                threshold,
                destination,
            })
        } else {
            Rule::Unconditional(UnconditionalRule { destination: s })
        }
    }

    fn apply(&self, part: &Part) -> Option<&'a str> {
        match self {
            Rule::Conditional(rule) => {
                let ConditionalRule {
                    category,
                    comparison,
                    threshold,
                    destination,
                } = rule;
                let rating = part.ratings[*category as usize];
                let pass = match comparison {
                    Comparison::LessThan => rating < *threshold,
                    Comparison::GreaterThan => rating > *threshold,
                };
                pass.then_some(destination)
            }
            Rule::Unconditional(rule) => {
                let UnconditionalRule { destination } = rule;
                Some(destination)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn from(s: &'a str) -> (&'a str, Self) {
        let (name, rules) = s.strip_suffix('}').unwrap().split_once('{').unwrap();
        let rules = rules.split(',').map(|line| Rule::from(line)).collect();
        (name, Workflow { rules })
    }

    fn apply(&self, part: &Part) -> &'a str {
        for rule in self.rules.iter() {
            if let Some(destination) = rule.apply(part) {
                return destination;
            }
        }
        panic!("no rule matched the part");
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Part {
    ratings: [u64; Category::SIZE as usize],
}

impl Part {
    fn from(s: &str) -> Self {
        let mut ratings = [0; Category::SIZE as usize];
        for rating in s
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
        {
            let category = Category::from(rating.as_bytes()[0]);
            let value = rating[2..].parse().unwrap();
            ratings[category as usize] = value;
        }
        Part { ratings }
    }
}

struct System<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
}

impl<'a> System<'a> {
    fn from(s: &'a str) -> Self {
        let workflows = s.split('\n').map(|line| Workflow::from(line)).collect();
        System { workflows }
    }

    fn is_part_accepted(&self, part: &Part) -> bool {
        let mut workflow = &self.workflows["in"];
        loop {
            match workflow.apply(&part) {
                "A" => break true,
                "R" => break false,
                name => workflow = &self.workflows[name],
            }
        }
    }
}

fn part1(filename: &str) -> u64 {
    let data = std::fs::read_to_string(filename).unwrap();
    let (workflows, parts) = data.trim().split_once("\n\n").unwrap();
    let system = System::from(workflows);
    parts
        .split('\n')
        .map(|line| Part::from(line))
        .filter(|part| system.is_part_accepted(part))
        .map(|part| part.ratings.iter().sum::<u64>())
        .sum()
}

fn main() {
    assert_eq!(part1("example"), 19114);
    assert_eq!(part1("input"), 382440);
}

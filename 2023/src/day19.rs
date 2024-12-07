use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, Copy, Eq, PartialEq)]
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
    Size,
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

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Category::X => 'x',
                Category::M => 'm',
                Category::A => 'a',
                Category::S => 's',
                Category::Size => unreachable!(),
            }
        )
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
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

impl Debug for Comparison {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Comparison::LessThan => '<',
                Comparison::GreaterThan => '>',
            }
        )
    }
}

#[derive(Clone, Eq, PartialEq)]
struct ConditionalRule<'a> {
    category: Category,
    comparison: Comparison,
    threshold: u64,
    destination: &'a str,
}

#[derive(Clone, Eq, PartialEq)]
struct UnconditionalRule<'a> {
    destination: &'a str,
}

#[derive(Clone, Eq, PartialEq)]
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

    fn apply_set(&self, set: PartSet) -> (Option<(&'a str, PartSet)>, Option<PartSet>) {
        match self {
            Rule::Conditional(rule) => {
                let ConditionalRule {
                    category,
                    comparison,
                    threshold,
                    destination,
                } = rule;
                let start_rating = set.start_ratings[*category as usize];
                let end_rating = set.end_ratings[*category as usize];
                match comparison {
                    Comparison::LessThan => {
                        if end_rating < *threshold {
                            // whole range is below threshold, whole set passes the condition
                            (Some((destination, set)), None)
                        } else if start_rating < *threshold {
                            // part of the range is below the threshold, split the set
                            let mut below = set.clone();
                            let mut above = set;
                            below.end_ratings[*category as usize] = *threshold - 1;
                            // NOTE: above does include the threshold
                            above.start_ratings[*category as usize] = *threshold;
                            (Some((destination, below)), Some(above))
                        } else {
                            // whole range is above threshold, nothing passes the condition
                            (None, Some(set))
                        }
                    }
                    Comparison::GreaterThan => {
                        if start_rating > *threshold {
                            // whole range is above threshold, whole set passes the condition
                            (Some((destination, set)), None)
                        } else if end_rating > *threshold {
                            // part of the range is above the threshold, split the set
                            let mut above = set.clone();
                            let mut below = set;
                            above.start_ratings[*category as usize] = *threshold + 1;
                            // NOTE: below does include the threshold
                            below.end_ratings[*category as usize] = *threshold;
                            (Some((destination, above)), Some(below))
                        } else {
                            // whole range is above threshold, nothing passes the condition
                            (None, Some(set))
                        }
                    }
                }
            }
            Rule::Unconditional(rule) => {
                let UnconditionalRule { destination } = rule;
                (Some((destination, set)), None)
            }
        }
    }
}

impl<'a> Debug for Rule<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Rule::Conditional(rule) => {
                let ConditionalRule {
                    category,
                    comparison,
                    threshold,
                    destination,
                } = rule;
                write!(
                    f,
                    "{:?}{:?}{}:{}",
                    category, comparison, threshold, destination
                )
            }
            Rule::Unconditional(rule) => {
                let UnconditionalRule { destination } = rule;
                write!(f, "{}", destination)
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn from(s: &'a str) -> (&'a str, Self) {
        let (name, rules) = s.strip_suffix('}').unwrap().split_once('{').unwrap();
        let rules = rules.split(',').map(Rule::from).collect();
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

    fn apply_set(&self, mut set: PartSet) -> Vec<(&'a str, PartSet)> {
        let mut ret = Vec::new();
        for rule in self.rules.iter() {
            let (pass, fail) = rule.apply_set(set);
            ret.extend(pass);
            let Some(next) = fail else {
                break;
            };
            set = next;
        }
        ret
    }
}

impl<'a> Debug for Workflow<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut it = self.rules.iter();
        if let Some(rule) = it.next() {
            write!(f, "{:?}", rule)?;
        }
        for rule in it {
            write!(f, ",{:?}", rule)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Part {
    ratings: [u64; Category::Size as usize],
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PartSet {
    start_ratings: [u64; Category::Size as usize],
    end_ratings: [u64; Category::Size as usize],
}

impl Part {
    fn from(s: &str) -> Self {
        let mut ratings = [0; Category::Size as usize];
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
        let workflows = s.split('\n').map(Workflow::from).collect();
        System { workflows }
    }

    fn is_part_accepted(&self, part: &Part) -> bool {
        let mut workflow = &self.workflows["in"];
        loop {
            match workflow.apply(part) {
                "A" => break true,
                "R" => break false,
                name => workflow = &self.workflows[name],
            }
        }
    }

    fn acceptable_ratings(&self) -> u64 {
        let set = PartSet {
            start_ratings: [1; 4],
            end_ratings: [4000; 4],
        };
        let mut count = 0;
        let mut q = vec![("in", set)];
        while let Some((state, set)) = q.pop() {
            let workflow = &self.workflows[state];
            let res = workflow.apply_set(set);
            for (state, set) in res {
                if state == "R" {
                    continue;
                } else if state == "A" {
                    let mut c = 1;
                    for category in [Category::X, Category::M, Category::A, Category::S] {
                        let start_rating = set.start_ratings[category as usize];
                        let end_rating = set.end_ratings[category as usize];
                        c *= end_rating - start_rating + 1;
                    }
                    // NOTE: we split sets without duplicating parts, so all sets are disjoint
                    count += c;
                } else {
                    q.push((state, set));
                }
            }
        }
        count
    }
}

pub fn part1(input: &str) -> u64 {
    let (workflows, parts) = input.trim().split_once("\n\n").unwrap();
    let system = System::from(workflows);
    parts
        .split('\n')
        .map(Part::from)
        .filter(|part| system.is_part_accepted(part))
        .map(|part| part.ratings.iter().sum::<u64>())
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let (workflows, _) = input.trim().split_once("\n\n").unwrap();
    let system = System::from(workflows);
    system.acceptable_ratings()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day19.txt");
    const INPUT: &str = include_str!("../inputs/day19.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 19114);
        assert_eq!(part1(INPUT), 382440);

        assert_eq!(part2(EXAMPLE), 167409079868000);
        assert_eq!(part2(INPUT), 136394217540123);
    }
}

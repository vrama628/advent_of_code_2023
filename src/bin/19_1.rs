use std::{collections::HashMap, io::stdin, str::FromStr};

enum Category {
    X,
    M,
    A,
    S,
}

enum Comparison {
    LessThan,
    GreaterThan,
}

struct Condition {
    category: Category,
    comparison: Comparison,
    value: usize,
}

impl Condition {
    fn accepts(&self, part: &Part) -> bool {
        let category_value = match self.category {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
        };
        match self.comparison {
            Comparison::LessThan => category_value < self.value,
            Comparison::GreaterThan => category_value > self.value,
        }
    }
}

impl FromStr for Condition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let category = match chars.next().unwrap() {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => panic!(),
        };
        let comparison = match chars.next().unwrap() {
            '<' => Comparison::LessThan,
            '>' => Comparison::GreaterThan,
            _ => panic!(),
        };
        let value = chars.as_str().parse().unwrap();
        Ok(Self {
            category,
            comparison,
            value,
        })
    }
}

type WorkflowLabel = String;

enum Send {
    Intermediate(WorkflowLabel),
    Final(bool),
}

impl FromStr for Send {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Final(true),
            "R" => Self::Final(false),
            _ => Self::Intermediate(s.to_owned()),
        })
    }
}

struct Rule {
    condition: Option<Condition>,
    send: Send,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once(':') {
            None => Self {
                condition: None,
                send: s.parse().unwrap(),
            },
            Some((condition, send)) => Self {
                condition: Some(condition.parse().unwrap()),
                send: send.parse().unwrap(),
            },
        })
    }
}

struct Workflows(HashMap<WorkflowLabel, Vec<Rule>>);

impl Workflows {
    fn workflow_accepts(&self, workflow: &str, part: &Part) -> bool {
        let rules = self.0.get(workflow).unwrap();
        for rule in rules {
            if rule
                .condition
                .as_ref()
                .map_or(true, |condition| condition.accepts(part))
            {
                return match &rule.send {
                    Send::Intermediate(workflow) => self.workflow_accepts(workflow, part),
                    Send::Final(b) => *b,
                };
            }
        }
        panic!()
    }
    fn accepts(&self, part: &Part) -> bool {
        self.workflow_accepts("in", part)
    }
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',');
        let x = parts
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .parse()
            .unwrap();
        let m = parts
            .next()
            .unwrap()
            .strip_prefix("m=")
            .unwrap()
            .parse()
            .unwrap();
        let a = parts
            .next()
            .unwrap()
            .strip_prefix("a=")
            .unwrap()
            .parse()
            .unwrap();
        let s = parts
            .next()
            .unwrap()
            .strip_prefix("s=")
            .unwrap()
            .parse()
            .unwrap();
        Ok(Self { x, m, a, s })
    }
}

struct Input {
    workflows: Workflows,
    parts: Vec<Part>,
}

impl Input {
    fn parse() -> Self {
        let mut lines = stdin().lines().map(|line| line.unwrap());
        let mut workflows = Workflows(HashMap::new());
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            } else {
                let (label, rules) = line.split_once('{').unwrap();
                let label = label.to_owned();
                let rules = rules
                    .strip_suffix('}')
                    .unwrap()
                    .split(",")
                    .map(|rule| rule.parse().unwrap())
                    .collect();
                workflows.0.insert(label, rules);
            }
        }
        let parts = lines.map(|line| line.parse().unwrap()).collect();
        Self { workflows, parts }
    }

    fn solve(&self) -> usize {
        self.parts
            .iter()
            .filter(|part| self.workflows.accepts(part))
            .map(|part| part.sum())
            .sum()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}

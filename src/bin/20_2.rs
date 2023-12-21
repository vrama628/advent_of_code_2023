use std::{
    collections::{HashMap, VecDeque},
    io::stdin,
};

struct Pulse {
    source: String,
    destination: String,
    value: bool,
}

impl Pulse {
    fn button() -> Self {
        Self {
            source: "button".to_owned(),
            destination: "broadcaster".to_owned(),
            value: false,
        }
    }
}

struct PulseQueue {
    queue: VecDeque<Pulse>,
    button_presses: usize,
    rx_was_sent_low_pulse: bool,
}

impl PulseQueue {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            button_presses: 0,
            rx_was_sent_low_pulse: false,
        }
    }

    fn enqueue(&mut self, pulse: Pulse) {
        if pulse.source == "button" {
            self.button_presses += 1;
        }
        if pulse.destination == "rx" && !pulse.value {
            self.rx_was_sent_low_pulse = true;
        }
        self.queue.push_back(pulse);
    }

    fn dequeue(&mut self) -> Option<Pulse> {
        self.queue.pop_front()
    }
}

enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcast,
}

impl ModuleType {
    fn process(&mut self, pulse: &Pulse) -> Option<bool> {
        match self {
            ModuleType::FlipFlop(ref mut state) => {
                if pulse.value {
                    None
                } else {
                    *state = !*state;
                    Some(*state)
                }
            }
            ModuleType::Conjunction(state) => {
                state.insert(pulse.source.clone(), pulse.value);
                Some(!state.values().all(|v| *v))
            }
            ModuleType::Broadcast => Some(pulse.value),
        }
    }
}

struct Module {
    type_: ModuleType,
    destinations: Vec<String>,
}

struct Modules(HashMap<String, Module>);

impl Modules {
    fn initialize_conjunctions(&mut self) {
        let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
        for (label, Module { destinations, .. }) in self.0.iter() {
            for destination in destinations {
                inputs
                    .entry(destination.to_owned())
                    .or_default()
                    .push(label.to_owned());
            }
        }
        for (label, Module { type_, .. }) in self.0.iter_mut() {
            if let ModuleType::Conjunction(conjunction) = type_ {
                for input in inputs.get(label).unwrap() {
                    conjunction.insert(input.to_owned(), false);
                }
            }
        }
    }

    fn process_queue(&mut self, queue: &mut PulseQueue) {
        while let Some(pulse) = queue.dequeue() {
            if let Some(module) = self.0.get_mut(&pulse.destination) {
                if let Some(value) = module.type_.process(&pulse) {
                    for destination in module.destinations.iter() {
                        queue.enqueue(Pulse {
                            source: pulse.destination.to_owned(),
                            destination: destination.to_owned(),
                            value,
                        });
                    }
                }
            }
        }
    }
}

struct Input(Modules);

impl Input {
    fn parse() -> Self {
        let mut modules = Modules(
            stdin()
                .lines()
                .map(|line| line.unwrap())
                .map(|line| {
                    let (type_label, destinations) = line.split_once(" -> ").unwrap();
                    let (type_, label) = if let Some(label) = type_label.strip_prefix('%') {
                        (ModuleType::FlipFlop(false), label)
                    } else if let Some(label) = type_label.strip_prefix('&') {
                        (ModuleType::Conjunction(HashMap::new()), label)
                    } else {
                        (ModuleType::Broadcast, type_label)
                    };
                    let destinations = destinations
                        .split(", ")
                        .map(|destination| destination.to_owned())
                        .collect();
                    (
                        label.to_owned(),
                        Module {
                            type_,
                            destinations,
                        },
                    )
                })
                .collect(),
        );
        modules.initialize_conjunctions();
        Self(modules)
    }

    fn solve(&mut self) -> usize {
        let mut queue = PulseQueue::new();
        while !queue.rx_was_sent_low_pulse {
            queue.enqueue(Pulse::button());
            self.0.process_queue(&mut queue);
        }
        queue.button_presses
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}

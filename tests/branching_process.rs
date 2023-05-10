use cond_prob_sim::{select, Condition, Event, Outcome};
use num_bigint::BigUint;

#[derive(Debug, Clone)]
pub enum BPEvent {
    Split,
    Still,
    Die,
}

impl Event for BPEvent {}

#[derive(Debug)]
pub enum BPOutcome {
    Extinct,
}

impl Outcome for BPOutcome {}

pub struct BPCondition {
    population: BigUint,
}

impl BPCondition {
    pub fn new() -> Self {
        Self {
            population: BigUint::from(1_usize),
        }
    }
}

impl Default for BPCondition {
    fn default() -> Self {
        Self::new()
    }
}

impl Condition for BPCondition {
    type Event = BPEvent;
    type Outcome = BPOutcome;

    fn push(&mut self, event: Self::Event) {
        match event {
            BPEvent::Split => self.population += 1_usize,
            BPEvent::Still => self.population += 0_usize,
            BPEvent::Die => self.population -= 1_usize,
        }
    }

    fn outcome(&self) -> Option<Self::Outcome> {
        if self.population == BigUint::from(0_usize) {
            Some(BPOutcome::Extinct)
        } else {
            None
        }
    }

    fn select_event(&self) -> Self::Event {
        let space = &[
            (1. / 3., BPEvent::Split),
            (1. / 3., BPEvent::Still),
            (1. / 3., BPEvent::Die),
        ];
        select(space).clone()
    }
}

#[test]
fn rounds_100() {
    let rounds = 100;
    let sim = cond_prob_sim::RoundSimulator;
    let mut extinct = 0;
    for _ in 0..rounds {
        let start = BPCondition::new();
        let outcome = sim.run::<BPEvent, BPOutcome, BPCondition>(start);
        match outcome {
            BPOutcome::Extinct => extinct += 1,
        }
    }
    let p_extinct = extinct as f64 / rounds as f64;
    println!("p_extinct = {}", p_extinct);
    assert!(p_extinct == 1.);
}

use cond_prob_sim::{select, Condition, Event, Outcome};

#[derive(Debug, Clone)]
pub enum BernEvent {
    Success,
    Failure,
}

impl Event for BernEvent {}

impl Outcome for BernEvent {}

pub struct BernCondition {
    event: Option<BernEvent>,
    /// Probability of success.
    p: f64,
}

impl BernCondition {
    fn new(p: f64) -> Self {
        Self { event: None, p }
    }
}

impl Condition for BernCondition {
    type Event = BernEvent;
    type Outcome = BernEvent;

    fn push(&mut self, event: Self::Event) {
        assert!(self.event.is_none());
        self.event = Some(event);
    }

    fn select_event(&self) -> Self::Event {
        let q = 1. - self.p;
        let space = &[(self.p, BernEvent::Success), (q, BernEvent::Failure)];
        select(space).clone()
    }

    fn outcome(&self) -> Option<Self::Outcome> {
        self.event.clone()
    }
}

#[test]
fn rounds_1_000_000_p_0_2() {
    let rounds = 1_000_000;
    let p = 0.2;
    let sim = cond_prob_sim::RoundSimulator;
    let mut successes = 0;
    let mut failures = 0;
    for _ in 0..rounds {
        let start = BernCondition::new(p);
        let outcome = sim.run(start);
        match outcome {
            BernEvent::Success => successes += 1,
            BernEvent::Failure => failures += 1,
        }
    }
    let p_success = successes as f64 / rounds as f64;
    let p_failure = failures as f64 / rounds as f64;
    println!("p_success = {}", p_success);
    println!("p_failure = {}", p_failure);
    assert!((p_success - p).abs() < 0.01);
}

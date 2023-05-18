use cond_prob_sim::{sample, select, Condition, Event, Outcome};

#[derive(Debug, Clone)]
pub enum MHEvent {
    Door1,
    Door2,
    Door3,
}

impl Event for MHEvent {}

#[derive(Debug)]
pub enum MHOutcome {
    Win,
    Lose,
}

impl Outcome for MHOutcome {}

pub struct MHCondition {
    /// Tuple format:
    /// > (Your choice, Car's location, Host's choice, Whether you switch)
    ///
    /// You always switch in the end.
    events: Vec<MHEvent>,
}

impl MHCondition {
    pub fn new() -> Self {
        Self {
            events: Vec::with_capacity(3),
        }
    }
}

impl Default for MHCondition {
    fn default() -> Self {
        Self::new()
    }
}

impl Condition for MHCondition {
    type Event = MHEvent;
    type Outcome = MHOutcome;

    fn push(&mut self, event: Self::Event) {
        self.events.push(event);
    }

    fn outcome(&self) -> Option<Self::Outcome> {
        match self.events.as_slice() {
            [MHEvent::Door1, MHEvent::Door1, MHEvent::Door2] => Some(MHOutcome::Lose),
            [MHEvent::Door1, MHEvent::Door1, MHEvent::Door3] => Some(MHOutcome::Lose),
            [MHEvent::Door1, MHEvent::Door2, MHEvent::Door3] => Some(MHOutcome::Win),
            [MHEvent::Door1, MHEvent::Door3, MHEvent::Door2] => Some(MHOutcome::Win),
            _ => None,
        }
    }

    fn select_event(&self) -> Self::Event {
        let space: &[(f64, Self::Event)] = match self.events.as_slice() {
            [MHEvent::Door1] => &[
                (1.0 / 3.0, MHEvent::Door1),
                (1.0 / 3.0, MHEvent::Door2),
                (1.0 / 3.0, MHEvent::Door3),
            ],
            [MHEvent::Door1, MHEvent::Door1] => {
                &[(1.0 / 2.0, MHEvent::Door2), (1.0 / 2.0, MHEvent::Door3)]
            }
            [MHEvent::Door1, MHEvent::Door2] => &[(1.0, MHEvent::Door3)],
            [MHEvent::Door1, MHEvent::Door3] => &[(1.0, MHEvent::Door2)],
            _ => unreachable!(),
        };
        select(space).clone()
    }
}

#[test]
fn rounds_1_000_000() {
    let rounds = 1_000_000;
    let mut wins = 0;
    let mut losses = 0;
    for _ in 0..rounds {
        let mut start = MHCondition::new();
        start.push(MHEvent::Door1);
        let outcome = sample(start);
        match outcome {
            MHOutcome::Win => wins += 1,
            MHOutcome::Lose => losses += 1,
        }
    }
    let p_win = wins as f64 / rounds as f64;
    let p_lose = losses as f64 / rounds as f64;
    println!("p_win = {}", p_win);
    println!("p_lose = {}", p_lose);
    assert!(p_win > 0.66 && p_win < 0.67);
    assert!(p_lose > 0.33 && p_lose < 0.34);
}

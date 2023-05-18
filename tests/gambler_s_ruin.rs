use cond_prob_sim::{sample, select, Condition, Event, Outcome};

#[derive(Debug, Clone)]
pub struct GREvent(usize);

impl Event for GREvent {}

#[derive(Debug)]
pub enum GROutcome {
    AWin,
    BWin,
}

impl Outcome for GROutcome {}

pub struct GRCondition {
    /// Amount of money A has.
    event: GREvent,
    /// Criteria for A winning.
    n: usize,
    /// Probability of A winning.
    p: f64,
}

impl GRCondition {
    pub fn new(i: usize, n: usize, p: f64) -> Self {
        Self {
            event: GREvent(i),
            n,
            p,
        }
    }
}

impl Condition for GRCondition {
    type Event = GREvent;
    type Outcome = GROutcome;

    fn push(&mut self, event: Self::Event) {
        self.event = event;
    }

    fn outcome(&self) -> Option<Self::Outcome> {
        assert!(self.event.0 <= self.n);
        match self.event.0 {
            0 => Some(GROutcome::BWin),
            x if x == self.n => Some(GROutcome::AWin),
            _ => None,
        }
    }

    fn select_event(&self) -> Self::Event {
        assert!(self.event.0 > 0);
        assert!(self.event.0 < self.n);
        let q = 1. - self.p;
        let space = &[
            (self.p, GREvent(self.event.0 + 1)),
            (q, GREvent(self.event.0 - 1)),
        ];
        select(space).clone()
    }
}

#[test]
fn i_5_n_100_p_0_5() {
    let rounds = 100_000;
    let i = 5;
    let n = 100;
    let p = 0.5;
    println!("i = {}", i);
    println!("n = {}", n);
    println!("p = {}", p);
    let mut a_wins = 0;
    let mut b_wins = 0;
    for _ in 0..rounds {
        let start = GRCondition::new(i, n, p);
        let outcome = sample(start);
        match outcome {
            GROutcome::AWin => a_wins += 1,
            GROutcome::BWin => b_wins += 1,
        }
    }
    let p_a_wins = a_wins as f64 / rounds as f64;
    let p_b_wins = b_wins as f64 / rounds as f64;
    println!("p(A wins) = {}", p_a_wins);
    println!("p(B wins) = {}", p_b_wins);
    assert!((p_a_wins - 0.05).abs() < 0.01);
    assert!((p_b_wins - 0.95).abs() < 0.01);
}

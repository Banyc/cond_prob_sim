use cond_prob_sim::{select, Condition, Event, Outcome};

#[derive(Debug, Clone)]
pub enum HGeomEvent {
    Success,
    Failure,
}

impl Event for HGeomEvent {}

pub struct HGeomOutcome {
    pub successes: usize,
    pub failures: usize,
}

impl Outcome for HGeomOutcome {}

pub struct HGeomCondition {
    successes: usize,
    failures: usize,
    successes_remaining: usize,
    failures_remaining: usize,
    /// Number of draws.
    n: usize,
}

impl HGeomCondition {
    pub fn new(successes_remaining: usize, failures_remaining: usize, n: usize) -> Self {
        Self {
            successes: 0,
            failures: 0,
            successes_remaining,
            failures_remaining,
            n,
        }
    }
}

impl Condition for HGeomCondition {
    type Event = HGeomEvent;
    type Outcome = HGeomOutcome;

    fn push(&mut self, event: Self::Event) {
        match &event {
            HGeomEvent::Success => {
                self.successes_remaining -= 1;
                self.successes += 1;
            }
            HGeomEvent::Failure => {
                self.failures_remaining -= 1;
                self.failures += 1;
            }
        }
    }

    fn select_event(&self) -> Self::Event {
        let p = self.successes_remaining as f64
            / (self.successes_remaining + self.failures_remaining) as f64;
        let q = 1. - p;
        let space = &[(p, HGeomEvent::Success), (q, HGeomEvent::Failure)];
        select(space).clone()
    }

    fn outcome(&self) -> Option<Self::Outcome> {
        assert!(self.successes + self.failures <= self.n);
        if self.successes + self.failures == self.n {
            Some(HGeomOutcome {
                successes: self.successes,
                failures: self.failures,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_1_000_000_n_10_s_5_f_45() {
        let rounds = 1_000_000;
        let n = 10;
        let successes_remaining = 5;
        let failures_remaining = 45;
        let sim = cond_prob_sim::RoundSimulator;
        let mut counts = vec![0; n + 1];
        for _ in 0..rounds {
            let start = HGeomCondition::new(successes_remaining, failures_remaining, n);
            let outcome = sim.run(start);
            counts[outcome.successes] += 1;
        }
        let prob_mass_func = counts
            .iter()
            .map(|&x| x as f64 / rounds as f64)
            .collect::<Vec<_>>();
        println!("PMF: {:?}", prob_mass_func);
    }
}
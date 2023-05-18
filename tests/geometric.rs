use cond_prob_sim::{select, Condition, Event, Outcome};

#[derive(Debug, Clone)]
pub enum GeomEvent {
    Success,
    Failure,
}

impl Event for GeomEvent {}

pub struct GeomOutcome {
    pub successes: usize,
}

impl Outcome for GeomOutcome {}

pub struct GeomCondition {
    successes: usize,
    failed: bool,
    /// Probability of success.
    p: f64,
}

impl GeomCondition {
    pub fn new(p: f64) -> Self {
        Self {
            successes: 0,
            failed: false,
            p,
        }
    }
}

impl Condition for GeomCondition {
    type Event = GeomEvent;
    type Outcome = GeomOutcome;

    fn push(&mut self, event: Self::Event) {
        assert!(!self.failed);
        match &event {
            GeomEvent::Success => self.successes += 1,
            GeomEvent::Failure => self.failed = true,
        }
    }

    fn select_event(&self) -> Self::Event {
        let q = 1. - self.p;
        let space = &[(self.p, GeomEvent::Success), (q, GeomEvent::Failure)];
        select(space).clone()
    }

    fn outcome(&self) -> Option<Self::Outcome> {
        if self.failed {
            Some(GeomOutcome {
                successes: self.successes,
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
    fn rounds_1_000_000_p_0_5() {
        let rounds = 1_000_000;
        let n = 20;
        let p = 0.5;
        let sim = cond_prob_sim::RoundSimulator;
        let mut counts = vec![0; n];
        for _ in 0..rounds {
            let start = GeomCondition::new(p);
            let outcome = sim.run(start);
            if outcome.successes >= counts.len() {
                continue;
            }
            counts[outcome.successes] += 1;
        }
        let prob_mass_func = counts
            .iter()
            .map(|&x| x as f64 / rounds as f64)
            .collect::<Vec<_>>();
        println!("PMF: {:?}", prob_mass_func);
        for (i, p) in prob_mass_func.iter().enumerate() {
            assert!((p - 0.5_f64.powi(i as i32) * 0.5).abs() < 0.01);
        }
    }
}

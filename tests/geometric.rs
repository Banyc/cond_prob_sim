use cond_prob_sim::{select, Condition, Event, Outcome};

#[derive(Debug, Clone)]
pub enum GeomEvent {
    Success,
    Failure,
}

impl Event for GeomEvent {}

pub struct GeomOutcome {
    pub failures: usize,
}

impl Outcome for GeomOutcome {}

pub struct GeomCondition {
    failures: usize,
    succeeded: bool,
    /// Probability of success.
    p: f64,
}

impl GeomCondition {
    pub fn new(p: f64) -> Self {
        Self {
            failures: 0,
            succeeded: false,
            p,
        }
    }
}

impl Condition for GeomCondition {
    type Event = GeomEvent;
    type Outcome = GeomOutcome;

    fn push(&mut self, event: Self::Event) {
        assert!(!self.succeeded);
        match &event {
            GeomEvent::Success => self.succeeded = true,
            GeomEvent::Failure => self.failures += 1,
        }
    }

    fn select_event(&self) -> Self::Event {
        let q = 1. - self.p;
        let space = &[(self.p, GeomEvent::Success), (q, GeomEvent::Failure)];
        select(space).clone()
    }

    fn outcome(&self) -> Option<Self::Outcome> {
        if self.succeeded {
            Some(GeomOutcome {
                failures: self.failures,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use cond_prob_sim::sample;

    use super::*;

    #[test]
    fn rounds_1_000_000_p_0_5() {
        let rounds = 1_000_000;
        let n = 20;
        let p = 0.5;
        let mut counts = vec![0; n];
        for _ in 0..rounds {
            let start = GeomCondition::new(p);
            let outcome = sample(start);
            if outcome.failures >= counts.len() {
                continue;
            }
            counts[outcome.failures] += 1;
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

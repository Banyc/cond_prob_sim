use cond_prob_sim::{select, Condition, Event, NonnegativeRandomVariable, Outcome, StartCondition};

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

pub struct GeomStartCondition {
    pub p: f64,
}

impl StartCondition for GeomStartCondition {
    type Event = GeomEvent;
    type Outcome = GeomOutcome;
    type Condition = GeomCondition;

    fn build(&self) -> Self::Condition {
        GeomCondition::new(self.p)
    }
}

pub struct GeomRandomVariable {
    pub n: usize,
}

impl NonnegativeRandomVariable for GeomRandomVariable {
    type Outcome = GeomOutcome;

    fn map(&self, outcome: Self::Outcome) -> usize {
        outcome.failures
    }

    fn space_len(&self) -> usize {
        self.n + 1
    }
}

#[cfg(test)]
mod tests {
    use cond_prob_sim::sample_repeat;

    use super::*;

    #[test]
    fn rounds_1_000_000_p_0_5() {
        let rounds = 1_000_000;
        let n = 20;
        let p = 0.5;
        let mass = sample_repeat(GeomStartCondition { p }, rounds, GeomRandomVariable { n });
        let prob_mass_func = mass
            .iter()
            .map(|&x| x as f64 / rounds as f64)
            .collect::<Vec<_>>();
        println!("PMF: {:?}", prob_mass_func);
        for (i, p) in prob_mass_func.iter().enumerate() {
            assert!((p - 0.5_f64.powi(i as i32) * 0.5).abs() < 0.01);
        }
    }
}

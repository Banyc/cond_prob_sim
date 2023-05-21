use cond_prob_sim::{select, Condition, Event, NonnegativeRandomVariable, Outcome, StartCondition};

#[derive(Debug, Clone)]
pub enum NBinEvent {
    Success,
    Failure,
}

impl Event for NBinEvent {}

pub struct NBinOutcome {
    pub failures: usize,
}

impl Outcome for NBinOutcome {}

pub struct NBinCondition {
    successes: usize,
    failures: usize,
    /// Probability of success.
    p: f64,
    /// Number of successes required.
    r: usize,
}

impl NBinCondition {
    pub fn new(r: usize, p: f64) -> Self {
        Self {
            successes: 0,
            failures: 0,
            p,
            r,
        }
    }
}

impl Condition for NBinCondition {
    type Event = NBinEvent;
    type Outcome = NBinOutcome;

    fn push(&mut self, event: Self::Event) {
        match &event {
            NBinEvent::Success => self.successes += 1,
            NBinEvent::Failure => self.failures += 1,
        }
    }

    fn select_event(&self) -> Self::Event {
        let q = 1. - self.p;
        let space = &[(self.p, NBinEvent::Success), (q, NBinEvent::Failure)];
        select(space).clone()
    }

    fn outcome(&self) -> Option<Self::Outcome> {
        assert!(self.successes <= self.r);
        if self.successes == self.r {
            Some(NBinOutcome {
                failures: self.failures,
            })
        } else {
            None
        }
    }
}

pub struct NBinStartCondition {
    pub r: usize,
    pub p: f64,
}

impl StartCondition for NBinStartCondition {
    type Event = NBinEvent;
    type Outcome = NBinOutcome;
    type Condition = NBinCondition;

    fn build(&self) -> Self::Condition {
        NBinCondition::new(self.r, self.p)
    }
}

pub struct NBinRandomVariable {
    pub n: usize,
}

impl NonnegativeRandomVariable for NBinRandomVariable {
    type Outcome = NBinOutcome;

    fn map(&self, outcome: Self::Outcome) -> usize {
        outcome.failures
    }

    fn space_len(&self) -> usize {
        self.n + 1
    }
}

#[cfg(test)]
mod tests {
    use cond_prob_sim::{prob_mass_func, sample_repeat};

    use super::*;

    #[test]
    fn rounds_100_000_r_10_p_0_2() {
        let rounds = 100_000;
        let n = 100;
        let r = 10;
        let p = 0.2;
        let mass = sample_repeat(
            NBinStartCondition { r, p },
            rounds,
            NBinRandomVariable { n },
        );
        let prob_mass_func = prob_mass_func(&mass, rounds);
        println!("PMF: {:?}", prob_mass_func);
    }
}

use cond_prob_sim::{select, Condition, Event, NonnegativeRandomVariable, Outcome, StartCondition};

#[derive(Debug, Clone)]
pub enum BinEvent {
    Success,
    Failure,
}

impl Event for BinEvent {}

pub struct BinOutcome {
    pub successes: usize,
    pub failures: usize,
}

impl Outcome for BinOutcome {}

pub struct BinCondition {
    successes: usize,
    failures: usize,
    /// Probability of success.
    p: f64,
    /// Number of trials.
    n: usize,
}

impl BinCondition {
    pub fn new(n: usize, p: f64) -> Self {
        Self {
            successes: 0,
            failures: 0,
            p,
            n,
        }
    }
}

impl Condition for BinCondition {
    type Event = BinEvent;
    type Outcome = BinOutcome;

    fn push(&mut self, event: Self::Event) {
        match &event {
            BinEvent::Success => self.successes += 1,
            BinEvent::Failure => self.failures += 1,
        }
    }

    fn select_event(&self) -> Self::Event {
        let q = 1. - self.p;
        let space = &[(self.p, BinEvent::Success), (q, BinEvent::Failure)];
        select(space).clone()
    }

    fn outcome(&self) -> Option<Self::Outcome> {
        assert!(self.successes + self.failures <= self.n);
        if self.successes + self.failures == self.n {
            Some(BinOutcome {
                successes: self.successes,
                failures: self.failures,
            })
        } else {
            None
        }
    }
}

pub struct BinStartCondition {
    pub n: usize,
    pub p: f64,
}

impl StartCondition for BinStartCondition {
    type Event = BinEvent;
    type Outcome = BinOutcome;
    type Condition = BinCondition;

    fn build(&self) -> Self::Condition {
        BinCondition::new(self.n, self.p)
    }
}

pub struct BinRandomVariable {
    pub n: usize,
}

impl NonnegativeRandomVariable for BinRandomVariable {
    type Outcome = BinOutcome;

    fn map(&self, outcome: Self::Outcome) -> usize {
        outcome.successes
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
    fn rounds_1_000_000_n_10_p_0_2() {
        let rounds = 1_000_000;
        let n = 10;
        let p = 0.2;
        let mass = sample_repeat(BinStartCondition { n, p }, rounds, BinRandomVariable { n });
        let prob_mass_func = mass
            .iter()
            .map(|&x| x as f64 / rounds as f64)
            .collect::<Vec<_>>();
        println!("PMF: {:?}", prob_mass_func);
        // ref: <https://www.sjsu.edu/people/saul.cohn/courses/stats/s0/BinomialProbabTable.pdf>
        assert!((prob_mass_func[0] - 0.107).abs() < 0.01);
        assert!((prob_mass_func[1] - 0.268).abs() < 0.01);
        assert!((prob_mass_func[2] - 0.302).abs() < 0.01);
        assert!((prob_mass_func[3] - 0.201).abs() < 0.01);
        assert!((prob_mass_func[4] - 0.088).abs() < 0.01);
        assert!((prob_mass_func[5] - 0.026).abs() < 0.01);
        assert!((prob_mass_func[6] - 0.006).abs() < 0.01);
        assert!((prob_mass_func[7] - 0.001).abs() < 0.01);
        assert!((prob_mass_func[8] - 0.000).abs() < 0.01);
        assert!((prob_mass_func[9] - 0.000).abs() < 0.01);
        assert!((prob_mass_func[10] - 0.000).abs() < 0.01);
    }
}

use cond_prob_sim::{select, Condition, Event, NonnegativeRandomVariable, Outcome, StartCondition};

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

pub struct BernStartCondition {
    pub p: f64,
}

impl StartCondition for BernStartCondition {
    type Event = BernEvent;
    type Outcome = BernEvent;
    type Condition = BernCondition;

    fn build(&self) -> Self::Condition {
        BernCondition::new(self.p)
    }
}

pub struct BernRandomVariable;

impl NonnegativeRandomVariable for BernRandomVariable {
    type Outcome = BernEvent;

    fn map(&self, outcome: Self::Outcome) -> usize {
        match outcome {
            BernEvent::Success => 1,
            BernEvent::Failure => 0,
        }
    }

    fn space_len(&self) -> usize {
        2
    }
}

#[cfg(test)]
mod tests {
    use cond_prob_sim::{prob_mass_func, sample_repeat};

    use super::*;

    #[test]
    fn rounds_1_000_000_p_0_2() {
        let rounds = 1_000_000;
        let p = 0.2;
        let mass = sample_repeat(BernStartCondition { p }, rounds, BernRandomVariable);
        let prob_mass_func = prob_mass_func(&mass, rounds);
        println!("p_success = {}", prob_mass_func[1]);
        println!("p_failure = {}", prob_mass_func[0]);
        assert!((prob_mass_func[1] - p).abs() < 0.01);
    }
}

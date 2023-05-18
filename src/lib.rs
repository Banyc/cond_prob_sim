use rand::Rng;

pub trait Event {}

/// Context that walks through a sequence of choices until an outcome is reached.
pub trait Condition {
    type Event: Event;
    type Outcome: Outcome;

    /// Incorporate an event into the probability condition.
    fn push(&mut self, event: Self::Event);
    /// Select an event according to the probability given the current condition.
    fn select_event(&self) -> Self::Event;
    /// Return the outcome if the sequence of choices leads to one.
    fn outcome(&self) -> Option<Self::Outcome>;
}

pub trait Outcome {}

/// Run a simulation until an outcome is reached.
pub fn sample<E, O, C>(start: C) -> O
where
    E: Event,
    O: Outcome,
    C: Condition<Event = E, Outcome = O>,
{
    let mut cond = start;
    loop {
        if let Some(outcome) = cond.outcome() {
            return outcome;
        }

        let event = cond.select_event();
        cond.push(event);
    }
}

/// Select an event from a space of events with given probabilities.
pub fn select<E>(space: &[(f64, E)]) -> &E {
    let mut rng = rand::thread_rng();
    let r: f64 = rng.gen_range(0.0..1.0);
    let mut sum = 0.0;
    let mut event = None;
    for (p, e) in space {
        sum += p;
        if r < sum {
            event = Some(e);
            break;
        }
    }
    event.unwrap()
}

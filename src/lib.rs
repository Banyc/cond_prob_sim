use rand::Rng;

pub trait Event: Clone {}

pub trait Condition {
    type Event: Event;
    type Outcome: Outcome;

    fn push(&mut self, event: Self::Event);
    fn select_event(&self) -> Self::Event;
    fn outcome(&self) -> Option<Self::Outcome>;
}

pub trait Outcome {}

pub struct RoundSimulator;

impl RoundSimulator {
    pub fn run<E, O, C>(&self, start: C) -> O
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
}

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

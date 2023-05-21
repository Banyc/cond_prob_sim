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

pub trait StartCondition {
    type Event: Event;
    type Outcome: Outcome;
    type Condition: Condition<Event = Self::Event, Outcome = Self::Outcome>;

    fn build(&self) -> Self::Condition;
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

pub trait NonnegativeRandomVariable {
    type Outcome: Outcome;

    fn map(&self, outcome: Self::Outcome) -> usize;
    /// The number of possible values the random variable can take.
    ///
    /// $0, 1, 2, ..., \text{space_len} - 1$
    fn space_len(&self) -> usize;
}

/// Sample a random variable a number of times and return the number of times each value was
pub fn sample_repeat<S, RV, O>(start: S, rounds: usize, rv: RV) -> Vec<usize>
where
    O: Outcome,
    S: StartCondition<Outcome = O>,
    RV: NonnegativeRandomVariable<Outcome = O>,
{
    let mut mass = vec![0; rv.space_len()];
    for _ in 0..rounds {
        let outcome = sample(start.build());
        let v = rv.map(outcome);
        if v >= mass.len() {
            continue;
        }
        mass[v] += 1;
    }
    mass
}

pub fn prob_mass_func(mass: &[usize], rounds: usize) -> Vec<f64> {
    mass.iter().map(|&x| x as f64 / rounds as f64).collect()
}

pub fn expectation(pmf: &[f64]) -> f64 {
    pmf.iter().enumerate().map(|(x, &p)| x as f64 * p).sum()
}

pub fn variance(pmf: &[f64], expectation: f64) -> f64 {
    pmf.iter()
        .enumerate()
        .map(|(x, &p)| (x as f64 - expectation).powi(2) * p)
        .sum()
}

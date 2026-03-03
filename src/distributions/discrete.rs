//! Discrete probability distributions.

use rand::Rng;
use std::collections::HashMap;
use std::hash::Hash;

/// A discrete probability distribution consists of a set of events and their
/// probabilities.
#[derive(Debug)]
pub struct Discrete<Event> {
    distribution: HashMap<Event, f64>,
}

impl<Event> Discrete<Event> {
    /// The `support` of a discrete distribution is its set of events.
    pub fn support<'a>(&'a self) -> impl Iterator<Item = &'a Event>
    where
        Event: 'a,
    {
        self.distribution.keys()
    }

    /// Marginalize a distribution by converting the original events to
    /// new events and combining their respective probabilities.
    pub fn marginalize<NewEvent, Conversion>(
        &self,
        convert: Conversion,
    ) -> Discrete<NewEvent>
    where
        NewEvent: Clone + Eq + Hash,
        Conversion: Fn(&Event) -> NewEvent,
    {
        let mut distribution = HashMap::new();
        for (old_event, probability) in self.distribution.iter() {
            let new_event = convert(old_event);
            let entry = distribution.entry(new_event).or_insert(0.0);
            *entry += probability;
        }
        Discrete::build(distribution)
    }

    /// Build a probability distribution. Ensure that the probabilities
    /// sum to 1.
    fn build(events: HashMap<Event, f64>) -> Discrete<Event> {
        assert_ne!(
            0,
            events.len(),
            "Discrete distribution must have at least one event."
        );

        let total: f64 = events.values().sum();
        assert!(
            0.0 < total,
            "Sum of discrete event probabilities must be greater than 0."
        );

        let mut events = events;
        for probability in events.values_mut() {
            *probability /= total;
        }

        Discrete {
            distribution: events,
        }
    }
}

impl<Event> Discrete<Event>
where
    Event: Eq + Hash,
{
    /// Get the probability of an event.
    pub fn probability(&self, event: &Event) -> f64 {
        self.distribution.get(event).copied().unwrap_or(0.0)
    }

    /// Randomly draw an event from the distribution.
    pub fn draw(&self) -> &Event {
        let number: f64 = rand::rng().random_range(0.0..=1.0);
        let mut sum = 0.0;

        for event in self.support() {
            sum += self.probability(event);
            if number < sum {
                return event;
            }
        }
        self.support().last().unwrap()
    }
}

impl<Event> Discrete<Event>
where
    Event: Clone + Eq + Hash,
{
    /// Condition a distribution to an event. `filter` specifies the
    /// events that you are conditioning for.
    pub fn condition<Filter>(&self, filter: Filter) -> Discrete<Event>
    where
        Filter: FnMut(&&Event) -> bool,
    {
        Discrete::from_iter(
            self.support()
                .filter(filter)
                .map(|event| (event.clone(), self.probability(event))),
        )
    }
}

impl<Event> Discrete<Event> {}

impl<Event, const N: usize> From<[(Event, f64); N]> for Discrete<Event>
where
    Event: Eq + Hash,
{
    /// Convert an array of events to a discrete probability distribution.
    fn from(events: [(Event, f64); N]) -> Self {
        Self::build(HashMap::from(events))
    }
}

impl<Event> FromIterator<(Event, f64)> for Discrete<Event>
where
    Event: Eq + Hash,
{
    /// Convert an iterator of events to a discrete probability distribution.
    fn from_iter<Iterator>(events: Iterator) -> Self
    where
        Iterator: IntoIterator<Item = (Event, f64)>,
    {
        Self::build(HashMap::from_iter(events))
    }
}

impl<Event> PartialEq for Discrete<Event>
where
    Event: Eq + Hash,
{
    /// Compare two discrete distributions.
    fn eq(&self, other: &Self) -> bool {
        if self.support().count() != other.support().count() {
            return false;
        }

        self.support().all(|support| {
            let probability1 = self.probability(support);
            let probability2 = other.probability(support);
            (probability1 - probability2).abs() < 0.001
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_of_distribution_a() {
        let d_a = distribution_a();
        let mut support: Vec<_> = d_a.support().collect();
        support.sort();
        assert_eq!(support, [&"a1", &"a2"]);
    }

    #[test]
    fn probability_of_events_in_distribution_a() {
        let d_a = distribution_a();
        assert_eq!(d_a.probability(&"a1"), 0.9);
        assert_eq!(d_a.probability(&"a2"), 0.1);
    }

    #[test]
    fn draw_from_distribution_a() {
        let d_a = distribution_a();
        let mut results = HashMap::new();
        for _ in 1..=100 {
            let event = *d_a.draw();
            *results.entry(event).or_insert(0) += 1;
        }
        assert!(results["a1"] > results["a2"]);
    }

    #[test]
    fn marginalize_distribution_without_a() {
        assert_eq!(
            distribution_a_and_b().marginalize(|(_, b)| *b),
            Discrete::from([("b1", 0.65), ("b2", 0.35),])
        )
    }

    #[test]
    fn marginalize_distribution_without_b() {
        assert_eq!(
            distribution_a_and_b().marginalize(|(a, _)| *a),
            Discrete::from([("a1", 0.90), ("a2", 0.10),])
        )
    }

    #[test]
    fn conditional_distribution_to_b1() {
        assert_eq!(
            distribution_a_and_b()
                .condition(|(_, b)| b == &"b1")
                .marginalize(|(a, _)| *a),
            Discrete::from([("a1", 0.97), ("a2", 0.03),])
        )
    }

    type A = &'static str;
    type B = &'static str;

    fn distribution_a() -> Discrete<A> {
        Discrete::from([("a1", 0.9), ("a2", 0.1)])
    }

    fn distribution_a_and_b() -> Discrete<(A, B)> {
        Discrete::from([
            (("a1", "b1"), 0.63),
            (("a1", "b2"), 0.27),
            (("a2", "b1"), 0.02),
            (("a2", "b2"), 0.08),
        ])
    }
}

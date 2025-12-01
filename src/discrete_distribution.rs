use super::conditional_distribution::ConditionalDistribution;
use rand::Rng;
use std::collections::HashMap;
use std::hash::Hash;

pub struct DiscreteDistribution<Event>
where
    Event: Copy + Eq + Hash,
{
    distribution: HashMap<Event, f64>,
}

impl<Event> DiscreteDistribution<Event>
where
    Event: Copy + Eq + Hash,
{
    fn from_events(events: HashMap<Event, f64>) -> DiscreteDistribution<Event> {
        assert_ne!(
            0,
            events.len(),
            "Discrete distribution must have at least one event."
        );

        let total: f64 = events.values().sum();
        assert!(
            0.0 < total,
            "Sum of event probabilities must be greater than 0."
        );

        let mut events = events;
        for probability in events.values_mut() {
            *probability /= total;
        }
        DiscreteDistribution {
            distribution: events,
        }
    }

    pub fn probability(&self, event: &Event) -> f64 {
        self.distribution.get(event).copied().unwrap_or(0.0)
    }

    pub fn support(&self) -> impl Iterator<Item = &Event> {
        self.distribution.keys()
    }

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

    pub fn join<AnotherEvent>(
        self,
        conditional_distribution: ConditionalDistribution<Event, AnotherEvent>,
    ) -> DiscreteDistribution<(Event, AnotherEvent)>
    where
        AnotherEvent: Copy + Eq + Hash,
    {
        let joined_events = self
            .support()
            .flat_map(|condition| {
                let probability = self.probability(condition);
                let distribution = conditional_distribution.when(condition)?;
                Some(((condition, probability), distribution))
            })
            .flat_map(|((condition, probability), distribution)| {
                distribution.support().map(move |event| {
                    (
                        (*condition, *event),
                        (probability * distribution.probability(event)),
                    )
                })
            });
        DiscreteDistribution::from_iter(joined_events)
    }

    pub fn marginalize<NewEvent>(
        self,
        convert_event: impl Fn(Event) -> NewEvent,
    ) -> DiscreteDistribution<NewEvent>
    where
        NewEvent: Copy + Eq + Hash,
    {
        let mut distribution = HashMap::new();
        for (old_event, probability) in self.distribution {
            let new_event = convert_event(old_event);
            let entry = distribution.entry(new_event).or_insert(0.0);
            *entry += probability;
        }

        DiscreteDistribution::from_events(distribution)
    }

    pub fn condition(self, condition: impl FnMut(&&Event) -> bool) -> Self {
        DiscreteDistribution::from_iter(
            self.support()
                .filter(condition)
                .map(|event| (*event, self.probability(event))),
        )
    }

    pub fn bayes<AnotherEvent>(
        self,
        conditional_distribution: ConditionalDistribution<Event, AnotherEvent>,
        evidence: &AnotherEvent,
    ) -> DiscreteDistribution<Event>
    where
        AnotherEvent: Copy + Eq + Hash,
    {
        self.join(conditional_distribution)
            .condition(|(_, another_event)| another_event == evidence)
            .marginalize(|(event, _)| event)
    }

    pub fn total_probability<AnotherEvent>(
        self,
        conditional_distribution: ConditionalDistribution<Event, AnotherEvent>,
    ) -> DiscreteDistribution<AnotherEvent>
    where
        AnotherEvent: Copy + Eq + Hash,
    {
        self.join(conditional_distribution)
            .marginalize(|(_, another_event)| another_event)
    }
}

impl<Event, const N: usize> From<[(Event, f64); N]>
    for DiscreteDistribution<Event>
where
    Event: Copy + Eq + Hash,
{
    fn from(events: [(Event, f64); N]) -> Self {
        let events = HashMap::from(events);
        DiscreteDistribution::from_events(events)
    }
}

impl<Event> From<Vec<(Event, f64)>> for DiscreteDistribution<Event>
where
    Event: Copy + Eq + Hash,
{
    fn from(events: Vec<(Event, f64)>) -> Self {
        let events = HashMap::from_iter(events);
        DiscreteDistribution::from_events(events)
    }
}

impl<Event> FromIterator<(Event, f64)> for DiscreteDistribution<Event>
where
    Event: Copy + Eq + Hash,
{
    fn from_iter<Iterator>(events: Iterator) -> Self
    where
        Iterator: IntoIterator<Item = (Event, f64)>,
    {
        let events = HashMap::from_iter(events);
        DiscreteDistribution::from_events(events)
    }
}

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

    pub fn probability(&self, key: &Event) -> f64 {
        self.distribution.get(key).copied().unwrap_or(0.0)
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

    pub fn join<NewEvent>(
        self,
        conditional_distribution: ConditionalDistribution<Event, NewEvent>,
    ) -> DiscreteDistribution<(Event, NewEvent)>
    where
        NewEvent: Copy + Eq + Hash,
    {
        let joined_events = conditional_distribution
            .distribution
            .into_iter()
            .flat_map(|(condition, dependent_distribution)| {
                let probability = self.probability(&condition);
                dependent_distribution.distribution.into_iter().map(
                    move |(event, conditional_probability)| {
                        (
                            (condition, event),
                            (probability * conditional_probability),
                        )
                    },
                )
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
}

pub struct ConditionalDistribution<Condition, Event>
where
    Condition: Eq + Hash,
    Event: Copy + Eq + Hash,
{
    distribution: HashMap<Condition, DiscreteDistribution<Event>>,
}

impl<Condition, Event> ConditionalDistribution<Condition, Event>
where
    Condition: Eq + Hash,
    Event: Copy + Eq + Hash,
{
    fn from_conditional(
        conditional: HashMap<Condition, DiscreteDistribution<Event>>,
    ) -> Self {
        ConditionalDistribution {
            distribution: conditional,
        }
    }

    pub fn when(
        &self,
        condition: &Condition,
    ) -> Option<&DiscreteDistribution<Event>> {
        self.distribution.get(condition)
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

impl<Condition, Event, const N: usize>
    From<[(Condition, DiscreteDistribution<Event>); N]>
    for ConditionalDistribution<Condition, Event>
where
    Condition: Eq + Hash,
    Event: Copy + Eq + Hash,
{
    fn from(
        conditional_events: [(Condition, DiscreteDistribution<Event>); N],
    ) -> Self {
        let conditional_events = HashMap::from(conditional_events);
        ConditionalDistribution::from_conditional(conditional_events)
    }
}

impl<Condition, Event> From<Vec<(Condition, DiscreteDistribution<Event>)>>
    for ConditionalDistribution<Condition, Event>
where
    Condition: Eq + Hash,
    Event: Copy + Eq + Hash,
{
    fn from(
        conditional_events: Vec<(Condition, DiscreteDistribution<Event>)>,
    ) -> Self {
        let conditional_events = HashMap::from_iter(conditional_events);
        ConditionalDistribution::from_conditional(conditional_events)
    }
}

impl<Condition, Event> FromIterator<(Condition, DiscreteDistribution<Event>)>
    for ConditionalDistribution<Condition, Event>
where
    Condition: Eq + Hash,
    Event: Copy + Eq + Hash,
{
    fn from_iter<Iterator>(events: Iterator) -> Self
    where
        Iterator: IntoIterator<Item = (Condition, DiscreteDistribution<Event>)>,
    {
        let events = HashMap::from_iter(events);
        ConditionalDistribution::from_conditional(events)
    }
}

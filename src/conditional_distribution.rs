use super::discrete_distribution::DiscreteDistribution;
use std::collections::HashMap;
use std::hash::Hash;

pub struct ConditionalDistribution<Given, Event>
where
    Given: Eq + Hash,
    Event: Copy + Eq + Hash,
{
    distribution: HashMap<Given, DiscreteDistribution<Event>>,
}

impl<Given, Event> ConditionalDistribution<Given, Event>
where
    Given: Eq + Hash,
    Event: Copy + Eq + Hash,
{
    fn from_conditional(
        conditional: HashMap<Given, DiscreteDistribution<Event>>,
    ) -> Self {
        ConditionalDistribution {
            distribution: conditional,
        }
    }

    pub fn when(
        &self,
        condition: &Given,
    ) -> Option<&DiscreteDistribution<Event>> {
        self.distribution.get(condition)
    }
}

impl<Given, Event, const N: usize>
    From<[(Given, DiscreteDistribution<Event>); N]>
    for ConditionalDistribution<Given, Event>
where
    Given: Eq + Hash,
    Event: Copy + Eq + Hash,
{
    fn from(
        conditional_events: [(Given, DiscreteDistribution<Event>); N],
    ) -> Self {
        let conditional_events = HashMap::from(conditional_events);
        ConditionalDistribution::from_conditional(conditional_events)
    }
}

impl<Given, Event> From<Vec<(Given, DiscreteDistribution<Event>)>>
    for ConditionalDistribution<Given, Event>
where
    Given: Eq + Hash,
    Event: Copy + Eq + Hash,
{
    fn from(
        conditional_events: Vec<(Given, DiscreteDistribution<Event>)>,
    ) -> Self {
        let conditional_events = HashMap::from_iter(conditional_events);
        ConditionalDistribution::from_conditional(conditional_events)
    }
}

impl<Given, Event> FromIterator<(Given, DiscreteDistribution<Event>)>
    for ConditionalDistribution<Given, Event>
where
    Given: Eq + Hash,
    Event: Copy + Eq + Hash,
{
    fn from_iter<Iterator>(events: Iterator) -> Self
    where
        Iterator: IntoIterator<Item = (Given, DiscreteDistribution<Event>)>,
    {
        let events = HashMap::from_iter(events);
        ConditionalDistribution::from_conditional(events)
    }
}

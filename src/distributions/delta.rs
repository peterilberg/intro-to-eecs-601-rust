use crate::distributions::Discrete;
use std::hash::Hash;

pub struct Delta {}

impl Delta {
    pub fn with<Event>(event: Event) -> Discrete<Event>
    where
        Event: Eq + Hash,
    {
        Discrete::from([(event, 1.0); 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_distribution_support() {
        let delta = Delta::with(42);
        let support: Vec<_> = delta.support().collect();
        assert_eq!(support, [&42]);
    }

    #[test]
    fn delta_distribution_probability() {
        assert_eq!(Delta::with(42), Discrete::from([(42, 1.0)]));
    }
}

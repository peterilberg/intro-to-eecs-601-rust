//! A delta discrete probability distribution.

use crate::distributions::Discrete;
use std::hash::Hash;

/// A discrete probability distributions with a single event of probability 1.
pub struct Delta {}

impl Delta {
    /// Create a new delta distribution with a single event.
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

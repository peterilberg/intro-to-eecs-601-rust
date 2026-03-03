//! A uniform probability distributions.

use crate::distributions::Discrete;
use std::hash::Hash;

/// A uniformly distributed probability distribution.
pub struct Uniform {}

impl Uniform {
    /// Create a new probability distribution where all events have the
    /// same probability.
    pub fn with<Event>(events: &[Event]) -> Discrete<Event>
    where
        Event: Clone + Eq + Hash,
    {
        let probability = 1.0 / events.len() as f64;
        Discrete::from_iter(
            events.iter().map(|event| (event.clone(), probability)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_distribution_support() {
        let uniform = Uniform::with(&["a1", "a2", "a3", "a4"]);
        let mut support: Vec<_> = uniform.support().collect();
        support.sort();
        assert_eq!(support, [&"a1", &"a2", &"a3", &"a4"]);
    }

    #[test]
    fn uniform_distribution_probability() {
        assert_eq!(
            Uniform::with(&["a1", "a2", "a3", "a4"]),
            Discrete::from([
                ("a1", 0.25),
                ("a2", 0.25),
                ("a3", 0.25),
                ("a4", 0.25),
            ])
        );
    }
}

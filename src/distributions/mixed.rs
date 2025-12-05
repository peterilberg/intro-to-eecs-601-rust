use crate::distributions::Discrete;

pub struct Mixed {}

impl Mixed {
    pub fn with(mix: f64, a: Discrete<i32>, b: Discrete<i32>) -> Discrete<i32> {
        assert!(
            (0.0..=1.0).contains(&mix),
            "Mix must be between 0.0 and 1.0."
        );

        let mut support: Vec<_> = a.support().chain(b.support()).collect();
        support.sort();
        support.dedup();

        Discrete::from_iter(support.into_iter().map(|event| {
            let probability_a = a.probability(event);
            let probability_b = b.probability(event);
            (*event, mix * probability_a + (1.0 - mix) * probability_b)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributions::{Square, Triangle};

    #[test]
    fn mixed_distribution_support() {
        let mixed = mixed();
        let mut support: Vec<_> = mixed.support().collect();
        support.sort();
        assert_eq!(support, [&0, &1, &2, &3, &4]);
    }

    #[test]
    fn mixed_distribution_probability() {
        assert_eq!(
            mixed(),
            Discrete::from([
                (0, 0.7 * 1.0 / 3.0),
                (1, 0.7 * 1.0 / 3.0),
                (2, 0.7 * 1.0 / 3.0 + 0.3 * 1.0 / 4.0),
                (3, 0.3 * 2.0 / 4.0),
                (4, 0.3 * 1.0 / 4.0),
            ])
        );
    }

    fn mixed() -> Discrete<i32> {
        let triangle = Triangle::with(3, 1);
        let square = Square::with(0, 3);
        Mixed::with(0.3, triangle, square)
    }
}

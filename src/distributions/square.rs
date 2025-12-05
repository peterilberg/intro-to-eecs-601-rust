use crate::distributions::Discrete;

pub struct Square {}

impl Square {
    pub fn with(low: i32, high: i32) -> Discrete<i32> {
        assert!(low < high, "Lower bound must be less than upper bound.");

        let probability = 1.0 / (high - low) as f64;
        Discrete::from_iter((low..high).map(|event| (event, probability)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_distribution_support() {
        let square = Square::with(-1, 2);
        let mut support: Vec<_> = square.support().collect();
        support.sort();
        assert_eq!(support, [&-1, &0, &1]);
    }

    #[test]
    fn square_distribution_probability() {
        assert_eq!(
            Square::with(-1, 2),
            Discrete::from([(-1, 1.0 / 3.0), (0, 1.0 / 3.0), (1, 1.0 / 3.0)])
        );
    }
}

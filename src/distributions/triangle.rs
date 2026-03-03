//! A triangular probability distribution.

use crate::distributions::Discrete;

/// A triangular probability distribution with a peak and two symmetric wings.
pub struct Triangle {}

impl Triangle {
    /// Create a new triangular probability distribution with a central peak
    /// and two symmetric wings.
    pub fn with(peak: i32, half_width: usize) -> Discrete<i32> {
        let mut events = Vec::with_capacity(1 + 2 * half_width);

        let half_width = half_width as i32;
        events.push((peak, (half_width + 1) as f64));

        for i in 1..=half_width {
            events.push((peak - i, (half_width + 1 - i) as f64));
            events.push((peak + i, (half_width + 1 - i) as f64));
        }
        Discrete::from_iter(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_distribution_support_1() {
        let triangle = Triangle::with(0, 2);
        assert_eq!(support(&triangle), [&-2, &-1, &0, &1, &2]);
    }

    #[test]
    fn triangle_distribution_support_2() {
        let triangle = Triangle::with(3, 2);
        assert_eq!(support(&triangle), [&1, &2, &3, &4, &5]);
    }

    #[test]
    fn triangle_distribution_support_3() {
        let triangle = Triangle::with(3, 1);
        assert_eq!(support(&triangle), [&2, &3, &4]);
    }

    #[test]
    fn triangle_distribution_probability_1() {
        assert_eq!(
            Triangle::with(0, 2),
            Discrete::from([
                (-2, 1.0 / 9.0),
                (-1, 2.0 / 9.0),
                (0, 3.0 / 9.0),
                (1, 2.0 / 9.0),
                (2, 1.0 / 9.0),
            ])
        );
    }

    #[test]
    fn triangle_distribution_probability_2() {
        assert_eq!(
            Triangle::with(3, 2),
            Discrete::from([
                (1, 1.0 / 9.0),
                (2, 2.0 / 9.0),
                (3, 3.0 / 9.0),
                (4, 2.0 / 9.0),
                (5, 1.0 / 9.0),
            ])
        );
    }

    #[test]
    fn triangle_distribution_probability_3() {
        assert_eq!(
            Triangle::with(3, 1),
            Discrete::from([(2, 1.0 / 4.0), (3, 2.0 / 4.0), (4, 1.0 / 4.0),])
        );
    }

    fn support(distribution: &Discrete<i32>) -> Vec<&i32> {
        let mut support: Vec<_> = distribution.support().collect();
        support.sort();
        support
    }
}

pub mod histograms;

pub trait MetricSpace {
    fn distance(&self, other: &Self) -> f64;
}

pub enum HistValue {
    Count(usize),
    NormalizedValue(f64),
}

pub enum HistFormat {
    Count,
    DivideByMax,
}

pub trait Histogram<I: Clone, X> {
    fn bucket_index(&self, pt: &X) -> Option<I>;

    fn increment(&mut self, idx: I);
    fn add_sample(&mut self, pt: &X);

    fn set(&mut self, idx: I, count: usize);
    fn get(&self, idx: I, format: HistFormat) -> Option<HistValue>;
}

pub struct Orbit<X> {
    data: Vec<X>,
}

impl<X: Clone + PartialOrd> Orbit<X> {
    pub fn trace<F>(func: F, initial_point: X, iteration_limit: usize) -> Orbit<X>
    where
        F: Fn(X) -> X,
        X: Copy,
    {
        let mut xn = initial_point;
        let mut orbit = Orbit {
            data: vec![initial_point],
        };

        for _ in 0..iteration_limit {
            xn = func(xn);
            orbit.data.push(xn);
        }

        orbit
    }

    pub fn trace_with_early_exit<I: Clone, H, F>(
        hist: &mut H,
        func: F,
        initial_point: X,
        iteration_limit: usize,
        early_exit_eps: f64,
        early_exit_batch: usize,
    ) -> Orbit<X>
    where
        F: Fn(X) -> X,
        X: Copy,
        H: Histogram<I, X> + Clone + MetricSpace,
    {
        let mut xn = initial_point;
        let mut orbit = Orbit {
            data: vec![],
        };
        let mut prev_hist = hist.clone();

        for i in 0..iteration_limit {
            orbit.data.push(xn);
            hist.add_sample(&xn);
            if i % early_exit_batch == 0 {
                if hist.distance(&prev_hist) < early_exit_eps {
                    return orbit;
                }
            }
            prev_hist.add_sample(&initial_point);

            xn = func(xn);
        }

        orbit
    }

    pub fn range(&self) -> (X, X) {
        let mut lower_bound = &self.data[0];
        let mut upper_bound = &self.data[0];

        for entry in &self.data[1..] {
            if entry < lower_bound {
                lower_bound = entry;
            }

            if entry > upper_bound {
                upper_bound = entry;
            }
        }

        (lower_bound.clone(), upper_bound.clone())
    }

    pub fn update_histogram<I: Clone>(&self, hist: &mut dyn Histogram<I, X>) {
        for xn in &self.data {
            hist.add_sample(&xn);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl MetricSpace for i32 {
        fn distance(&self, other: &Self) -> f64 {
            i32::abs(self - other).into()
        }
    }

    #[test]
    fn test_trace_linear_function() {
        let result = Orbit::trace(|x: i32| x + 1, 0, 5);
        let expected_orbit = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(result.data, expected_orbit);
    }

    #[test]
    fn test_trace_quadratic_function() {
        let result = Orbit::trace(|x: i32| x * x, 2, 3);
        let expected_orbit = vec![2, 4, 16, 256];
        assert_eq!(result.data, expected_orbit);
    }

    #[test]
    fn test_trace_no_iterations() {
        let result = Orbit::trace(|x: i32| x, 42, 0);
        let expected_orbit = vec![42];
        assert_eq!(result.data, expected_orbit);
    }

    #[test]
    fn test_range() {
        let result = Orbit::trace(|x: i32| -x * x * x, 2, 3);
        assert_eq!(result.range(), (-134217728, 512));
    }

    #[test]
    fn test_range_no_iterations() {
        let result = Orbit::trace(|x: i32| x * x, 2, 0);
        assert_eq!(result.range(), (2, 2));
    }
}

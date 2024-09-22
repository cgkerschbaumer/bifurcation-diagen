use std::{usize};

use anyhow::Result;

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

    fn set(&mut self, idx: I, count: usize);
    fn get(&self, idx: I, format: HistFormat) -> Option<HistValue>;
}

pub struct HistogramR1 {
    lower_bound: f64,
    upper_bound: f64,

    buckets: usize,
    counts: Vec<usize>,
}

impl Histogram<usize, f64> for HistogramR1 {
    fn bucket_index(&self, pt: &f64) -> Option<usize> {
        if *pt < self.lower_bound {
            None
        } else if *pt > self.upper_bound {
            None
        } else {
            None
        }
    }

    fn increment(&mut self, idx: usize) {
        if idx < self.counts.len() {
            self.counts[idx] += 1;
        }
    }

    fn set(&mut self, idx: usize, count: usize) {
        if idx < self.counts.len() {
            self.counts[idx] = count;
        }
    }

    fn get(&self, idx: usize, format: HistFormat) -> Option<HistValue> {
        if idx >= self.counts.len() {
            return None;
        }

        match format {
            HistFormat::Count => Some(HistValue::Count(self.counts[idx])),
            HistFormat::DivideByMax => {
                let max_val = *self.counts.iter().max().unwrap() as f64;
                let idx_val = self.counts[idx] as f64;

                Some(HistValue::NormalizedValue(idx_val / max_val))
            }
        }
    }
}

struct Orbit<X> {
    data: Vec<X>,
}

impl<X: Clone + PartialOrd> Orbit<X> {
    fn trace<F>(func: F, initial_point: X, iteration_limit: usize) -> Result<Orbit<X>>
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

        Ok(orbit)
    }

    fn range(&self) -> (X, X) {
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

    fn update_histogram<I: Clone>(&self, hist: &mut dyn Histogram<I, X>) {
        for xn in &self.data {
            if let Some(idx) = hist.bucket_index(&xn) {
                hist.increment(idx);
            }
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
        let result = Orbit::trace(|x: i32| x + 1, 0, 5).unwrap();
        let expected_orbit = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(result.data, expected_orbit);
    }

    #[test]
    fn test_trace_quadratic_function() {
        let result = Orbit::trace(|x: i32| x * x, 2, 3).unwrap();
        let expected_orbit = vec![2, 4, 16, 256];
        assert_eq!(result.data, expected_orbit);
    }

    #[test]
    fn test_trace_no_iterations() {
        let result = Orbit::trace(|x: i32| x, 42, 0).unwrap();
        let expected_orbit = vec![42];
        assert_eq!(result.data, expected_orbit);
    }

    #[test]
    fn test_range() {
        let result = Orbit::trace(|x: i32| -x * x * x, 2, 3).unwrap();
        assert_eq!(result.range(), (-134217728, 512));
    }

    #[test]
    fn test_range_no_iterations() {
        let result = Orbit::trace(|x: i32| x * x, 2, 0).unwrap();
        assert_eq!(result.range(), (2, 2));
    }
}

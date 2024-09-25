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

#[derive(Clone)]
pub struct HistogramR1 {
    lower_bound: f64,
    upper_bound: f64,

    sub_divisions: usize,
    counts: Vec<usize>,

    max_value: usize,
}

impl HistogramR1 {
    pub fn new(interval: (f64, f64), sub_divisions: usize) -> Self {
        assert!(interval.0 < interval.1);

        HistogramR1 {
            lower_bound: interval.0,
            upper_bound: interval.1,
            sub_divisions,
            counts: vec![0; sub_divisions],
            max_value: 0,
        }
    }

    fn get_normalized(&self, idx: usize) -> f64 {
        (self.counts[idx] as f64) / (self.max_value as f64)
    }
}

impl MetricSpace for HistogramR1 {
    fn distance(&self, other: &Self) -> f64 {
        let mut sup_norm = 0f64;

        assert!(
            self.counts.len() == other.counts.len(),
            "R1 Histogram sub-divisions are incompatible."
        );

        for i in 0..self.counts.len() {
            let delta = (self.get_normalized(i) - other.get_normalized(i)).abs();

            if delta > sup_norm {
                sup_norm = delta;
            }
        }

        sup_norm
    }
}

impl Histogram<usize, f64> for HistogramR1 {
    fn bucket_index(&self, pt: &f64) -> Option<usize> {
        if *pt < self.lower_bound || *pt > self.upper_bound {
            None
        } else {
            let bucket_width = (self.upper_bound - self.lower_bound) / (self.sub_divisions as f64);
            Some(f64::floor((*pt - self.lower_bound) / bucket_width) as usize)
        }
    }

    fn increment(&mut self, idx: usize) {
        if idx < self.counts.len() {
            if self.counts[idx] == self.max_value {
                self.max_value += 1;
            }

            self.counts[idx] += 1;
        }
    }

    fn add_sample(&mut self, pt: &f64) {
        if let Some(idx) = self.bucket_index(&pt) {
            self.increment(idx);
        }
    }

    fn set(&mut self, idx: usize, count: usize) {
        if idx < self.counts.len() {
            if count > self.max_value {
                self.max_value = count;
            }

            self.counts[idx] = count;
        }
    }

    fn get(&self, idx: usize, format: HistFormat) -> Option<HistValue> {
        if idx >= self.counts.len() {
            return None;
        }

        match format {
            HistFormat::Count => Some(HistValue::Count(self.counts[idx])),
            HistFormat::DivideByMax => Some(HistValue::NormalizedValue(self.get_normalized(idx))),
        }
    }
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

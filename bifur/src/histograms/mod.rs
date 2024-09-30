use crate::*;

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

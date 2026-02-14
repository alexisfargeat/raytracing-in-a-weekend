#[derive(Copy, Clone)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::MIN,
            max: f64::MAX,
        }
    }
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Interval {
        assert!(
            min <= max,
            "min element should be less than the max element of the interval"
        );
        Interval { min, max }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn length(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        x > self.min && x < self.max
    }
}

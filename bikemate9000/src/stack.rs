use crate::heart_rate::HeartRateReading;

#[derive(Debug)]
pub struct MinMaxStack {
    values: Vec<HeartRateReading>,
    min_values: Vec<HeartRateReading>,
    max_values: Vec<HeartRateReading>
}

// Uses three stacks to achieve O(1) time for push, pop, get_min, and get_max.
impl MinMaxStack {
    pub fn new() -> Self {
        Self { values: Vec::new(), min_values: Vec::new(), max_values: Vec::new() }
    }

    pub fn push(&mut self, v: HeartRateReading) {
        match self.get_min() {
            Some(val) => if v.hr <= val.hr { self.min_values.push(v); }
            None => self.min_values.push(v)
        }

        match self.get_max() {
            Some(val) => if v.hr >= val.hr { self.max_values.push(v); }
            None => self.max_values.push(v)
        }

        self.values.push(v);
    }

    pub fn pop(&mut self) -> Option<HeartRateReading> {
        let v = self.values.pop();

        let min_v = self.get_min();
        if let (Some(popped), Some(min)) = (v, min_v) {
            if popped.hr == min.hr {
                self.min_values.pop();
            }
        }

        let max_v = self.get_max();
        if let (Some(popped), Some(max)) = (v, max_v) {
            if popped.hr == max.hr {
                self.max_values.pop();
            }
        }

        v
    }

    pub fn top(&self) -> Option<&HeartRateReading> {
        self.values.last()
    }

    pub fn get_min(&self) -> Option<HeartRateReading> {
        self.min_values.last().copied()
    }

    pub fn get_max(&self) -> Option<HeartRateReading> {
        self.max_values.last().copied()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

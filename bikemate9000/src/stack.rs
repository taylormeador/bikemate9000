use crate::heart_rate::{HeartRateReading, RichHeartRateReading};



#[derive(Debug)]
pub struct MinStack {
    values: Vec<RichHeartRateReading>,
    min_values: Vec<RichHeartRateReading>
}

impl MinStack {
    pub fn new() -> Self {
        Self { values: Vec::new(), min_values: Vec::new() }
    }

    pub fn push(&mut self, v: RichHeartRateReading) {
        match self.get_min() {
            Some(val) => if v.hr_reading <= val.hr_reading { self.min_values.push(v); }
            None => self.min_values.push(v)
        }
        self.values.push(v);
    }

    pub fn pop(&mut self) -> Option<RichHeartRateReading> {
        let v = self.values.pop();
        let min_v = self.get_min();
        if let (Some(popped), Some(min)) = (v, min_v) {
            if popped.hr_reading == min.hr_reading {
                self.min_values.pop();
            }
        }
        v
    }

    pub fn get_min(&self) -> Option<RichHeartRateReading> {
        self.min_values.last().copied()
    }
} 

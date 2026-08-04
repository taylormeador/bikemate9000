use tracing::info;

use crate::stack;
use crate::heart_rate::{HeartRateReading};

pub struct SlidingWindow {
    duration: u128,  // time in millis
    count: u64,
    sum: u64,
    back_stack: stack::MinMaxStack,
    front_stack: stack::MinMaxStack
}

impl SlidingWindow {
    pub fn new(duration: u128) -> Self {
        let count = 0;
        let sum = 0;
        let back_stack = stack::MinMaxStack::new();
        let front_stack = stack::MinMaxStack::new();
        SlidingWindow{
            duration,
            count,
            sum,
            back_stack,
            front_stack
        }
    }

    // Transfer the back stack to the front stack so that we can pop the oldest items in O(1).
    // The transfer is linear but amortizes to O(1).
    fn transfer_to_front(&mut self) {
        loop {
            if let Some(item) = self.back_stack.pop() {
                self.front_stack.push(item);
            } else {
                break;
            }
        } 
    }

    pub fn handle_reading(&mut self, hr_reading: HeartRateReading) {
        self.back_stack.push(hr_reading);
        self.count += 1;
        self.sum += hr_reading.hr_reading as u64;
        let cutoff = hr_reading.ts - self.duration; // TODO what if this is negative somehow
        
        if self.front_stack.is_empty() {
            self.transfer_to_front();
        }

        while let Some(item) = self.front_stack.top() {
            if item.ts < cutoff {
                info!("evicting: {:?}", item);
                self.sum -= item.hr_reading as u64;
                self.front_stack.pop();
                self.count -= 1;
            } else {
                break;
            }
        }
    }

    pub fn get_min(&self) -> HeartRateReading {
        let front_min = self.front_stack.get_min();
        let back_min = self.back_stack.get_min();
        match (front_min, back_min) {
            (Some(a), Some(b)) => {
                if a.hr_reading < b.hr_reading {
                    a
                } else {
                    b
                }
            },
            (Some(a), None) => {
                a
            },
            (None, Some(b)) => {
                b
            },
            (None, None) => {
                HeartRateReading{ ts: 0, hr_reading: 0 }
            }
        }
    }

    pub fn get_max(&self) -> HeartRateReading {
        let front_max = self.front_stack.get_max();
        let back_max = self.back_stack.get_max();
        match (front_max, back_max) {
            (Some(a), Some(b)) => {
                if a.hr_reading > b.hr_reading {
                    a
                } else {
                    b
                }
            },
            (Some(a), None) => {
                a
            },
            (None, Some(b)) => {
                b
            },
            (None, None) => {
                HeartRateReading{ ts: 0, hr_reading: 0 }
            }
        }
    }

    pub fn get_average(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum as f64 / self.count as f64
        }
        
    }

    pub fn get_pct_change(&self) -> f64 {
        if let (Some(newest), Some(oldest)) = (self.back_stack.top(), self.front_stack.top()) {
            newest.hr_reading as f64 / oldest.hr_reading as f64 - 1.0
        } else {
            0.0
        }
    }
}

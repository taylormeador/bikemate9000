use tracing::info;

use crate::stack;
use crate::heart_rate::{HeartRateReading};

pub struct SlidingWindow {
    duration: u128,  // time in millis
    back_stack: stack::MinMaxStack,
    front_stack: stack::MinMaxStack
}

impl SlidingWindow {
    pub fn new(duration: u128) -> Self {
        let back_stack = stack::MinMaxStack::new();
        let front_stack = stack::MinMaxStack::new();
        SlidingWindow{
            duration,
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
        let cutoff = hr_reading.ts - self.duration; // TODO what if this is negative somehow
        
        if self.front_stack.is_empty() {
            self.transfer_to_front();
        }

        while let Some(item) = self.front_stack.top() {
            if item.ts < cutoff {
                info!("evicting: {:?}", item);
                self.front_stack.pop();
            } else {
                break;
            }
        }
    }

    pub fn get_min(&self) -> Option<HeartRateReading> {
        let front_min = self.front_stack.get_min();
        let back_min = self.back_stack.get_min();
        match (front_min, back_min) {
            (Some(a), Some(b)) => {
                if a.hr_reading < b.hr_reading {
                    front_min
                } else {
                    back_min
                }
            },
            (Some(_a), None) => {
                front_min
            },
            (None, Some(_b)) => {
                back_min
            },
            (None, None) => {
                None
            }
        }
    }

    pub fn get_max(&self) -> Option<HeartRateReading> {
        let front_max = self.front_stack.get_max();
        let back_max = self.back_stack.get_max();
        match (front_max, back_max) {
            (Some(a), Some(b)) => {
                if a.hr_reading > b.hr_reading {
                    front_max
                } else {
                    back_max
                }
            },
            (Some(_a), None) => {
                front_max
            },
            (None, Some(_b)) => {
                back_max
            },
            (None, None) => {
                None
            }
        }
    }
}

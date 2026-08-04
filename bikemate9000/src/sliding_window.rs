use tracing::info;

use crate::stack;
use crate::heart_rate;

pub struct SlidingWindow {
    duration: u128,  // time in millis
    back_stack: stack::MinStack,
    front_stack: stack::MinStack
}

impl SlidingWindow {
    pub fn new(duration: u128) -> Self {
        let back_stack = stack::MinStack::new();
        let front_stack = stack::MinStack::new();
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

    pub fn handle_reading(&mut self, hr_reading: heart_rate::HeartRateReading) {
        self.back_stack.push(hr_reading);
        let cutoff = hr_reading.ts - self.duration; // TODO what if this is negative somehow
        
        if self.front_stack.is_empty() {
            self.transfer_to_front();
        }

        while let Some(item) = self.front_stack.peek() {
            if item.ts < cutoff {
                info!("evicting: {:?}", item);
                self.front_stack.pop();
            } else {
                break;
            }
        }
    }
}

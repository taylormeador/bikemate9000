struct MinStack {
    values: Vec<u64>,
    min_values: Vec<u64>
}

impl MinStack {
    fn new() -> Self {
        Self { values: Vec::new(), min_values: Vec::new() }
    }

    fn push(&mut self, v: u64) {
        match self.get_min() {
            Some(val) => if v <= val { self.min_values.push(v); }
            None => self.min_values.push(v)
        }
        self.values.push(v);
    }

    fn pop(&mut self) -> Option<u64> {
        let v = self.values.pop();
        let min_v = self.get_min();
        if let (Some(popped), Some(min)) = (v, min_v) {
            if popped == min {
                self.min_values.pop();
            }
        }
        v
    }

    fn get_min(&self) -> Option<u64> {
        self.min_values.last().copied()
    }
} 

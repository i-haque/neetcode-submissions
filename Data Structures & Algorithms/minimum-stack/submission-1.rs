struct MinStack {
    stack: Vec<(i32, i32)>
}

impl MinStack {
    pub fn new() -> Self {
        let mut stack = Vec::new();
        Self { stack }
    }

    pub fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push((val, val));
        } else {
            let min_val = min(val, self.stack.last().unwrap().0);
            self.stack.push((min_val, val));
        }
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn top(&self) -> i32 {
        self.stack.last().unwrap().1
    }

    pub fn get_min(&self) -> i32 {
        self.stack.last().unwrap().0
    }
}

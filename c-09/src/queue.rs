pub struct Queue<T> {
    pub older: Vec<T>,
    pub younger: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    pub fn push(&mut self, c: T) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

impl Queue<f64> {
    fn sum(&self) -> f64 {
        // ...
        return 0.0;
    }
}

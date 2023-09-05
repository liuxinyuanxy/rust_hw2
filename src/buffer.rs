pub struct Buffer<T> {
    data: Vec<T>,
}

impl<T> Buffer<T> {
    pub fn new() -> Self {
        Buffer { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

impl<T: std::ops::Add<Output = T> + Copy> Buffer<T> {
    pub fn sum(&self) -> Option<T> {
        let mut sum = None;
        for value in &self.data {
            sum = match sum {
                None => Some(*value),
                Some(sum) => Some(sum + *value),
            };
        }
        sum
    }
}

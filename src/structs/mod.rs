pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.stack.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn get(&self) -> &Vec<T> {
        &self.stack
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    pub fn top(&self) -> Option<&T> {
        self.stack.last()
    }
}

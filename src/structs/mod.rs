pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            stack: Vec::new(),
        }
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
}
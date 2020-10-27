#[derive(Clone, Debug)]
struct StackNode<T> {
    value: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T> StackNode<T> {
    fn new(val: T) -> Self {
        StackNode {
            value: val,
            next: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
    size: u32,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack { top: None, size: 0 }
    }

    pub fn push(&mut self, val: T) {
        let mut new_stack_node = StackNode::new(val);
        new_stack_node.next = self.top.take();
        self.top = Some(Box::new(new_stack_node));
        self.size += 1;
    }

    pub fn top(&mut self) -> Option<T> {
        match self.top.clone() {
            None => None,
            Some(x) => Some(x.value),
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.top.take() {
            None => None,
            Some(x) => {
                self.top = x.next;
                self.size -= 1;
                Some(x.value)
            }
        }
    }

    pub fn reverse(&mut self) -> Self {
        let mut stack = Stack::new();
        while let Some(x) = self.pop() {
            stack.push(x)
        }
        stack
    }

    pub fn clear(&mut self) {
        while self.size > 0 {
            self.pop();
        }
    }
}

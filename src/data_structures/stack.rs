

#[derive(Debug)]
struct StackNode<T> {
    value: T,
    next: Node<T>,
}

pub struct Stack<T> {
    head: Node<T>,
}

type Node<T> = Option<Box<StackNode<T>>>;

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {head: None}
    }

    pub fn push(&mut self, val: T){
   
        let new_node = Box::new(StackNode{value: val, next: self.head.take()});
        self.head = Some(new_node);
    }
}
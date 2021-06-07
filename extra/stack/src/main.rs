#[derive(Debug, Clone)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32, next: Option<Box<Node>>) -> Node {
        Node { value, next }
    }
}

#[derive(Debug)]
struct Stack {
    top: Option<Box<Node>>,
    size: u32,
}

impl Stack {
    fn new() -> Stack {
        Stack { top: None, size: 0 }
    }

    fn top(&self) -> Option<&i32> {
        match &self.top {
            Some(node) => Some(&node.value),
            None => None
        }
    }

    fn push(&mut self, value: i32) {
        self.top = Some(Box::new(Node::new(value, self.top.clone())));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<i32> {
        match &self.top {
            Some(node) => {
                let old_top = *node.clone();
                self.top = old_top.next;
                self.size -= 1;
                Some(old_top.value)
            },
            None => None
        }
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    println!("{:?}", stack);
    println!("{:?}", stack.top());
    println!("{:?}", stack.pop());
    println!("{:?}", stack);
    println!("{:?}", stack.pop());
    println!("{:?}", stack.top());
}

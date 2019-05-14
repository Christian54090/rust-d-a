pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List { head: None }
    }

    fn push(&mut self, elem: T) {
        // create new Box<Node>, set next to old head
        // set head to new node
    }

    fn pop(&mut self) -> Option<T> {
        // set head to old head's next
        // return old head
    }
}
pub struct Tree<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Tree<T> {
    pub fn new(head: T) -> Self {
        Tree { head }
    }
}
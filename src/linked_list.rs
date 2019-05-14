pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        // create new Box<Node>, set next to old head
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        // set head to new node
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // set head to old head's next
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
        // return old head
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn push_and_pop() {
        let mut list = List::new();

        list.push(1); list.push(2); list.push(3);

        // Check removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4); list.push(5);

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
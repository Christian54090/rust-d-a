pub struct Tree {
    head: Box<Node>,
}

type Link = Option<Box<Node>>;

struct Node {
    data: i32,
    left: Link,
    right: Link,
}

impl Tree {
    pub fn new(head: i32) -> Self {
        Tree {
            head: Box::new(Node { data: head, left: None, right: None }),
        }
    }

    pub fn insert(self, data: i32) -> Tree {
        let new_node = Box::new(Node {
            data,
            left: None,
            right: None,
        });

        if data > self.head.data {
            let mut node = self.head;
            while node.right.is_some() {
                node = node.right.unwrap();
            }

            node.right = Some(new_node);
        } else {
            let mut node = self.head;
            while node.left.is_some() {
                node = node.left.unwrap();
            }

            node.left = Some(new_node);
        }
        Tree { head: self.head }
    }
}

#[cfg(test)]
mod test {
    use super::Tree;

    #[test]
    fn basics() {
        let mut tree = Tree::new(4);
        let head = tree.head;

        // inserting left
        tree = tree.insert(2);
        assert_eq!(head.data, 4);
        assert_eq!(head.left.unwrap().data, 2);

        // inserting right
        tree = tree.insert(5);
        assert_eq!(head.data, 4);
        assert_eq!(head.right.unwrap().data, 5);
    }
}
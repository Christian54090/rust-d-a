pub struct Tree {
    head: Link
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
            head: Some(Box::new(Node { data: head, left: None, right: None })),
        }
    }

    pub fn insert(mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            left: None,
            right: None,
        });

        let head = self.head.cloned();
        if data > head.unwrap().data {
            let mut r_node = self.head.unwrap();
            while r_node.right.is_some() {
                r_node = r_node.right.unwrap();
            }

            r_node.right = Some(new_node);
        } else {
            let mut l_node = self.head.unwrap();
            while l_node.left.is_some() {
                l_node = l_node.left.unwrap();
            }

            l_node.left = Some(new_node);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut tree = Tree::new(4);

        // inserting left
        tree.insert(2);
        assert_eq!(tree.head.data, Some(4));
        assert_eq!(tree.head.left.data, Some(2));

        // inserting right
        tree.insert(5);
        assert_eq!(tree.head.data, Some(4));
        assert_eq!(tree.head.right.data, Some(5));
    }
}
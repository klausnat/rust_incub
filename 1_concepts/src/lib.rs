use std::sync::{Arc, Mutex, Weak};

#[derive(Debug, Clone)]
pub struct Node<T: Clone + Copy> {
    r_previous: Option<Weak<Mutex<Node<T>>>>,
    r_next: Option<Arc<Mutex<Node<T>>>>,
    data: T,
}

pub struct List<T: Clone + Copy> {
    first: Arc<Mutex<Node<T>>>,
    last: Arc<Mutex<Node<T>>>,
}

impl<T: Clone + Copy> List<T> {
    // constructor
    pub fn new(data1: T, data2: T) -> Self {
        // create 2 nodes
        let first = Arc::new(Mutex::new(Node {
            r_previous: None,
            r_next: None,
            data: data1,
        }));
        let last = Arc::new(Mutex::new(Node {
            r_previous: None,
            r_next: None,
            data: data2,
        }));

        // establish circular referencing
        {
            let mut fst = first.lock().unwrap();
            fst.r_next = Some(Arc::clone(&last));

            let mut snd = last.lock().unwrap();
            snd.r_previous = Some(Arc::downgrade(&first));
        }

        List { first, last }
    }

    pub fn len(&self) -> i32 {
        let mut len = 0;
        let mut res = Arc::clone(&self.first);
        loop {
            let next = {
                let mut node = res.lock().unwrap();
                len += 1;
                node.r_next.clone()
            };

            if let Some(next) = next {
                res = next
            } else {
                break;
            }
        }
        len
    }

    // Traverse (apply function to each element)
    // allows mutation during traversal
    pub fn traverse<F>(&self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        let mut current = Arc::clone(&self.first);
        loop {
            let next = {
                let mut node = current.lock().unwrap();
                f(&mut node.data);
                node.r_next.clone()
            };

            if let Some(next) = next {
                current = next
            } else {
                break;
            }
        }
    }

    // insert new node after specific node
    pub fn insert_after(&self, existing_node: &Arc<Mutex<Node<T>>>, new_data: T) {
        // create new node
        let new_node = Arc::new(Mutex::new(Node {
            r_previous: Some(Arc::downgrade(existing_node)),
            r_next: None,
            data: new_data,
        }));

        let mut existing = existing_node.lock().unwrap();

        // store the old next node
        let old_next = existing.r_next.take();

        // link the new node to the old_next node if it exists
        if let Some(ref next_node) = old_next {
            let mut next = next_node.lock().unwrap();
            next.r_previous = Some(Arc::downgrade(&new_node));
        }

        existing.r_next = Some(Arc::clone(&new_node));

        new_node.lock().unwrap().r_next = old_next;
    }

    // remove node
    // Remove a node identified by its Arc<Mutex<Node<T>>>
    pub fn remove_node(&self, node_to_remove: &Arc<Mutex<Node<T>>>) -> Result<(), &'static str> {
        // Get the node's previous and next pointers
        let (prev, next) = {
            let node = node_to_remove.lock().unwrap();
            (node.r_previous.clone(), node.r_next.clone())
        };

        // Update previous node's next pointer if it exists
        if let Some(prev_weak) = prev {
            if let Some(prev_node) = prev_weak.upgrade() {
                let mut prev = prev_node.lock().unwrap();
                prev.r_next = next.clone();
            }
        }
        // Update next node's previous pointer if it exists
        else if let Some(next_node) = next {
            let mut next = next_node.lock().unwrap();
            next.r_previous = prev;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_list() {
        let list = List::new(2, 3);
        assert_eq!(
            (
                list.first.lock().unwrap().data,
                list.last.lock().unwrap().data
            ),
            (2, 3)
        );
    }

    #[test]
    fn traverse_mut_list() {
        let list = List::new(2, 3);
        list.traverse(|x| *x += 1);
        assert_eq!(
            (
                list.first.lock().unwrap().data,
                list.last.lock().unwrap().data
            ),
            (3, 4)
        );
    }

    #[test]
    fn length_and_insert_after() {
        let mut list = List::new(2, 3);

        assert_eq!(list.len(), 2);

        let node = Arc::clone(&list.first);

        list.insert_after(&node, 5);

        assert_eq!(list.len(), 3);

        // Verify the order
        let mut items = Vec::new();
        list.traverse(|x| items.push(*x));
        assert_eq!(items, vec![2, 5, 3]);
    }

    #[test]
    fn test_remove_node() {
        let list = List::new(2, 3);

        // Insert a node to make list: 2 -> 5 -> 3
        let first_node = Arc::clone(&list.first);
        list.insert_after(&first_node, 5);
        assert_eq!(list.len(), 3);

        // Get the middle node (5)
        let middle_node = {
            let first = first_node.lock().unwrap();
            first.r_next.as_ref().unwrap().clone()
        };

        // Remove the middle node
        list.remove_node(&middle_node).unwrap();
        assert_eq!(list.len(), 2);

        // Verify list is now 2 -> 3
        let mut items = Vec::new();
        list.traverse(|x| items.push(*x));
        assert_eq!(items, vec![2, 3]);
    }
}

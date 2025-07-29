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
    pub fn insert_after(&mut self, mut node: Node<T>, mut new_node: Node<T>) {
        node.r_next = Some(Arc::new(Mutex::new(new_node.clone())));
        let save_prev = new_node.r_previous.clone();
        new_node.r_previous = Some(Arc::downgrade(&Arc::new(Mutex::new(node))));
        match save_prev {
            None => new_node.r_next = None,
            _ => new_node.r_next = Some(save_prev.unwrap().upgrade().unwrap()),
        }
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
    fn length() {
        let list = List::new(2, 3);

        assert_eq!(list.len(), 2);

        let new_node = Node {
            r_previous: None,
            r_next: None,
            data: 5,
        };

        list.insert_after(self.first, new_node);
        assert_eq!(list.len(), 3);
    }
}

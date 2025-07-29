use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
pub struct Node<T> {
    r_previous: Option<Weak<Mutex<Node<T>>>>,
    r_next: Option<Arc<Mutex<Node<T>>>>,
    data: T,
}

pub struct List<T> {
    first: Arc<Mutex<Node<T>>>,
    last: Arc<Mutex<Node<T>>>,
}

impl<T> List<T> {
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

    // traverse (apply function to each element)
    // Simple traverse method that applies a function to each element
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
}

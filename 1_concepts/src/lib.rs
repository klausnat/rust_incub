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
    // in order to create new list we require data for 2 nodes
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_node() {
        let list = List::new(2, 3);
        
    }
}

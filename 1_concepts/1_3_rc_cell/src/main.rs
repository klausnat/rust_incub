// Write a GlobalStack<T> collection which represents a trivial unsized stack
// (may grow infinitely) and has the following semantics:

// 1. can be mutated through multiple shared references (&GlobalStack<T>);
// 2. cloning doesn't clone data, but only produces a pointer, so multiple owners mutate the same data.

use std::cell::RefCell;
// Using Rc in this example, but for multy-thread safety usage I could use Arc instead
use std::rc::Rc;

#[derive(Debug)]
struct GlobalStack<T>(Rc<RefCell<Vec<T>>>);

impl<T> GlobalStack<T> {
    fn new() -> Self {
        GlobalStack(Rc::new(RefCell::new(vec![])))
    }

    fn push(&self, elem: T) {
        self.0.borrow_mut().push(elem);
    }

    fn pop(&self) -> Option<T> {
        self.0.borrow_mut().pop()
    }

    fn len(&self) -> usize {
        self.0.borrow().len()
    }

    fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }
}

impl<T> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        GlobalStack(Rc::clone(&self.0))
    }
}

fn main() {
    // some tests
    let st1 = GlobalStack::new();
    st1.push(5);
    st1.push(6);
    st1.push(1);

    println!("my stack {:?}, and is it empty? {}", st1, st1.is_empty());
    st1.pop();

    println!(
        "my stack after pop {:?}, and it's lenght {}",
        st1,
        st1.len()
    );

    let st2 = st1.clone();
    println!("my cloned stack {:?}", st2);
    println!("and my old stack {:?}", st1);
}

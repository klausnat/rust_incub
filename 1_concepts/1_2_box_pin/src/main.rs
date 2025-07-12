use std::{default, f32::INFINITY, fmt, pin::Pin, process::exit, rc::Rc};

// TASK: for the following types: Box<T>, Rc<T>, Vec<T>, String, &[u8], T.
// Implement the following traits:
trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        // Implementation must be meaningful, and
        // obviously call something requiring `&mut self`.
        // The point here is to practice dealing with
        // `Pin<&mut Self>` -> `&mut self` conversion
        // in different contexts, without introducing
        // any `Unpin` trait bounds.
    }
}

impl<T: fmt::Debug> SayHi for Box<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Redefined say_hi from Boxed_T: {:?}", self);
    }
}

impl<T: fmt::Debug> SayHi for Rc<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Redefined say_hi from Rc_T: {:?}", self);
    }
}

impl<T: fmt::Debug> SayHi for Vec<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Redefined say_hi from Vec_T: {:?}", self);
    }
}

impl SayHi for String {
    fn say_hi(self: Pin<&Self>) {
        println!("Redefined say_hi from String");
    }
}

impl SayHi for &[u8] {
    // use default implementation
}

// we do not need to implement SayHi for T to avoid Rust coherence rules violation

impl<T> MutMeSomehow for Box<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let inner = unsafe { self.get_unchecked_mut() };
        *inner = Box::new(unsafe { std::ptr::read(&**inner) });
    }
}

// increment strong count (requires mutable access to the reference counter)
impl<T> MutMeSomehow for Rc<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let inner = unsafe { self.get_unchecked_mut() };
        let _clone = inner.clone();
    }
}

impl<T> MutMeSomehow for Vec<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let inner = unsafe { self.get_unchecked_mut() };
        inner.reverse();
    }
}

impl MutMeSomehow for String {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let inner = unsafe { self.get_unchecked_mut() };
        inner.insert(0, 'f');
    }
}

// For &[u8] - modify through the mutable reference. But this is not safe
impl MutMeSomehow for &[u8] {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let inner = unsafe { self.get_unchecked_mut() };
        let _ = inner.iter().filter(|x| **x == 0);
    }
}

//Commented because of conflict with implementation for type Box<> abowe
// impl<T> MutMeSomehow for T
// where
//     T: Default,
// {
//     fn mut_me_somehow(self: Pin<&mut Self>) {
//         let inner = unsafe { self.get_unchecked_mut() };
//         *inner = T::default();
//     }
// }

fn main() {
    let mut s = String::from("older");
    let pinned = Pin::new(&mut s);

    // calls my implementation that ass char 'f' to the string
    pinned.mut_me_somehow();
    println!("{}", s); // Output: "folder"
}

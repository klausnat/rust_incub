use std::{fmt, future::Future, pin::Pin, rc::Rc, task::Poll, time::Instant};

// TASK 1 : for the following types: Box<T>, Rc<T>, Vec<T>, String, &[u8], T.
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

// TASK 2. For the following structure
struct MeasurableFuture<Fut> {
    inner_future: Fut,
    started_at: Option<Instant>,
}

// constructor
impl<Fut> MeasurableFuture<Fut> {
    fn new(inner_future: Fut) -> Self {
        Self {
            inner_future,
            started_at: None,
        }
    }
}

// Provide a Future trait implementation, transparently polling the inner_future,
// and printing its execution time in nanoseconds once it's ready.
// Using Fut: Unpin trait bound (or similar) is not allowed.

impl<Fut: Future> Future for MeasurableFuture<Fut> {
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        unsafe {
            let u_fut = self.get_unchecked_mut();

            // start calculating time from this point
            if u_fut.started_at.is_none() {
                u_fut.started_at = Some(Instant::now());
            }

            // create pin for inner_future
            let inner_pin = Pin::new_unchecked(&mut u_fut.inner_future);

            //call poll for inner future
            match inner_pin.poll(cx) {
                Poll::Ready(res) => {
                    let took_time = u_fut.started_at.unwrap().elapsed();
                    println!("took time: {} nanoseconds", took_time.as_nanos());
                    Poll::Ready(res)
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }
}

fn main() {
    // example: call mut_me_somehow() for the String
    let mut s = String::from("older");
    let pinned = Pin::new(&mut s);

    // calls my implementation that ass char 'f' to the string
    pinned.mut_me_somehow();
    println!("{}", s); // Output: "folder"
}

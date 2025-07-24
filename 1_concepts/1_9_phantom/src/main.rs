// Implement a Fact<T> type which returns some random fact about T type that Fact<T> is implemented for.

// let f: Fact<Vec<T>> = Fact::new();
// println!("Fact about Vec: {}", f.fact());
// println!("Fact about Vec: {}", f.fact());

// Fact about Vec: Vec is heap-allocated.
// Fact about Vec: Vec may re-allocate on growing.
use std::marker::PhantomData;

struct Fact<T> {
    some_fact: &'static str,
    _phtype: PhantomData<T>,
}

impl<T> Fact<T> {
    fn new() -> Self {
        Fact {
            some_fact: "Default Fact: this is Rust",
            _phtype: PhantomData,
        }
    }
    
    // getter
    fn getter(&self) -> &'static str {
        self.some_fact
    }
}

impl<T> Fact<Vec<T>> {
    fn fact(&self) -> Distance<Kilometers> {
        Distance::<Kilometers>::new(self.value * 1.60934)
    }
}
////////////////////////////////////////// EXAMPLE ////////////////////////////////////////////////
// Unit markers (zero-sized types)
struct Miles;
struct Kilometers;

// Generic Distance wrapper with phantom type parameter
struct Distance<T> {
    value: f64,
    _unit: PhantomData<T>, // Marks the unit type without runtime storage
}

impl<T> Distance<T> {
    // Constructor
    fn new(value: f64) -> Self {
        Distance {
            value,
            _unit: PhantomData,
        }
    }
    
    // Getter (works for any unit)
    fn value(&self) -> f64 {
        self.value
    }
}

// Specific implementation for Miles-to-Kilometers conversion
impl Distance<Miles> {
    fn to_kilometers(&self) -> Distance<Kilometers> {
        Distance::<Kilometers>::new(self.value * 1.60934)
    }
}

// Specific implementation for Kilometers-to-Miles conversion
impl Distance<Kilometers> {
    fn to_miles(&self) -> Distance<Miles> {
        Distance::<Miles>::new(self.value / 1.60934)
    }
}

fn main() {
    // Type-safe distance creation
    let distance_to_moon: Distance<Miles> = Distance::new(238_855.0);
    let distance_to_moon_km = distance_to_moon.to_kilometers();
    
    println!("Moon distance: {:.1} miles", distance_to_moon.value());
    println!("Moon distance: {:.1} km", distance_to_moon_km.value());
    
    // Compile-time type safety:
    // let sum = distance_to_moon + distance_to_moon_km; // Won't compile - different units!
 
}
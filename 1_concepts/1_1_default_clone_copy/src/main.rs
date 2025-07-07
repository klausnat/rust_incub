#[derive(Debug, Copy, Clone, Default)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Polyline {
    points: Vec<Point>,
    // to prevent Default derivation
    _non_default: (),
}

impl Polyline {
    fn new(points: Vec<Point>) -> Option<Polyline> {
        match points.len() {
            0 => None,
            _ => Some(Polyline {
                points,
                _non_default: (),
            }),
        }
    }
}

fn main() {
    let p1 = Point::default();
    let p2 = Point::new(1.0, 2.0);
    let p3 = p2; // Copy works

    // Polyline creation
    let points = vec![p1, p2, p3];
    let polyline = Polyline::new(points).unwrap();

    let polyline_clone = polyline.clone(); // Clone works

    println!("Implement me!");
}

#![allow(dead_code)]
struct Circle {
    radius: f64,
}

// Without implementation
fn area(c: &Circle) -> f64 {
    3.14 * c.radius * c.radius
}

// With implementation
impl Circle {
    fn area (&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

fn main() {
    let c = Circle { radius: 5.0 };
    println!("Area: {}", c.area());
}
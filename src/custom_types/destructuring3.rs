#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

fn main() {
    let rect = Rectangle {
        top_left: Point { x: 1.0, y: 5.0 },
        bottom_right: Point { x: 4.0, y: 1.0 },
    };

    // Nested destructuring
    let Rectangle {
        top_left: Point { x: top_left_x, y: top_left_y },
        bottom_right: Point { x: bottom_left_x, y: bottom_right_y },
    } = rect;

    let width = bottom_right_x - top_left_x;
    let height = top_left_y - bottom_right_y;
}
mod field_element;

mod point;
use point::Point;

fn main() {
    let p1 = Point::new_with_numbers(192, 105, 0, 7, 223);
    let p2 = &p1 * 2;

    println!("p: {}", p2);
}

mod field_element;
use field_element::FieldElement;
use rug::Integer;

mod point;
use point::Point;

fn main() {
    let p1 = a_point(192, 105, 0, 7, 223);
    let p2 = p1 * 2;

    println!("p: {}", p2);
}

fn a_point(x: i32, y: i32, a: i32, b: i32, prime: u32) -> Point {
    let xfe = FieldElement::new(Integer::from(x), prime);
    let yfe = FieldElement::new(Integer::from(y), prime);
    let afe = FieldElement::new(Integer::from(a), prime);
    let bfe = FieldElement::new(Integer::from(b), prime);

    Point::new(Some(xfe), Some(yfe), afe, bfe)
}

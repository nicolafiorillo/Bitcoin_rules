mod field_element;
use field_element::FieldElement;
use rug::Integer;

mod point;
use point::Point;

fn main() {
    let _a = FieldElement::new(Integer::from(1), 8);
    let r: i32 = 76 - 12;

    println!("r: {}", r.rem_euclid(13));

    let _point = Point::new(Some(-1), Some(-1), 5, 7);
}

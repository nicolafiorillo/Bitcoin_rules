mod field_element;
use field_element::FieldElement;
use rug::Integer;

mod point;
use point::Point;

fn main() {
    let _a = FieldElement::new(Integer::from(1), 8);
    let r: i32 = 76 - 12;

    let _a = Integer::from(5);

    println!("r: {}", r.rem_euclid(13));

    let x = FieldElement::new(Integer::from(-1), 256);
    let y = FieldElement::new(Integer::from(-1), 256);
    let a = FieldElement::new(Integer::from(5), 256);
    let b = FieldElement::new(Integer::from(7), 256);

    let _point = Point::new(Some(x), Some(y), a, b);
}

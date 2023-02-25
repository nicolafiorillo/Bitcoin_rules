mod field_element;
use field_element::*;

fn main() {
    let _a = FieldElement::new(1, 8);
    let r: i32 = 76 - 12;

    println!("r: {}", r.rem_euclid(13));
}

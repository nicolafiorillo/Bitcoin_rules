use crate::integer_ex::IntegerEx;
use rug::Integer;
use sha256::digest;

pub fn hash256(s: String) -> Integer {
    let mut ll_s = digest(digest(s));
    let mut lr_s = ll_s.split_off(16);
    let mut rl_s = lr_s.split_off(16);
    let rr_s = rl_s.split_off(16);

    let ll = u64::from_str_radix(ll_s.as_str(), 16).unwrap();
    let lr = u64::from_str_radix(lr_s.as_str(), 16).unwrap();
    let rl = u64::from_str_radix(rl_s.as_str(), 16).unwrap();
    let rr = u64::from_str_radix(rr_s.as_str(), 16).unwrap();

    Integer::new_from_256_digits(ll, lr, rl, rr)
}

#[cfg(test)]
mod hash256_test {
    use rug::Integer;

    use crate::integer_ex::IntegerEx;

    use super::hash256;

    #[test]
    fn verify_a_hash() {
        let hashed = hash256("A SECRET".to_string());
        let expected = Integer::new_from_256_digits(
            850352716611885034,
            2634878701457754521,
            16998301821151769569,
            7873941489445698121,
        );

        assert_eq!(hashed, expected);
    }

    #[test]
    fn verify_empty_string_hash() {
        let hashed = hash256("".to_string());
        let expected = Integer::new_from_256_digits(
            14787340370178502671,
            12141869398808674207,
            6623462329246154534,
            15663337444025497830,
        );

        assert_eq!(hashed, expected);
    }
}

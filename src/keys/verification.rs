use rug::Integer;

use crate::{
    bitcoin::ecdsa::{G, N},
    ecdsa::point::Point,
    std_lib::integer_ex::IntegerEx,
};

use super::signature::Signature;

/// Verify `z` versus a `Signature`.
/// `z`: the hashed message
/// `sig`: the public signature
pub fn verify(point: &Point, z: &Integer, signature: &Signature) -> bool {
    let s_inv = signature.s.invert_by_modulo(&N);

    let mu = z * &s_inv;
    let (_q, u) = Integer::from(mu).div_rem_euc((*N).clone());

    let mv = &signature.r * &s_inv;
    let (_q, v) = Integer::from(mv).div_rem_euc((*N).clone());

    let total = (&(*G).clone() * u) + &(point * v);

    total.x_as_num() == signature.r
}

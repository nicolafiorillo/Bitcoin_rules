// @generated automatically by Diesel CLI.

diesel::table! {
    headers (id) {
        id -> Bytea,
        version -> Int4,
        previous_block -> Bytea,
        merkle_root -> Bytea,
        timestamp -> Timestamp,
        bits -> Int4,
        nonce -> Int4,
    }
}

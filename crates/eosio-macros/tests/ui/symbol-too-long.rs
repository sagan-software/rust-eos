extern crate eosio_macros;

use eosio_macros::s;

fn main() {
    let _ = s!(4, ABCDEFGH);
}

pub mod i_can_throw_one;
pub mod i_can_throw_two;
pub mod wrapper;

use crate::wrapper::wrapper;

#[macro_use]
extern crate something;

fn main() {
    if wrapper().is_err() {
        println!("Hello, darkness!");
    } else {
        println!("Hello, world!");
    }
}

use crate::i_can_throw_one::i_can_throw_one; //open file inside proc macro
use crate::i_can_throw_two::i_can_throw_two; //open file inside proc macro

use something::show_streams;

#[derive(ExactHowConceptDerive)]
pub enum WrapperExactHowError {
    One(bool),
    Two(u32),
}

#[show_streams("./src/i_can_throw_one.rs" "./src/i_can_throw_two.rs")]
pub fn wrapper() -> Result<(), WrapperExactHowError> {
    i_can_throw_one()?;
    i_can_throw_two()?;
    Ok(())
}

// // Example: Basic function
// #[show_streams]
// fn invoke1() {}
// // out: attr: ""
// // out: item: "fn invoke1() { }"

// // Example: Attribute with input
// #[show_streams(bar)]
// fn invoke2() {}
// // out: attr: "bar"
// // out: item: "fn invoke2() {}"

// // Example: Multiple tokens in the input
// #[show_streams(multiple => tokens)]
// fn invoke3() {}
// // out: attr: "multiple => tokens"
// // out: item: "fn invoke3() {}"

// // Example:
// #[show_streams { delimiters }]
// fn invoke4() {}
// // out: attr: "delimiters"
// // out: item: "fn invoke4() {}"

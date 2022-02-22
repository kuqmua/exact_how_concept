pub mod i_can_throw_one;
pub mod i_can_throw_two;

use crate::i_can_throw_one::i_can_throw_one;
use crate::i_can_throw_two::i_can_throw_two;

enum WrapperReturn {
    One(bool),
    Two(u32),
}

impl From<bool> for WrapperReturn {
    fn from(e: bool) -> Self {
        WrapperReturn::One(e)
    }
}

impl From<u32> for WrapperReturn {
    fn from(e: u32) -> Self {
        WrapperReturn::Two(e)
    }
}

fn wrapper() -> Result<(), WrapperReturn> {
    i_can_throw_one()?;
    i_can_throw_two()?;
    Ok(())
}

fn main() {
    println!("Hello, world!");
}

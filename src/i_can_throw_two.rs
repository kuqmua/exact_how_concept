// pub struct ICanThrowTwoError {
//     source: u32,
// }

pub fn i_can_throw_two() -> Result<(), u32> {
    Err(32)
}

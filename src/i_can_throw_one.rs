// pub struct ICanThrowOneError {
//     source: bool,
// }

pub fn i_can_throw_one() -> Result<(), bool> {
    Err(false)
}

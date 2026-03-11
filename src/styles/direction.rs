/// The direction that childs will be display
#[derive(Clone, Debug)]
#[cfg_attr(feature = "types_stable_abi", repr(u8))]
pub enum Direction {
    Row,
    Column,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Column
    }
}

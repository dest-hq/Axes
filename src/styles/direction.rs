/// Direction that children will be laid out in
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "types_stable_abi", repr(u8))]
pub enum Direction {
    Row,
    #[default]
    Column,
}

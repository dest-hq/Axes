/// The vertical alignment
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "types_stable_abi", repr(u8))]
pub enum VerticalAlign {
    #[default]
    Top,
    Center,
    Bottom,
}

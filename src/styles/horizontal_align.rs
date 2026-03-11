#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "types_stable_abi", repr(u8))]
pub enum HorizontalAlign {
    #[default]
    Left,
    Center,
    Right,
}

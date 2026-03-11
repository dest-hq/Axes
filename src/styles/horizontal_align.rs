#[derive(Clone, Debug)]
#[cfg_attr(feature = "types_stable_abi", repr(u8))]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

impl Default for HorizontalAlign {
    fn default() -> Self {
        Self::Left
    }
}

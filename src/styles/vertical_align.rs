#[derive(Clone, Debug)]
#[cfg_attr(feature = "types_stable_abi", repr(u8))]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

impl Default for VerticalAlign {
    fn default() -> Self {
        Self::Top
    }
}

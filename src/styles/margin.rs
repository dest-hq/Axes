/// Space around a node
#[derive(Clone, Debug)]
#[cfg_attr(feature = "types_stable_abi", repr(C))]
pub struct Margin {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Default for Margin {
    fn default() -> Self {
        Margin {
            top: 0.0,
            left: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }
}

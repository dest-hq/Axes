/// Space inside a node
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "types_stable_abi", repr(C))]
pub struct Padding {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
}

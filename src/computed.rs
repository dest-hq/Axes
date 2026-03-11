/// The info that will be return about Node
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "types_stable_abi", repr(C))]
pub struct ComputedLayout {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
}

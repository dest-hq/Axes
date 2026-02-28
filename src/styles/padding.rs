#[derive(Clone, Debug)]
/// Space inside an node
pub struct Padding {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Default for Padding {
    fn default() -> Self {
        Padding {
            top: 0.0,
            left: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }
}

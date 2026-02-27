#[derive(Clone, Copy, Debug)]
pub enum Units {
    Pixels(f32),
    /// Range from 0.0 to 1.0
    Percentage(f32),
}

impl Default for Units {
    fn default() -> Self {
        Self::Percentage(1.0)
    }
}

/// Size of node
#[derive(Clone, Copy, Debug, Default)]
pub struct Size {
    pub width: Units,
    pub height: Units,
}

impl From<f32> for Units {
    fn from(value: f32) -> Self {
        Units::Pixels(value)
    }
}

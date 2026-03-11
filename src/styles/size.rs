#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "types_stable_abi", repr(C))]
pub enum Units {
    Pixels(f32),
    /// Range from 0.0 to 1.0
    Percentage(f32),
}

impl Default for Units {
    fn default() -> Self {
        Self::Pixels(0.0)
    }
}

/// Size of node
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "types_stable_abi", repr(C))]
pub struct Size {
    pub width: Units,
    pub height: Units,
}

impl Units {
    pub fn as_pixels(&self) -> f32 {
        match self {
            Units::Pixels(px) => *px,
            Units::Percentage(_) => 0.0, // Shouldn't happen
        }
    }
}

impl From<f32> for Units {
    fn from(value: f32) -> Self {
        Units::Pixels(value)
    }
}

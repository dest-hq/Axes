/// Style of Node
#[derive(Default, Clone, Copy, Debug)]
#[cfg_attr(feature = "types_stable_abi", repr(C))]
pub struct Style {
    pub size: Size,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub gap: Size,
    pub direction: Direction,
    pub margin: Margin,
    pub padding: Padding,
    pub vertical_align: VerticalAlign,
    pub horizontal_align: HorizontalAlign,
}

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

/// Direction that children will be laid out in
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "types_stable_abi", repr(u8))]
pub enum Direction {
    Row,
    #[default]
    Column,
}

/// Space around a node
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "types_stable_abi", repr(C))]
pub struct Margin {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
}

/// Space inside a node
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "types_stable_abi", repr(C))]
pub struct Padding {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
}

/// The vertical alignment
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "types_stable_abi", repr(u8))]
pub enum VerticalAlign {
    #[default]
    Top,
    Center,
    Bottom,
}

#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "types_stable_abi", repr(u8))]
pub enum HorizontalAlign {
    #[default]
    Left,
    Center,
    Right,
}

use freya::prelude::Color;

/// Context for providing an icon colour from a parent component (e.g., a button).
#[derive(Clone, Copy)]
pub struct IconColorContext(pub Color);

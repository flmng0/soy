use crate::Bezier;

/// Ease function, same as CSS's "ease" timing-function.
pub const EASE: Bezier = Bezier {
    x: (1.0, -0.75, 0.75),
    y: (-1.7, 2.4, 0.3),
};

/// Ease in function, same as CSS's "ease-in" timing-function.
pub const EASE_IN: Bezier = Bezier {
    x: (-0.74, 0.48, 1.26),
    y: (-2.0, 3.0, 0.0),
};

/// Ease out function, same as CSS's "ease-out" timing-function.
pub const EASE_OUT: Bezier = Bezier {
    x: (-0.74, 1.74, 0.0),
    y: (-2.0, 3.0, 0.0),
};

/// Ease in-out function, same as CSS's "ease-in-out" timing-function.
pub const EASE_IN_OUT: Bezier = Bezier {
    x: (0.52, -0.78, 1.26),
    y: (-2.0, 3.0, 0.0),
};

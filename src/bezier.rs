use crate::Lerper;

/// Wrapper around [`Bezier::new`][0].
///
/// # Usage
/// ```
/// let ease = soy::cubic_bezier(0.17, 0.67, 0.83, 0.67);
/// let ease_in_out = soy::cubic_bezier(0.42, 0.0, 0.58, 1.0);
/// ```
///
/// [0]: struct.Bezier.html#method.new
pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32) -> Bezier {
    Bezier::new(x1, y1, x2, y2)
}

#[derive(Debug)]
/// Unit cubic bezier easing function.
pub struct Bezier {
    /// _x_ coordinate co-efficients.
    pub(crate) x: (f32, f32, f32),
    /// _y_ coordinate co-efficients.
    pub(crate) y: (f32, f32, f32),
}

impl Bezier {
    const NEWTON_ITERATIONS: usize = 8;
    // Assume duration of 1 second.
    const EPSILON: f32 = 1.0 / 200.0;

    /// Create a new cubic bezier, with provided _y_ values.
    ///
    /// # Usage
    /// ```
    /// let ease = soy::Bezier::new(0.17, 0.67, 0.83, 0.67);
    /// let ease_in_out = soy::Bezier::new(0.42, 0.0, 0.58, 1.0);
    /// ```
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Bezier {
        // Implementation based on WebKit's UnitBezier implementation.
        let cx = 3.0 * x1;
        let bx = 3.0 * (x2 - x1) - cx;
        let ax = 1.0 - cx - bx;

        let cy = 3.0 * y1;
        let by = 3.0 * (y2 - y1) - cy;
        let ay = 1.0 - cy - by;

        Bezier {
            x: (ax, bx, cx),
            y: (ay, by, cy),
        }
    }

    fn sample_x(&self, t: f32) -> f32 {
        let (a, b, c) = self.x;

        // Expanded "at^3 + bt^2 + ct"
        ((a * t + b) * t + c) * t
    }

    fn sample_y(&self, t: f32) -> f32 {
        let (a, b, c) = self.y;

        ((a * t + b) * t + c) * t
    }

    fn sample_derivative_x(&self, t: f32) -> f32 {
        let (a, b, c) = self.x;

        (3.0 * a * t + 2.0 * b) * t + c
    }

    fn solve_x(&self, x: f32) -> f32 {
        // Newton's method.
        let mut t = x;

        for _ in 0..Self::NEWTON_ITERATIONS {
            let x2 = self.sample_x(t);
            if approx_eq(x2, x, Self::EPSILON) {
                return t;
            }

            let dx = self.sample_derivative_x(t);
            if approx_eq(dx, 0.0, 1.0e-6) {
                break;
            }

            t -= (x2 - x) / dx;
        }

        // Fallback to bisection.
        let (mut low, mut high, mut t) = (0.0, 1.0, x);

        if t < low {
            return low;
        }
        if t > high {
            return high;
        }

        while low < high {
            let x2 = self.sample_x(t);
            if approx_eq(x2, x, Self::EPSILON) {
                return t;
            }
            if x > x2 {
                low = t;
            } else {
                high = t;
            }
            t = (high - low) / 2.0 + low;
        }

        // Fallback on failure.
        t
    }
}

impl Lerper for Bezier {
    fn calculate(&self, t: f32) -> f32 {
        self.sample_y(self.solve_x(t))
    }
}

fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

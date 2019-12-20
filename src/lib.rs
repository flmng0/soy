use core::ops::{Add, Mul, Sub};

/// Interpolate between two values given a tweening method.
///
/// # Arguments:
/// - `tweener`: Tweening method to use.
/// - `start`: Initial data point.
/// - `end`: Final data point.
/// - `t`: Amount to interpolate between the values.
///
/// # Example:
/// ```
/// fn main() {
///     let start = 5.0;
///     let end = 10.0;
///
///     let quarter = soy::lerp(soy::Linear, start, end, 0.25);
///     assert_eq!(quarter, 6.25);
///
///     let half_way = soy::lerp(soy::Linear, start, end, 0.5);
///     assert_eq!(half_way, 7.5);
/// }
/// ```
pub fn lerp<T, D>(tweener: T, start: D, end: D, t: f32) -> D
where
    T: Tween,
    D: Copy,
    D: Add<Output = D>,
    D: Sub<Output = D>,
    D: Mul<f32, Output = D>,
{
    start + (end - start) * tweener.calculate(t)
}

/// Trait implemented by all tweening methods.
pub trait Tween {
    /// Given a timing function *y = f(t)*, this method calculates the _y_ value
    /// at the given _t_.
    fn calculate(self, t: f32) -> f32;
}

/// Function signature of a tweening function, used to construct a
/// `soy::Function`.
pub type TweenFunction = fn(f32) -> f32;

/// Timing function formed directly from a function.
///
/// TODO: Example usage
pub struct Function(TweenFunction);

impl Function {
    /// Create a new Function tweener.
    pub fn new(f: TweenFunction) -> Function {
        Function(f)
    }
}

impl Tween for Function {
    fn calculate(self, t: f32) -> f32 {
        (self.0)(t)
    }
}

/// When using a cubic bezier as a function _y = f(x)_, only the _y_ coordinates
/// of each point has an effect on the resulting transition.
pub struct CubicBezier(pub f32, pub f32);

impl CubicBezier {
    /// Create a new cubic bezier, with provided _y_ values.
    pub fn new(p: f32, q: f32) -> CubicBezier {
        CubicBezier(p, q)
    }
}

impl Tween for CubicBezier {
    fn calculate(self, t: f32) -> f32 {
        // https://en.wikipedia.org/wiki/Bézier_curve#Cubic_Bézier_curves
        let CubicBezier(p, q) = self;

        let a = 3.0 * (1.0 - t).powi(2) * t;
        let b = 3.0 * (1.0 - t) * t.powi(2);
        let c = t.powi(3);

        a * p + b * q + c
    }
}

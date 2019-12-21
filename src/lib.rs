//! # Soy
//! Rust interpolation library.
//!
//! # Usage
//! The main trait used for interpolation is [`soy::Lerper`]. It requires a
//! single method, `calculate`, which calculates interpolation progression at a
//! given time.
//!
//! Example implementing linear interpolation, taken directly from `soy`'s
//! implementation:
//! ```
//! struct Linear;
//!
//! impl soy::Lerper for Linear {
//!     fn calculate(&self, t: f32) -> {
//!         t
//!     }
//! }
//! ```
//!
//! [0]: trait.Lerper.html
#![deny(missing_docs)]

mod bezier;
mod constants;

use core::ops::{Add, Mul, Sub};

pub use bezier::{cubic_bezier, Bezier};
pub use constants::*;

/// Interpolate between two values given an interpolation method.
///
/// # Arguments:
/// - `lerper`: Interpolation method to use.
/// - `start`: Initial data point.
/// - `end`: Final data point.
/// - `t`: Amount to interpolate between the values.
///
/// # Usage
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
pub fn lerp<T, D>(lerper: T, start: D, end: D, t: f32) -> D
where
    T: Lerper,
    D: Copy,
    D: Add<Output = D>,
    D: Sub<Output = D>,
    D: Mul<f32, Output = D>,
{
    start + (end - start) * lerper.calculate(t)
}

/// Trait implemented by all interpolating methods.
pub trait Lerper {
    /// Given a timing function _y = f(t)_, this method calculates the _y_ value
    /// at the given _t_.
    fn calculate(&self, t: f32) -> f32;
}

/// Linear interpolator: _f(t) = t_.
pub struct Linear;

impl Lerper for Linear {
    fn calculate(&self, t: f32) -> f32 {
        t
    }
}

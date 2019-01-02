//! Conversions of fundamental numeric and geometric types.

use kurbo::Vec2;

/// This is our own implementation of a "lossy From" trait, representing
/// a conversion that can have precision loss. It is essentially adapted
/// from <https://github.com/rust-lang/rfcs/pull/2484>.
///
/// If and when such a trait is standardized, we should switch to that.
/// Alternatively, a case can be made it should move somewhere else, or
/// we should adopt a similar trait (it has some similarity to AsPrimitive
/// from num_traits).
pub trait RoundFrom<T> {
    fn round_from(x: T) -> Self;
}

/// The companion to `RoundFrom`. As with `From` and `Into`, a blanket
/// implementation is provided; for the most part, implement `RoundFrom`.
pub trait RoundInto<T> {
    fn round_into(self) -> T;
}

impl<T, U> RoundInto<U> for T
where
    U: RoundFrom<T>,
{
    fn round_into(self) -> U {
        U::round_from(self)
    }
}

impl RoundFrom<f64> for f32 {
    fn round_from(x: f64) -> f32 {
        x as f32
    }
}

impl RoundFrom<Vec2> for (f32, f32) {
    fn round_from(p: Vec2) -> (f32, f32) {
        (p.x as f32, p.y as f32)
    }
}

impl RoundFrom<(f32, f32)> for Vec2 {
    fn round_from(p: (f32, f32)) -> Vec2 {
        Vec2::new(p.0 as f64, p.1 as f64)
    }
}

impl RoundFrom<Vec2> for (f64, f64) {
    fn round_from(p: Vec2) -> (f64, f64) {
        (p.x, p.y)
    }
}

impl RoundFrom<(f64, f64)> for Vec2 {
    fn round_from(p: (f64, f64)) -> Vec2 {
        Vec2::new(p.0, p.1)
    }
}

/// Blanket implementation, no conversion needed.
impl<T> RoundFrom<T> for T {
    fn round_from(x: T) -> T {
        x
    }
}
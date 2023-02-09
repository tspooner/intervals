//! Module containing bound types.
use std::fmt;

/// Trait for types that represent upper/lower bounds.
pub trait Bound: crate::private::Sealed {
    /// Underlying type associated with the bound.
    type Value: PartialOrd;

    /// Corresponding bound given inclusion of limit point.
    type WithLimit: Bound<Value = Self::Value>;

    /// Returns the value of the bound if one exists.
    fn value(&self) -> Option<&Self::Value>;

    /// Returns true if the bound is open.
    fn is_open(&self) -> bool;

    /// Returns true if the bound is closed.
    fn is_closed(&self) -> bool;

    /// Returns the corresponding bound with its limit point.
    fn with_limit_point(self) -> Self::WithLimit;
}

/// Trait for bounds that are open or closed.
pub trait ProperBound: Bound {
    fn proper_value(&self) -> &Self::Value;
}

/// Trait for formatting bound upper/lower bound strings.
pub trait BoundDisplay: Bound {
    fn fmt_left(&self, f: &mut fmt::Formatter) -> fmt::Result;

    fn fmt_right(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

/// Trait for "pinching" bounds on the left and right.
///
/// We define the pinch operation as the logic applied at the left and
/// right boundaries of a pair of intervals when taking the intersection.
pub trait Pinch<T>: Bound {
    type Left: Bound<Value = Self::Value>;
    type Right: Bound<Value = Self::Value>;

    fn pinch_left(self, other: T) -> Self::Left;

    fn pinch_right(self, other: T) -> Self::Right;
}

mod no_bound;
pub use self::no_bound::NoBound;

mod open;
pub use self::open::Open;

mod closed;
pub use self::closed::Closed;

mod mixed;
pub use self::mixed::OpenOrClosed;

///////////////////////////////////////////////////////////////////
// Validation
///////////////////////////////////////////////////////////////////
/// Validate left-right bounds.
pub fn validate<L: Bound, R: Bound>(left: L, right: R) -> ValidationResult<L, R>
where
    Validator: ValidateBounds<L, R>,
{
    <Validator as ValidateBounds<L, R>>::validate(left, right)
}

/// Utility type for validation of bound pairs.
pub struct Validator;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum ValidationError<L, R> {
    DecreasingBounds(L, R)
}

pub type ValidationResult<L, R> = Result<(L, R), ValidationError<L, R>>;

pub trait ValidateBounds<L: Bound, R: Bound> {
    fn validate(left: L, right: R) -> ValidationResult<L, R>;
}

macro_rules! impl_val {
    ($v:ident; $l:ty, $r:ty) => {
        impl<V: PartialOrd> ValidateBounds<$l, $r> for Validator {
            fn validate(l: $l, r: $r) -> ValidationResult<$l, $r> { Ok((l, r)) }
        }
    };
    ($v:ident: $v0:ident $(+ $vt:ident)*; $l:ty, $r:ty) => {
        impl<$v: $v0 $(+ $vt)*> ValidateBounds<$l, $r> for Validator {
            fn validate(l: $l, r: $r) -> ValidationResult<$l, $r> {
                if l.0 > r.0 {
                    Err(ValidationError::DecreasingBounds(l, r))
                } else {
                    Ok((l, r))
                }
            }
        }
    };
}

impl<V: PartialOrd> ValidateBounds<NoBound<V>, NoBound<V>> for Validator {
    fn validate(l: NoBound<V>, r: NoBound<V>) -> ValidationResult<NoBound<V>, NoBound<V>> {
        Ok((l, r))
    }
}

impl_val!(V; NoBound<V>, Open<V>);
impl_val!(V; NoBound<V>, Closed<V>);

impl_val!(V; Open<V>, NoBound<V>);
impl_val!(V; Closed<V>, NoBound<V>);

impl_val!(V: PartialOrd; Open<V>, Open<V>);
impl_val!(V: PartialOrd; Closed<V>, Open<V>);
impl_val!(V: PartialOrd; Open<V>, Closed<V>);
impl_val!(V: PartialOrd; Closed<V>, Closed<V>);

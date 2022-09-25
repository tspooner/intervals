//! Module containing bound types.
use std::fmt;

/// Trait for types that represent upper/lower bounds.
pub trait Bound: crate::private::Sealed {
    /// Underlying type associated with the bound.
    type Value;

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

/// Trait for "pinching" bounds upwards and downwards.
pub trait Pinch<T>: Bound {
    type Up: Bound<Value = Self::Value>;
    type Down: Bound<Value = Self::Value>;

    fn pinch_up(self, other: T) -> Self::Up;

    fn pinch_down(self, other: T) -> Self::Down;
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
        impl<V> ValidateBounds<$l, $r> for Validator {
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

impl<V> ValidateBounds<NoBound<V>, NoBound<V>> for Validator {
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

// PartialEq => Open<V>
impl<V> std::cmp::PartialEq<Closed<V>> for Open<V> {
    fn eq(&self, _: &Closed<V>) -> bool { false }
}

impl<V> std::cmp::PartialEq<NoBound<V>> for Open<V> {
    fn eq(&self, _: &NoBound<V>) -> bool { false }
}

impl<V: PartialEq> std::cmp::PartialEq<OpenOrClosed<V>> for Open<V> {
    fn eq(&self, rhs: &OpenOrClosed<V>) -> bool {
        match rhs {
            &OpenOrClosed::Open(ref inner) => self.0.eq(&inner),
            _ => false,
        }
    }
}

// PartialEq => Closed<V>
impl<V> std::cmp::PartialEq<Open<V>> for Closed<V> {
    fn eq(&self, _: &Open<V>) -> bool { false }
}

impl<V> std::cmp::PartialEq<NoBound<V>> for Closed<V> {
    fn eq(&self, _: &NoBound<V>) -> bool { false }
}

impl<V: PartialEq> std::cmp::PartialEq<OpenOrClosed<V>> for Closed<V> {
    fn eq(&self, rhs: &OpenOrClosed<V>) -> bool {
        match rhs {
            &OpenOrClosed::Closed(ref inner) => self.0.eq(&inner),
            _ => false,
        }
    }
}

// PartialEq => NoBound<V>
impl<V> std::cmp::PartialEq<Open<V>> for NoBound<V> {
    fn eq(&self, _: &Open<V>) -> bool { false }
}

impl<V> std::cmp::PartialEq<Closed<V>> for NoBound<V> {
    fn eq(&self, _: &Closed<V>) -> bool { false }
}

impl<V> std::cmp::PartialEq<OpenOrClosed<V>> for NoBound<V> {
    fn eq(&self, _: &OpenOrClosed<V>) -> bool { false }
}

// PartialEq => OpenOrClosed<V>
impl<V: PartialEq> std::cmp::PartialEq<Open<V>> for OpenOrClosed<V> {
    fn eq(&self, rhs: &Open<V>) -> bool {
        match self {
            &OpenOrClosed::Open(ref inner) => inner.eq(&rhs.0),
            _ => false,
        }
    }
}

impl<V: PartialEq> std::cmp::PartialEq<Closed<V>> for OpenOrClosed<V> {
    fn eq(&self, rhs: &Closed<V>) -> bool {
        match self {
            &OpenOrClosed::Closed(ref inner) => inner.eq(&rhs.0),
            _ => false,
        }
    }
}

impl<V> std::cmp::PartialEq<NoBound<V>> for OpenOrClosed<V> {
    fn eq(&self, _: &NoBound<V>) -> bool { false }
}

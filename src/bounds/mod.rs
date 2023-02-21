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
///
/// Note: [Pinch] can be seen as the inverse of [Unroll].
pub trait Pinch<T>: Bound {
    type Left: Bound<Value = Self::Value>;
    type Right: Bound<Value = Self::Value>;

    /// Returns the left-pinched bound.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate intervals;
    /// # use intervals::bounds::{self, Pinch};
    /// let a = bounds::Closed(1.0f64);
    /// let b = bounds::Open(2.0f64);
    ///
    /// assert_eq!(a.pinch_left(b), b);
    /// ```
    fn pinch_left(self, other: T) -> Self::Left;

    /// Returns the right-pinched bound.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate intervals;
    /// # use intervals::bounds::{self, Pinch};
    /// let a = bounds::Closed(1.0f64);
    /// let b = bounds::Open(2.0f64);
    ///
    /// assert_eq!(a.pinch_right(b), a);
    /// ```
    fn pinch_right(self, other: T) -> Self::Right;
}

/// Trait for "unrolling" bounds on the left and right.
///
/// We define the unroll operation as the logic applied at the left and
/// right boundaries of a pair of intervals during the union-closure operation,
/// but before inclusion of the limits.
///
/// Note: [Unroll] can be seen as the inverse of [Pinch].
pub trait Unroll<T>: Bound {
    type Left: Bound<Value = Self::Value>;
    type Right: Bound<Value = Self::Value>;

    /// Returns the left-unrolled bound.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate intervals;
    /// # use intervals::bounds::{self, Unroll};
    /// let a = bounds::Closed(1.0f64);
    /// let b = bounds::Open(2.0f64);
    ///
    /// assert_eq!(a.unroll_left(b), a);
    /// ```
    fn unroll_left(self, other: T) -> Self::Left;

    /// Returns the right-unrolled bound.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate intervals;
    /// # use intervals::bounds::{self, Unroll};
    /// let a = bounds::Closed(1.0f64);
    /// let b = bounds::Open(2.0f64);
    ///
    /// assert_eq!(a.unroll_right(b), b);
    /// ```
    fn unroll_right(self, other: T) -> Self::Right;
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
    ($v:ident; NoBound, $r:ty) => {
        impl<V: PartialOrd> ValidateBounds<NoBound<$v>, $r> for Validator {
            fn validate(l: NoBound<$v>, r: $r) -> ValidationResult<NoBound<$v>, $r> { Ok((l, r)) }
        }
    };
    ($v:ident; $l:ty, NoBound) => {
        impl<V: PartialOrd> ValidateBounds<$l, NoBound<$v>> for Validator {
            fn validate(l: $l, r: NoBound<$v>) -> ValidationResult<$l, NoBound<$v>> { Ok((l, r)) }
        }
    };
    ($v:ident; $l:ty, $r:ty) => {
        impl<$v: PartialOrd> ValidateBounds<$l, $r> for Validator {
            fn validate(l: $l, r: $r) -> ValidationResult<$l, $r> {
                if l.proper_value() > r.proper_value() {
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

// Unbounded cases:
impl_val!(V; NoBound, Open<V>);
impl_val!(V; NoBound, Closed<V>);
impl_val!(V; NoBound, OpenOrClosed<V>);

// Simple cases:
impl_val!(V; Open<V>, NoBound);
impl_val!(V; Closed<V>, NoBound);
impl_val!(V; Closed<V>, Closed<V>);
impl_val!(V; OpenOrClosed<V>, NoBound);

// Mixed cases:
impl<V: PartialOrd> ValidateBounds<Closed<V>, Open<V>> for Validator {
    fn validate(l: Closed<V>, r: Open<V>) -> ValidationResult<Closed<V>, Open<V>> {
        if l.0 >= r.0 {
            Err(ValidationError::DecreasingBounds(l, r))
        } else {
            Ok((l, r))
        }
    }
}

impl<V: PartialOrd> ValidateBounds<Open<V>, Closed<V>> for Validator {
    fn validate(l: Open<V>, r: Closed<V>) -> ValidationResult<Open<V>, Closed<V>> {
        if l.0 >= r.0 {
            Err(ValidationError::DecreasingBounds(l, r))
        } else {
            Ok((l, r))
        }
    }
}

impl<V: PartialOrd> ValidateBounds<OpenOrClosed<V>, Closed<V>> for Validator {
    fn validate(l: OpenOrClosed<V>, r: Closed<V>) -> ValidationResult<OpenOrClosed<V>, Closed<V>> {
        let is_invalid = match &l {
            OpenOrClosed::Open(x) => x >= &r.0,
            OpenOrClosed::Closed(x) => x > &r.0,
        };

        if is_invalid {
            Err(ValidationError::DecreasingBounds(l, r))
        } else {
            Ok((l, r))
        }
    }
}

impl<V: PartialOrd> ValidateBounds<Closed<V>, OpenOrClosed<V>> for Validator {
    fn validate(l: Closed<V>, r: OpenOrClosed<V>) -> ValidationResult<Closed<V>, OpenOrClosed<V>> {
        let is_invalid = match &r {
            OpenOrClosed::Open(x) => &l.0 >= x,
            OpenOrClosed::Closed(x) => &l.0 > x,
        };

        if is_invalid {
            Err(ValidationError::DecreasingBounds(l, r))
        } else {
            Ok((l, r))
        }
    }
}

// All-Open cases:
impl<V: PartialOrd> ValidateBounds<Open<V>, Open<V>> for Validator {
    fn validate(l: Open<V>, r: Open<V>) -> ValidationResult<Open<V>, Open<V>> {
        if l.0 >= r.0 {
            Err(ValidationError::DecreasingBounds(l, r))
        } else {
            Ok((l, r))
        }
    }
}

impl<V: PartialOrd> ValidateBounds<OpenOrClosed<V>, Open<V>> for Validator {
    fn validate(l: OpenOrClosed<V>, r: Open<V>) -> ValidationResult<OpenOrClosed<V>, Open<V>> {
        let is_invalid = match &l {
            OpenOrClosed::Open(x) | OpenOrClosed::Closed(x) => x >= &r.0,
        };

        if is_invalid {
            Err(ValidationError::DecreasingBounds(l, r))
        } else {
            Ok((l, r))
        }
    }
}

impl<V: PartialOrd> ValidateBounds<Open<V>, OpenOrClosed<V>> for Validator {
    fn validate(l: Open<V>, r: OpenOrClosed<V>) -> ValidationResult<Open<V>, OpenOrClosed<V>> {
        let is_invalid = match &r {
            OpenOrClosed::Open(x) | OpenOrClosed::Closed(x) => &l.0 >= x,
        };

        if is_invalid {
            Err(ValidationError::DecreasingBounds(l, r))
        } else {
            Ok((l, r))
        }
    }
}

impl<V: PartialOrd> ValidateBounds<OpenOrClosed<V>, OpenOrClosed<V>> for Validator {
    fn validate(l: OpenOrClosed<V>, r: OpenOrClosed<V>) -> ValidationResult<OpenOrClosed<V>, OpenOrClosed<V>> {
        let is_invalid = match (&l, &r) {
            (&OpenOrClosed::Open(ref x), &OpenOrClosed::Open(ref y))
                | (&OpenOrClosed::Closed(ref x), &OpenOrClosed::Open(ref y))
                | (&OpenOrClosed::Open(ref x), &OpenOrClosed::Closed(ref y)) => x >= y,
            (&OpenOrClosed::Closed(ref x), &OpenOrClosed::Closed(ref y)) => x > y,
        };

        if is_invalid {
            Err(ValidationError::DecreasingBounds(l, r))
        } else {
            Ok((l, r))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_unbounded() {
        assert!(validate(NoBound::<f64>::new(), NoBound::<f64>::new()).is_ok());

        assert!(validate(NoBound::new(), Open(0.0f64)).is_ok());
        assert!(validate(NoBound::new(), Closed(0.0f64)).is_ok());
        assert!(validate(NoBound::new(), OpenOrClosed::Open(0.0f64)).is_ok());
        assert!(validate(NoBound::new(), OpenOrClosed::Closed(0.0f64)).is_ok());

        assert!(validate(Open(0.0f64), NoBound::new()).is_ok());
        assert!(validate(Closed(0.0f64), NoBound::new()).is_ok());
        assert!(validate(OpenOrClosed::Open(0.0f64), NoBound::new()).is_ok());
        assert!(validate(OpenOrClosed::Closed(0.0f64), NoBound::new()).is_ok());
    }

    #[test]
    fn test_validate_allopen() {
        assert!(validate(Open(0.0f64), Open(-1.0f64)).is_err());
        assert!(validate(Open(0.0f64), Open(0.0f64)).is_err());
        assert!(validate(Open(0.0f64), Open(1.0f64)).is_ok());

        assert!(validate(Open(0.0f64), OpenOrClosed::Open(-1.0f64)).is_err());
        assert!(validate(Open(0.0f64), OpenOrClosed::Open(0.0f64)).is_err());
        assert!(validate(Open(0.0f64), OpenOrClosed::Open(1.0f64)).is_ok());

        assert!(validate(OpenOrClosed::Open(0.0f64), Open(-1.0f64)).is_err());
        assert!(validate(OpenOrClosed::Open(0.0f64), Open(0.0f64)).is_err());
        assert!(validate(OpenOrClosed::Open(0.0f64), Open(1.0f64)).is_ok());

        assert!(validate(OpenOrClosed::Open(0.0f64), OpenOrClosed::Open(-1.0f64)).is_err());
        assert!(validate(OpenOrClosed::Open(0.0f64), OpenOrClosed::Open(0.0f64)).is_err());
        assert!(validate(OpenOrClosed::Open(0.0f64), OpenOrClosed::Open(1.0f64)).is_ok());
    }

    #[test]
    fn test_validate_closed() {
        assert!(validate(Closed(0.0f64), Closed(-1.0f64)).is_err());
        assert!(validate(Closed(0.0f64), Closed(0.0f64)).is_ok());
        assert!(validate(Closed(0.0f64), Closed(1.0f64)).is_ok());

        assert!(validate(Closed(0.0f64), OpenOrClosed::Closed(-1.0f64)).is_err());
        assert!(validate(Closed(0.0f64), OpenOrClosed::Closed(0.0f64)).is_ok());
        assert!(validate(Closed(0.0f64), OpenOrClosed::Closed(1.0f64)).is_ok());

        assert!(validate(OpenOrClosed::Closed(0.0f64), Closed(-1.0f64)).is_err());
        assert!(validate(OpenOrClosed::Closed(0.0f64), Closed(0.0f64)).is_ok());
        assert!(validate(OpenOrClosed::Closed(0.0f64), Closed(1.0f64)).is_ok());
    }

    #[test]
    fn test_validate_mixed() {
        assert!(validate(Closed(0.0f64), Open(-1.0f64)).is_err());
        assert!(validate(Closed(0.0f64), Open(0.0f64)).is_err());
        assert!(validate(Closed(0.0f64), Open(1.0f64)).is_ok());

        assert!(validate(Closed(0.0f64), OpenOrClosed::Open(-1.0f64)).is_err());
        assert!(validate(Closed(0.0f64), OpenOrClosed::Open(0.0f64)).is_err());
        assert!(validate(Closed(0.0f64), OpenOrClosed::Open(1.0f64)).is_ok());

        assert!(validate(OpenOrClosed::Closed(0.0f64), Open(-1.0f64)).is_err());
        assert!(validate(OpenOrClosed::Closed(0.0f64), Open(0.0f64)).is_err());
        assert!(validate(OpenOrClosed::Closed(0.0f64), Open(1.0f64)).is_ok());

        assert!(validate(OpenOrClosed::Closed(0.0f64), OpenOrClosed::Open(-1.0f64)).is_err());
        assert!(validate(OpenOrClosed::Closed(0.0f64), OpenOrClosed::Open(0.0f64)).is_err());
        assert!(validate(OpenOrClosed::Closed(0.0f64), OpenOrClosed::Open(1.0f64)).is_ok());

        assert!(validate(Open(0.0f64), Closed(-1.0f64)).is_err());
        assert!(validate(Open(0.0f64), Closed(0.0f64)).is_err());
        assert!(validate(Open(0.0f64), Closed(1.0f64)).is_ok());

        assert!(validate(Open(0.0f64), OpenOrClosed::Closed(-1.0f64)).is_err());
        assert!(validate(Open(0.0f64), OpenOrClosed::Closed(0.0f64)).is_err());
        assert!(validate(Open(0.0f64), OpenOrClosed::Closed(1.0f64)).is_ok());

        assert!(validate(OpenOrClosed::Open(0.0f64), Closed(-1.0f64)).is_err());
        assert!(validate(OpenOrClosed::Open(0.0f64), Closed(0.0f64)).is_err());
        assert!(validate(OpenOrClosed::Open(0.0f64), Closed(1.0f64)).is_ok());

        assert!(validate(OpenOrClosed::Open(0.0f64), OpenOrClosed::Closed(-1.0f64)).is_err());
        assert!(validate(OpenOrClosed::Open(0.0f64), OpenOrClosed::Closed(0.0f64)).is_err());
        assert!(validate(OpenOrClosed::Open(0.0f64), OpenOrClosed::Closed(1.0f64)).is_ok());
    }
}

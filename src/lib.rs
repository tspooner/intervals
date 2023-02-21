//! Generalised types for intervals and partitions thereof.
//!
//! `intervals` is oriented towards static-typing. The bounds are all
//! unique types, all operations between instances are exhaustively
//! implemented, and formatting is provided for ease-of-use.
extern crate num_traits;

#[cfg_attr(feature = "serde", macro_use)]
#[cfg(feature = "serde")]
extern crate serde_crate;

use num_traits::{Zero, One, Unsigned};

mod private {
    pub trait Sealed {}
}

pub mod bounds;
pub mod partitions;

pub type Result<T, L, R> = std::result::Result<T, bounds::ValidationError<L, R>>;
pub type IntervalResult<L, R = L> = Result<Interval<L, R>, L, R>;

/// Generalised type representing an interval between two points: a and b.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate intervals;
/// # use intervals::{Interval, bounds};
/// let x = Interval::closed_unchecked(-1.0, 0.0);
/// let y = Interval::closed_unchecked(0.0, 1.0);
///
/// assert!(x.contains(-0.5));
/// assert!(y.contains(0.5));
///
/// assert_eq!(x.intersect(y).unwrap(), Interval::degenerate(0.0));
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct Interval<L: bounds::Bound, R: bounds::Bound<Value = L::Value>> {
    /// The left-hand bound.
    pub left: L,

    /// The right-hand bound.
    pub right: R,
}

/// Alias for an unbounded interval.
pub type Unbounded<V> = Interval<bounds::NoBound<V>, bounds::NoBound<V>>;

/// Alias for a bounded open interval.
pub type Open<V> = Interval<bounds::Open<V>, bounds::Open<V>>;

/// Alias for a left-open interval, unbounded on the right.
pub type LeftOpen<V> = Interval<bounds::Open<V>, bounds::NoBound<V>>;

/// Alias for a right-open interval, unbounded on the left.
pub type RightOpen<V> = Interval<bounds::NoBound<V>, bounds::Open<V>>;

/// Alias for a bounded closed interval.
pub type Closed<V> = Interval<bounds::Closed<V>, bounds::Closed<V>>;

/// Alias for a left-closed interval, unbounded on the right.
pub type LeftClosed<V> = Interval<bounds::Closed<V>, bounds::NoBound<V>>;

/// Alias for a right-closed interval, unbounded on the left.
pub type RightClosed<V> = Interval<bounds::NoBound<V>, bounds::Closed<V>>;

/// Alias for a left-closed, right-open interval.
pub type LCRO<V> = Interval<bounds::Closed<V>, bounds::Open<V>>;

/// Alias for a left-open, right-closed interval.
pub type LORC<V> = Interval<bounds::Open<V>, bounds::Closed<V>>;

///////////////////////////////////////////////////////////////////////////////
// Core
///////////////////////////////////////////////////////////////////////////////
impl<L, R, LL, RR> PartialEq<Interval<LL, RR>> for Interval<L, R>
where
    L: bounds::Bound + PartialEq<LL>,
    R: bounds::Bound<Value = L::Value> + PartialEq<RR>,
    LL: bounds::Bound,
    RR: bounds::Bound<Value = LL::Value>,
{
    fn eq(&self, rhs: &Interval<LL, RR>) -> bool {
        self.left == rhs.left && self.right == rhs.right
    }
}

impl<L, R> Interval<L, R>
where
    L: bounds::Bound,
    R: bounds::Bound<Value = L::Value>,

    bounds::Validator: bounds::ValidateBounds<L, R>,
{
    /// Construct an interval with bound validation.
    pub fn new(left: L, right: R) -> IntervalResult<L, R> {
        bounds::validate(left, right).map(|(left, right)| Interval { left, right, })
    }
}

impl<L, R> Interval<L, R>
where
    L: bounds::Bound,
    R: bounds::Bound<Value = L::Value>,
{
    /// Construct an interval w/o bound validation.
    pub fn new_unchecked(left: L, right: R) -> Self { Interval { left, right, } }
}

impl<L: bounds::Bound> Interval<L, bounds::NoBound<L::Value>> {
    /// Construct a left-bounded interval, unbounded on the right.
    pub fn left_bounded(left: L) -> Self {
        Interval {
            left,
            right: bounds::NoBound::new(),
        }
    }
}

impl<V: PartialOrd> LeftOpen<V> {
    /// Construct a left-open interval, unbounded on the right.
    pub fn left_open(left: V) -> Self {
        Interval {
            left: bounds::Open(left),
            right: bounds::NoBound::new(),
        }
    }
}

impl<V: PartialOrd> LeftClosed<V> {
    /// Construct a left-closed interval, unbounded on the right.
    pub fn left_closed(left: V) -> Self {
        Interval {
            left: bounds::Closed(left),
            right: bounds::NoBound::new(),
        }
    }
}

impl<R: bounds::Bound> Interval<bounds::NoBound<R::Value>, R> {
    /// Construct a right-bounded interval, unbounded on the left.
    pub fn right_bounded(right: R) -> Self {
        Interval {
            left: bounds::NoBound::new(),
            right,
        }
    }
}

impl<V: PartialOrd> RightOpen<V> {
    /// Construct a right-open interval, unbounded on the left.
    pub fn right_open(right: V) -> Self {
        Interval {
            left: bounds::NoBound::new(),
            right: bounds::Open(right),
        }
    }
}

impl<V: PartialOrd> RightClosed<V> {
    /// Construct a right-closed interval, unbounded on the left.
    pub fn right_closed(right: V) -> Self {
        Interval {
            left: bounds::NoBound::new(),
            right: bounds::Closed(right),
        }
    }
}

impl<V: PartialOrd> LORC<V> {
    /// Construct a left-open, right-closed interval with bound validation.
    pub fn lorc(left: V, right: V) -> Result<Self, bounds::Open<V>, bounds::Closed<V>> {
        Interval::new(bounds::Open(left), bounds::Closed(right))
    }

    /// Construct a left-open, right-closed interval w/o bound validation.
    pub fn lorc_unchecked(left: V, right: V) -> Self {
        Interval::new_unchecked(bounds::Open(left), bounds::Closed(right))
    }
}

impl<V: PartialOrd> LCRO<V> {
    /// Construct a left-closed, right-open interval with bound validation.
    pub fn lcro(left: V, right: V) -> Result<Self, bounds::Closed<V>, bounds::Open<V>> {
        Interval::new(bounds::Closed(left), bounds::Open(right))
    }

    /// Construct a left-closed, right-open interval w/o bound validation.
    pub fn lcro_unchecked(left: V, right: V) -> Self {
        Interval::new_unchecked(bounds::Closed(left), bounds::Open(right))
    }
}

impl<V: PartialOrd> Unbounded<V> {
    /// Construct a totally unbounded interval.
    pub fn unbounded() -> Self {
        Interval {
            left: bounds::NoBound::new(),
            right: bounds::NoBound::new(),
        }
    }
}

impl<V: PartialOrd> Open<V> {
    /// Construct a bounded open interval with bound validation.
    pub fn open(left: V, right: V) -> IntervalResult<bounds::Open<V>, bounds::Open<V>> {
        Interval::new(bounds::Open(left), bounds::Open(right))
    }

    /// Construct a bounded open interval w/o bound validation.
    pub fn open_unchecked(left: V, right: V) -> Self {
        Interval::new_unchecked(bounds::Open(left), bounds::Open(right))
    }
}

impl<V: PartialOrd> Closed<V> {
    /// Construct a bounded closed interval with bound validation.
    pub fn closed(left: V, right: V) -> IntervalResult<bounds::Closed<V>, bounds::Closed<V>> {
        Interval::new(bounds::Closed(left), bounds::Closed(right))
    }

    /// Construct a bounded closed interval w/o bound validation.
    pub fn closed_unchecked(left: V, right: V) -> Self {
        Interval::new_unchecked(bounds::Closed(left), bounds::Closed(right))
    }
}

impl<V: PartialOrd + Clone> Closed<V> {
    /// Construct a degenerate interval: [a, a].
    pub fn degenerate(value: V) -> Self {
        Interval::new_unchecked(bounds::Closed(value.clone()), bounds::Closed(value))
    }
}

impl<V: Zero + One + PartialOrd> Closed<V> {
    /// Construct a unit interval: [0, 1].
    pub fn unit() -> Self {
        Interval::closed_unchecked(V::zero(), V::one())
    }
}

impl<V: PartialOrd> Closed<V> {
    /// Construct a uniform partition over the interval.
    pub fn linspace(self, n_partitions: usize) -> partitions::Uniform<V> {
        partitions::Uniform {
            size: n_partitions,
            left: self.left.0,
            right: self.right.0,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Ops
///////////////////////////////////////////////////////////////////////////////
/// Type alias to simplify intersection return types.
pub type IntersectionOf<L, R, LL, RR> = Interval<
    <L as bounds::Pinch<LL>>::Left,
    <R as bounds::Pinch<RR>>::Right,
>;

impl<L, R> Interval<L, R>
where
    L: bounds::Bound,
    R: bounds::Bound<Value = L::Value>,

    L::Value: PartialOrd,
{
    pub fn intersect<LL, RR>(self, other: Interval<LL, RR>) -> Option<IntersectionOf<L, R, LL, RR>>
    where
        L: bounds::Pinch<LL>,
        R: bounds::Pinch<RR>,

        LL: bounds::Bound,
        RR: bounds::Bound<Value = LL::Value>,

        bounds::Validator: bounds::ValidateBounds<L::Left, R::Right>,
    {
        let left = self.left.pinch_left(other.left);
        let right = self.right.pinch_right(other.right);

        Interval::new(left, right).ok()
    }
}

/// Type alias to simplify union-closure return types.
pub type UnionClosureOf<L, R, LL, RR> = Interval<
    <<L as bounds::Unroll<LL>>::Left as bounds::Bound>::WithLimit,
    <<R as bounds::Unroll<RR>>::Right as bounds::Bound>::WithLimit
>;

impl<L, R> Interval<L, R>
where
    L: bounds::Bound,
    R: bounds::Bound<Value = L::Value>,

    L::Value: PartialOrd,
{
    pub fn union_closure<LL, RR>(self, other: Interval<LL, RR>) -> UnionClosureOf<L, R, LL, RR>
    where
        L: bounds::Unroll<LL>,
        R: bounds::Unroll<RR>,

        LL: bounds::Bound,
        RR: bounds::Bound<Value = LL::Value>,
    {
        use bounds::Bound;

        let left = self.left.unroll_left(other.left).with_limit_point();
        let right = self.right.unroll_right(other.right).with_limit_point();

        Interval::new_unchecked(left, right)
    }
}

impl<L, R> Interval<L, R>
where
    L: bounds::Bound,
    R: bounds::Bound<Value = L::Value>,
{
    /// Returns true if the interval contains `val`.
    ///
    /// __Note__: see [Contains] for more details.
    pub fn contains(&self, val: L::Value) -> bool
    where
        Self: Contains<L, R>
    {
        Contains::<L, R>::contains(self, val)
    }

    /// Returns true if the interval is degenerate.
    ///
    /// A degenerate interval is bounded, where the upper and lower bounds are equal.
    ///
    /// # Examples
    /// ```
    /// # extern crate intervals;
    /// # use intervals::Interval;
    /// assert!(Interval::closed_unchecked(0.0, 0.0).is_degenerate());
    /// assert!(!Interval::open_unchecked(0.0, 0.0).is_degenerate());
    /// ```
    pub fn is_degenerate<'a>(&'a self) -> bool
    where
        &'a L::Value: PartialEq
    {
        match (self.left.value(), self.right.value()) {
            (Some(left), Some(right)) if self.left.is_closed() && self.right.is_closed()
                => left == right,
            _ => false,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Formatting
///////////////////////////////////////////////////////////////////////////////
impl<L, R> std::fmt::Display for Interval<L, R>
where
    L: bounds::BoundDisplay,
    R: bounds::BoundDisplay<Value = L::Value>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.left.fmt_left(f)
            .and_then(|_| write!(f, ", "))
            .and_then(|_| self.right.fmt_right(f))
    }
}

///////////////////////////////////////////////////////////////////////////////
// Boundedness
///////////////////////////////////////////////////////////////////////////////
/// Marker trait for bounded intervals.
///
/// A bounded interval is defined as any such interval for which the left/right
/// limits are open or closed.
pub trait Bounded: private::Sealed {}

macro_rules! impl_bounded {
    (V: $($trait:ident),* => $l:ty, $r:ty) => {
        impl<V: PartialOrd + $($trait),*> private::Sealed for Interval<$l, $r> {}
        impl<V: PartialOrd + $($trait),*> Bounded for Interval<$l, $r> {}
    };
    ($l:ty, $r:ty) => {
        impl<V: PartialOrd> private::Sealed for Interval<$l, $r> {}
        impl<V: PartialOrd> Bounded for Interval<$l, $r> {}
    };
}

impl_bounded!(V: Unsigned => bounds::NoBound<V>, bounds::Open<V>);
impl_bounded!(V: Unsigned => bounds::NoBound<V>, bounds::Closed<V>);

impl_bounded!(bounds::Closed<V>, bounds::Closed<V>);
impl_bounded!(bounds::Closed<V>, bounds::Open<V>);
impl_bounded!(bounds::Open<V>, bounds::Closed<V>);
impl_bounded!(bounds::Open<V>, bounds::Open<V>);

///////////////////////////////////////////////////////////////////////////////
// Containment
///////////////////////////////////////////////////////////////////////////////
/// Trait for intervals which can assert containment of their values.
pub trait Contains<L: bounds::Bound, R: bounds::Bound<Value = L::Value>> {
    /// Returns true if the interval contains `val`.
    ///
    /// # Examples
    /// ```
    /// # use intervals::{Interval, bounds};
    /// assert!(!Interval::unit().contains(-0.5));
    /// assert!(Interval::unit().contains(0.0));
    /// assert!(Interval::unit().contains(0.5));
    /// assert!(Interval::unit().contains(1.0));
    /// ```
    fn contains(&self, val: L::Value) -> bool;
}

impl<V: PartialOrd> Contains<bounds::NoBound<V>, bounds::NoBound<V>> for Unbounded<V> {
    fn contains(&self, _: V) -> bool { true }
}

impl<V: PartialOrd> Contains<bounds::Open<V>, bounds::Open<V>> for Open<V> {
    fn contains(&self, val: V) -> bool {
        val > self.left.0 && val < self.right.0
    }
}

impl<V: PartialOrd> Contains<bounds::Open<V>, bounds::NoBound<V>> for LeftOpen<V> {
    fn contains(&self, val: V) -> bool {
        val > self.left.0
    }
}

impl<V: PartialOrd> Contains<bounds::NoBound<V>, bounds::Open<V>> for RightOpen<V> {
    fn contains(&self, val: V) -> bool {
        val < self.right.0
    }
}

impl<V: PartialOrd> Contains<bounds::Closed<V>, bounds::Closed<V>> for Closed<V> {
    fn contains(&self, val: V) -> bool {
        val >= self.left.0 && val <= self.right.0
    }
}

impl<V: PartialOrd> Contains<bounds::Closed<V>, bounds::NoBound<V>> for LeftClosed<V> {
    fn contains(&self, val: V) -> bool {
        val >= self.left.0
    }
}

impl<V: PartialOrd> Contains<bounds::NoBound<V>, bounds::Closed<V>> for Closed<V> {
    fn contains(&self, val: V) -> bool {
        val <= self.right.0
    }
}

impl<V: PartialOrd> Contains<bounds::Closed<V>, bounds::Open<V>> for LCRO<V> {
    fn contains(&self, val: V) -> bool {
        val >= self.left.0 && val < self.right.0
    }
}

impl<V: PartialOrd> Contains<bounds::Open<V>, bounds::Closed<V>> for LORC<V> {
    fn contains(&self, val: V) -> bool {
        val > self.left.0 && val <= self.right.0
    }
}

impl<V: PartialOrd> Contains<bounds::NoBound<V>, bounds::OpenOrClosed<V>> for Interval<
    bounds::NoBound<V>, bounds::OpenOrClosed<V>
> {
    fn contains(&self, val: V) -> bool {
        match self.right {
            bounds::OpenOrClosed::Open(ref r) => val < *r,
            bounds::OpenOrClosed::Closed(ref r) => val <= *r,
        }
    }
}

impl<V: PartialOrd> Contains<bounds::Open<V>, bounds::OpenOrClosed<V>> for Interval<
    bounds::Open<V>, bounds::OpenOrClosed<V>
> {
    fn contains(&self, val: V) -> bool {
        val > self.left.0 && match &self.right {
            bounds::OpenOrClosed::Open(ref r) => val > *r,
            bounds::OpenOrClosed::Closed(ref r) => val <= *r,
        }
    }
}

impl<V: PartialOrd> Contains<bounds::Closed<V>, bounds::OpenOrClosed<V>> for Interval<
    bounds::Closed<V>, bounds::OpenOrClosed<V>
> {
    fn contains(&self, val: V) -> bool {
        val >= self.left.0 && match &self.right {
            bounds::OpenOrClosed::Open(ref r) => val > *r,
            bounds::OpenOrClosed::Closed(ref r) => val <= *r,
        }
    }
}

impl<V: PartialOrd> Contains<bounds::OpenOrClosed<V>, bounds::NoBound<V>> for Interval<
    bounds::OpenOrClosed<V>, bounds::NoBound<V>
> {
    fn contains(&self, val: V) -> bool {
        match self.left {
            bounds::OpenOrClosed::Open(ref l) => val > *l,
            bounds::OpenOrClosed::Closed(ref l) => val >= *l,
        }
    }
}

impl<V: PartialOrd> Contains<bounds::OpenOrClosed<V>, bounds::Open<V>> for Interval<
    bounds::OpenOrClosed<V>, bounds::Open<V>
> {
    fn contains(&self, val: V) -> bool {
        val < self.right.0 && match self.left {
            bounds::OpenOrClosed::Open(ref l) => val > *l,
            bounds::OpenOrClosed::Closed(ref l) => val >= *l,
        }
    }
}

impl<V: PartialOrd> Contains<bounds::OpenOrClosed<V>, bounds::Closed<V>> for Interval<
    bounds::OpenOrClosed<V>, bounds::Closed<V>
> {
    fn contains(&self, val: V) -> bool {
        val <= self.right.0 && match self.left {
            bounds::OpenOrClosed::Open(ref l) => val > *l,
            bounds::OpenOrClosed::Closed(ref l) => val >= *l,
        }
    }
}

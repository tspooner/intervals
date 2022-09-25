//! Module containing interval partition utilities.
use crate::{Interval, bounds};

#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum PartitionError<B> {
    IllFormedBounds(B),
}

impl<V: std::fmt::Debug + std::fmt::Display> std::fmt::Display for PartitionError<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartitionError::IllFormedBounds(bounds) => write!(
                f, "The bounds {} are not well defined.", bounds
            ),
        }
    }
}

/// Type representing a single subinterval of a partition.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct SubInterval<V> {
    /// The index of the subinterval.
    pub index: usize,

    /// The interval corresponding to this subinterval.
    pub interval: Interval<bounds::Closed<V>, bounds::OpenOrClosed<V>>,
}

impl<V: Clone> SubInterval<V> {
    pub fn width(&self) -> V::Output
    where
        V: std::ops::Sub,
    {
        let right = match self.interval.right.clone() {
            bounds::OpenOrClosed::Open(right) => right,
            bounds::OpenOrClosed::Closed(right) => right,
        };

        right - self.interval.left.0.clone()
    }

    pub fn midpoint(&self) -> V
    where
        V: std::ops::Add<Output = V> + std::ops::Div<Output = V> + num_traits::One,
    {
        let two = V::one() + V::one();
        let right = match self.interval.right.clone() {
            bounds::OpenOrClosed::Open(right) => right,
            bounds::OpenOrClosed::Closed(right) => right,
        };

        (self.interval.left.0.clone() + right) / two
    }
}

/// Trait for types that represent a partitioning over an interval.
pub trait Partition {
    /// The type associated with the overarching interval.
    type Value;

    /// Return the number of subintervals in the partition.
    fn len(&self) -> usize;

    /// Compute the index of the subinterval associated with the given value.
    fn index(&self, value: &Self::Value) -> Option<usize>;

    /// Return the kth subinterval of the partition.
    fn subinterval(&self, k: usize) -> Option<SubInterval<Self::Value>>;

    /// Return the subinterval to which the given value belongs.
    ///
    /// Note: the corresponding subintervals are taken to be closed on the left
    /// and open on the right.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate intervals;
    /// # use intervals::{bounds::*, partitions::*};
    /// let partition = Declarative::new_unchecked([0, 5, 10]);
    ///
    /// assert_eq!(partition.digitise(&1).unwrap().index, 0);
    /// assert_eq!(partition.digitise(&3).unwrap().index, 0);
    /// assert_eq!(partition.digitise(&6).unwrap().index, 1);
    /// assert_eq!(partition.digitise(&9).unwrap().index, 1);
    /// assert_eq!(partition.digitise(&10).unwrap().index, 1);
    /// ```
    fn digitise(&self, value: &Self::Value) -> Option<SubInterval<Self::Value>> {
        self.index(value).and_then(|k| self.subinterval(k))
    }
}

mod declarative;
pub use self::declarative::Declarative;

mod uniform;
pub use self::uniform::Uniform;

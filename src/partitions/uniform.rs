use crate::bounds;
use num_traits::{Num, NumCast};
use super::{Partition, SubInterval};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
/// Type representing a uniform partitioning of a closed interval.
///
/// # Examples
/// ```
/// # use intervals::partitions::{Partition, Uniform};
/// let partition = Uniform {
///     size: 5,
///     left: 0.0,
///     right: 1.0,
/// };
///
/// assert_eq!(partition.index(&0.2), Some(1));
/// assert_eq!(partition.index(&0.7), Some(3));
/// ```
pub struct Uniform<V> {
    /// The number of partitions in the partitioning.
    pub size: usize,

    /// The left side of the interval.
    pub left: V,

    /// The right side of the interval.
    pub right: V,
}

impl<V: Clone + Num + NumCast> Uniform<V> {
    pub fn partition_width(&self) -> V {
        let range = self.right.clone() - self.left.clone();

        range / NumCast::from(self.size).unwrap()
    }
}

impl<V: Clone + PartialOrd + Num + NumCast> Partition for Uniform<V> {
    type Value = V;

    fn len(&self) -> usize { self.size }

    fn index(&self, value: &V) -> Option<usize> {
        let value = value.clone();

        if value < self.left || value > self.right {
            return None
        }

        if value == self.right {
            return Some(self.size - 1)
        }

        let diff = value - self.left.clone();
        let width = self.partition_width();

        NumCast::from(diff / width.clone())
    }

    fn subinterval(&self, k: usize) -> Option<SubInterval<V>> {
        if k < self.size {
            let width = self.partition_width();

            Some(SubInterval {
                index: k,
                interval: crate::Interval {
                    left: bounds::Closed(self.left.clone()),
                    right: if k == self.size - 1 {
                        bounds::OpenOrClosed::Closed(self.left.clone() + width)
                    } else {
                        bounds::OpenOrClosed::Open(self.left.clone() + width)
                    },
                },
            })
        } else {
            None
        }
    }
}

impl<V: std::fmt::Display> std::fmt::Display for Uniform<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.size {
            1 => write!(f, "{{{} = x0, x1 = {}}}", self.left, self.right),
            2 => write!(f, "{{{} = x0, x1, x2 = {}}}", self.left, self.right),
            _ => write!(f, "{{{} = x0, x1, ..., x{} = {}}}", self.left, self.size, self.right),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let d = Uniform {
            size: 5,
            left: 0.0f64,
            right: 5.0f64,
        };

        assert!(d.index(&-1.0).is_none());
        assert!(d.index(&6.0).is_none());

        assert_eq!(d.index(&0.0).unwrap(), 0);
        assert_eq!(d.index(&1.0).unwrap(), 1);
        assert_eq!(d.index(&2.0).unwrap(), 2);
        assert_eq!(d.index(&3.0).unwrap(), 3);
        assert_eq!(d.index(&4.0).unwrap(), 4);
        assert_eq!(d.index(&5.0).unwrap(), 4);
    }
}

use crate::bounds;
use std::cmp::Ordering;
use super::{Partition, SubInterval, PartitionError};

/// Type representing an explicitly defined partition of an interval.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Declarative<const N: usize, V>([V; N]);

impl<const N: usize, V: PartialOrd> Declarative<N, V> {
    pub fn new(bounds: [V; N]) -> Result<Self, PartitionError<[V; N]>> {
        if bounds.windows(2).all(|w| w[0] <= w[1]) {
            Ok(Declarative(bounds))
        } else {
            Err(PartitionError::IllFormedBounds(bounds))
        }
    }

    pub fn new_unchecked(bounds: [V; N]) -> Self { Declarative(bounds) }

    pub fn iter(&self) -> std::slice::Iter<V> { self.0.iter() }
}

impl<const N: usize, V: PartialOrd + Clone> Partition for Declarative<N, V> {
    type Value = V;

    fn len(&self) -> usize { N - 2 }

    fn index(&self, value: &V) -> Option<usize> {
        if value == &self.0[N - 1] {
            Some(N - 2)
        } else {
            binary_search(&self.0, value)
        }
    }

    fn subinterval(&self, k: usize) -> Option<SubInterval<V>> {
        Some(SubInterval {
            index: k,
            interval: crate::Interval {
                left: bounds::Closed(self.0[k].clone()),
                right: if k == N - 1 {
                    bounds::OpenOrClosed::Closed(self.0[k + 1].clone())
                } else {
                    bounds::OpenOrClosed::Open(self.0[k + 1].clone())
                },
            }
        })
    }
}

impl<const N: usize, V> std::ops::Index<usize> for Declarative<N, V> {
    type Output = V;

    fn index(&self, idx: usize) -> &V { self.0.index(idx) }
}

impl<const N: usize, V: std::fmt::Display> std::fmt::Display for Declarative<N, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let l = &self.0[0];
        let r = &self.0[self.0.len() - 1];

        match N {
            1 => write!(f, "{{{} = x0, x1 = {}}}", l, r),
            2 => write!(f, "{{{} = x0, x1, x2 = {}}}", l, r),
            _ => write!(f, "{{{} = x0, x1, ..., x{} = {}}}", l, N - 1, r),
        }
    }
}

fn binary_search<'a, const N: usize, V: PartialOrd>(
    bounds: &'a [V; N],
    value: &V
) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = N - 1;

    while low < high {
        let middle = (low + high) / 2;

        let l = bounds[middle].partial_cmp(value);
        let r = bounds[middle + 1].partial_cmp(value);

        if let Some((l, r)) = l.zip(r) {
            match l {
                Ordering::Less | Ordering::Equal => {
                    match r {
                        Ordering::Greater => { return Some(middle) },
                        Ordering::Equal => { return Some(middle + 1) },
                        _ => { low = middle; }
                    }
                },
                _ => { high = middle; }
            }
        } else {
            return None
        }
    }

    None
}

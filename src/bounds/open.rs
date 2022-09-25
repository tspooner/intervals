use super::*;

/// Type representing an open bound.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct Open<V>(pub V);

impl<V> crate::private::Sealed for Open<V> {}

impl<V> Bound for Open<V> {
    type Value = V;
    type WithLimit = Closed<V>;

    fn value(&self) -> Option<&Self::Value> { Some(&self.0) }

    fn is_open(&self) -> bool { true }

    fn is_closed(&self) -> bool { false }

    fn with_limit_point(self) -> Self::WithLimit { Closed(self.0) }
}

impl<V> ProperBound for Open<V> {
    fn proper_value(&self) -> &Self::Value { &self.0 }
}

impl<V: fmt::Display> BoundDisplay for Open<V> {
    fn fmt_left(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}", self.0)
    }

    fn fmt_right(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{})", self.0)
    }
}

impl<V> Pinch<NoBound<V>> for Open<V> {
    type Up = Open<V>;
    type Down = Open<V>;

    fn pinch_up(self, _: NoBound<V>) -> Open<V> { self }

    fn pinch_down(self, _: NoBound<V>) -> Open<V> { self }
}

impl<V: PartialOrd> Pinch<Open<V>> for Open<V> {
    type Up = Open<V>;
    type Down = Open<V>;

    fn pinch_up(self, other: Open<V>) -> Open<V> {
        if self.0 >= other.0 { Open(self.0) } else { Open(other.0) }
    }

    fn pinch_down(self, other: Open<V>) -> Open<V> {
        if self.0 >= other.0 { Open(other.0) } else { Open(self.0) }
    }
}

impl<V: PartialOrd> Pinch<Closed<V>> for Open<V> {
    type Up = OpenOrClosed<V>;
    type Down = OpenOrClosed<V>;

    fn pinch_up(self, other: Closed<V>) -> OpenOrClosed<V> {
        if self.0 >= other.0 {
            OpenOrClosed::Open(self.0)
        } else {
            OpenOrClosed::Closed(other.0)
        }
    }

    fn pinch_down(self, other: Closed<V>) -> OpenOrClosed<V> {
        if self.0 <= other.0 {
            OpenOrClosed::Open(self.0)
        } else {
            OpenOrClosed::Closed(other.0)
        }
    }
}

impl<V: PartialOrd> Pinch<OpenOrClosed<V>> for Open<V> {
    type Up = OpenOrClosed<V>;
    type Down = OpenOrClosed<V>;

    fn pinch_up(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> {
        match other {
            OpenOrClosed::Open(v) => OpenOrClosed::Open(
                self.pinch_up(Open(v)).0
            ),
            OpenOrClosed::Closed(v) => self.pinch_up(Closed(v)),
        }
    }

    fn pinch_down(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> {
        match other {
            OpenOrClosed::Open(v) => OpenOrClosed::Open(
                self.pinch_down(Open(v)).0
            ),
            OpenOrClosed::Closed(v) => self.pinch_down(Closed(v)),
        }
    }
}


use super::*;

/// Type representing a closed bound.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct Closed<V>(pub V);

impl<V> crate::private::Sealed for Closed<V> {}

impl<V> Bound for Closed<V> {
    type Value = V;
    type WithLimit = Closed<V>;

    fn value(&self) -> Option<&Self::Value> { Some(&self.0) }

    fn is_open(&self) -> bool { false }

    fn is_closed(&self) -> bool { true }

    fn with_limit_point(self) -> Self::WithLimit { self }
}

impl<V> ProperBound for Closed<V> {
    fn proper_value(&self) -> &Self::Value { &self.0 }
}

impl<V: fmt::Display> BoundDisplay for Closed<V> {
    fn fmt_left(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}", self.0)
    }

    fn fmt_right(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}]", self.0)
    }
}

impl<V> Pinch<NoBound<V>> for Closed<V> {
    type Up = Closed<V>;
    type Down = Closed<V>;

    fn pinch_up(self, _: NoBound<V>) -> Closed<V> { self }

    fn pinch_down(self, _: NoBound<V>) -> Closed<V> { self }
}

impl<V: PartialOrd> Pinch<Open<V>> for Closed<V> {
    type Up = OpenOrClosed<V>;
    type Down = OpenOrClosed<V>;

    fn pinch_up(self, other: Open<V>) -> OpenOrClosed<V> {
        if other.0 >= self.0 {
            OpenOrClosed::Open(other.0)
        } else {
            OpenOrClosed::Closed(self.0)
        }
    }

    fn pinch_down(self, other: Open<V>) -> OpenOrClosed<V> {
        if other.0 <= self.0 {
            OpenOrClosed::Open(other.0)
        } else {
            OpenOrClosed::Closed(self.0)
        }
    }
}

impl<V: PartialOrd> Pinch<Closed<V>> for Closed<V> {
    type Up = Closed<V>;
    type Down = Closed<V>;

    fn pinch_up(self, other: Closed<V>) -> Closed<V> {
        if self.0 >= other.0 { Closed(self.0) } else { Closed(other.0) }
    }

    fn pinch_down(self, other: Closed<V>) -> Closed<V> {
        if self.0 >= other.0 { Closed(other.0) } else { Closed(self.0) }
    }
}

impl<V: PartialOrd> Pinch<OpenOrClosed<V>> for Closed<V> {
    type Up = OpenOrClosed<V>;
    type Down = OpenOrClosed<V>;

    fn pinch_up(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> {
        match other {
            OpenOrClosed::Closed(v) => OpenOrClosed::Closed(
                self.pinch_up(Closed(v)).0
            ),
            OpenOrClosed::Open(v) => self.pinch_up(Open(v)),
        }
    }

    fn pinch_down(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> {
        match other {
            OpenOrClosed::Closed(v) => OpenOrClosed::Closed(
                self.pinch_down(Closed(v)).0
            ),
            OpenOrClosed::Open(v) => self.pinch_down(Open(v)),
        }
    }
}


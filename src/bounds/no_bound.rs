use super::*;

/// Type representing the absence of a bound.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NoBound<V>(pub std::marker::PhantomData<V>);

impl<V> NoBound<V> {
    pub fn new() -> Self { NoBound(std::marker::PhantomData) }
}

impl<V> crate::private::Sealed for NoBound<V> {}

impl<V> Bound for NoBound<V> {
    type Value = V;
    type WithLimit = NoBound<V>;

    fn value(&self) -> Option<&Self::Value> { None }

    fn is_open(&self) -> bool { false }

    fn is_closed(&self) -> bool { false }

    fn with_limit_point(self) -> Self::WithLimit { self }
}

impl<V> BoundDisplay for NoBound<V> {
    fn fmt_left(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(\u{221E}")
    }

    fn fmt_right(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{221E})")
    }
}

impl<V> Pinch<NoBound<V>> for NoBound<V> {
    type Up = NoBound<V>;
    type Down = NoBound<V>;

    fn pinch_up(self, _: NoBound<V>) -> NoBound<V> { self }

    fn pinch_down(self, _: NoBound<V>) -> NoBound<V> { self }
}

impl<V> Pinch<Open<V>> for NoBound<V> {
    type Up = Open<V>;
    type Down = Open<V>;

    fn pinch_up(self, other: Open<V>) -> Open<V> { other }

    fn pinch_down(self, other: Open<V>) -> Open<V> { other }
}

impl<V> Pinch<Closed<V>> for NoBound<V> {
    type Up = Closed<V>;
    type Down = Closed<V>;

    fn pinch_up(self, other: Closed<V>) -> Closed<V> { other }

    fn pinch_down(self, other: Closed<V>) -> Closed<V> { other }
}

impl<V> Pinch<OpenOrClosed<V>> for NoBound<V> {
    type Up = OpenOrClosed<V>;
    type Down = OpenOrClosed<V>;

    fn pinch_up(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> { other }

    fn pinch_down(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> { other }
}

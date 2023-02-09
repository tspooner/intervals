use super::*;

/// Type representing the absence of a bound.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct NoBound<V>(pub std::marker::PhantomData<V>);

impl<V> NoBound<V> {
    pub fn new() -> Self { NoBound(std::marker::PhantomData) }
}

impl<V> crate::private::Sealed for NoBound<V> {}

impl<V: PartialOrd> Bound for NoBound<V> {
    type Value = V;
    type WithLimit = NoBound<V>;

    fn value(&self) -> Option<&Self::Value> { None }

    fn is_open(&self) -> bool { false }

    fn is_closed(&self) -> bool { false }

    fn with_limit_point(self) -> Self::WithLimit { self }
}

impl<V: PartialOrd> BoundDisplay for NoBound<V> {
    fn fmt_left(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(\u{221E}")
    }

    fn fmt_right(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{221E})")
    }
}

impl<V: PartialOrd> Pinch<NoBound<V>> for NoBound<V> {
    type Left = NoBound<V>;
    type Right = NoBound<V>;

    fn pinch_left(self, _: NoBound<V>) -> NoBound<V> { self }

    fn pinch_right(self, _: NoBound<V>) -> NoBound<V> { self }
}

impl<V: PartialOrd> Pinch<Open<V>> for NoBound<V> {
    type Left = Open<V>;
    type Right = Open<V>;

    fn pinch_left(self, other: Open<V>) -> Open<V> { other }

    fn pinch_right(self, other: Open<V>) -> Open<V> { other }
}

impl<V: PartialOrd> Pinch<Closed<V>> for NoBound<V> {
    type Left = Closed<V>;
    type Right = Closed<V>;

    fn pinch_left(self, other: Closed<V>) -> Closed<V> { other }

    fn pinch_right(self, other: Closed<V>) -> Closed<V> { other }
}

impl<V: PartialOrd> Pinch<OpenOrClosed<V>> for NoBound<V> {
    type Left = OpenOrClosed<V>;
    type Right = OpenOrClosed<V>;

    fn pinch_left(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> { other }

    fn pinch_right(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> { other }
}

impl<V> std::cmp::PartialEq<Open<V>> for NoBound<V> {
    fn eq(&self, _: &Open<V>) -> bool { false }
}

impl<V> std::cmp::PartialEq<Closed<V>> for NoBound<V> {
    fn eq(&self, _: &Closed<V>) -> bool { false }
}

impl<V> std::cmp::PartialEq<OpenOrClosed<V>> for NoBound<V> {
    fn eq(&self, _: &OpenOrClosed<V>) -> bool { false }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_properties() {
        let a: NoBound<f64> = NoBound::new();

        assert!(!a.is_open());
        assert!(!a.is_closed());

        assert!(a.value().is_none());
        assert_eq!(a.with_limit_point(), a);
    }

    #[test]
    fn test_pinch() {
        let a = NoBound::new();

        assert_eq!(a.pinch_left(a), a);
        assert_eq!(a.pinch_right(a), a);

        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            assert_eq!(a.pinch_left(Open(x)), Open(x));
            assert_eq!(a.pinch_right(Open(x)), Open(x));

            assert_eq!(a.pinch_left(OpenOrClosed::Open(x)), Open(x));
            assert_eq!(a.pinch_right(OpenOrClosed::Open(x)), Open(x));

            assert_eq!(a.pinch_left(Closed(x)), Closed(x));
            assert_eq!(a.pinch_right(Closed(x)), Closed(x));

            assert_eq!(a.pinch_left(OpenOrClosed::Closed(x)), Closed(x));
            assert_eq!(a.pinch_right(OpenOrClosed::Closed(x)), Closed(x));
        }
    }
}

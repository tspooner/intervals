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

impl<V: PartialOrd> Bound for Closed<V> {
    type Value = V;
    type WithLimit = Closed<V>;

    fn value(&self) -> Option<&Self::Value> { Some(&self.0) }

    fn is_open(&self) -> bool { false }

    fn is_closed(&self) -> bool { true }

    fn with_limit_point(self) -> Self::WithLimit { self }
}

impl<V: PartialOrd> ProperBound for Closed<V> {
    fn proper_value(&self) -> &Self::Value { &self.0 }
}

impl<V: PartialOrd + fmt::Display> BoundDisplay for Closed<V> {
    fn fmt_left(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}", self.0)
    }

    fn fmt_right(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}]", self.0)
    }
}

impl<V: PartialOrd> Pinch<NoBound<V>> for Closed<V> {
    type Left = Closed<V>;
    type Right = Closed<V>;

    fn pinch_left(self, _: NoBound<V>) -> Closed<V> { self }

    fn pinch_right(self, _: NoBound<V>) -> Closed<V> { self }
}

impl<V: PartialOrd> Pinch<Open<V>> for Closed<V> {
    type Left = OpenOrClosed<V>;
    type Right = OpenOrClosed<V>;

    fn pinch_left(self, other: Open<V>) -> OpenOrClosed<V> {
        if self.0 > other.0 {
            OpenOrClosed::Closed(self.0)
        } else {
            OpenOrClosed::Open(other.0)
        }
    }

    fn pinch_right(self, other: Open<V>) -> OpenOrClosed<V> {
        if self.0 < other.0 {
            OpenOrClosed::Closed(self.0)
        } else {
            OpenOrClosed::Open(other.0)
        }
    }
}

impl<V: PartialOrd> Pinch<Closed<V>> for Closed<V> {
    type Left = Closed<V>;
    type Right = Closed<V>;

    fn pinch_left(self, other: Closed<V>) -> Closed<V> {
        if self.0 >= other.0 { Closed(self.0) } else { Closed(other.0) }
    }

    fn pinch_right(self, other: Closed<V>) -> Closed<V> {
        if self.0 <= other.0 { Closed(self.0) } else { Closed(other.0) }
    }
}

impl<V: PartialOrd> Pinch<OpenOrClosed<V>> for Closed<V> {
    type Left = OpenOrClosed<V>;
    type Right = OpenOrClosed<V>;

    fn pinch_left(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> {
        match other {
            OpenOrClosed::Closed(v) => OpenOrClosed::Closed(
                self.pinch_left(Closed(v)).0
            ),
            OpenOrClosed::Open(v) => self.pinch_left(Open(v)),
        }
    }

    fn pinch_right(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> {
        match other {
            OpenOrClosed::Closed(v) => OpenOrClosed::Closed(
                self.pinch_right(Closed(v)).0
            ),
            OpenOrClosed::Open(v) => self.pinch_right(Open(v)),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_properties() {
        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            let a = Closed(x);

            assert!(!a.is_open());
            assert!(a.is_closed());

            assert_eq!(a.proper_value(), &x);
            assert_eq!(a.value().unwrap(), &x);
            assert_eq!(a.with_limit_point(), a);
        }
    }

    #[test]
    fn test_pinch_nobound() {
        let a = Closed(0.0f64);

        assert_eq!(a.pinch_left(NoBound::new()), a);
        assert_eq!(a.pinch_right(NoBound::new()), a);
    }

    #[test]
    fn test_pinch_closed() {
        let a = Closed(0.0f64);

        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            assert_eq!(a.pinch_left(Closed(x)), Closed(x.max(0.0)));
            assert_eq!(a.pinch_right(Closed(x)), Closed(x.min(0.0)));

            assert_eq!(a.pinch_left(OpenOrClosed::Closed(x)), Closed(x.max(0.0)));
            assert_eq!(a.pinch_right(OpenOrClosed::Closed(x)), Closed(x.min(0.0)));
        }
    }

    #[test]
    fn test_pinch_open() {
        let a = Closed(0.0f64);

        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            if x < 0.0 {
                assert_eq!(a.pinch_left(Open(x)), Closed(0.0));
                assert_eq!(a.pinch_left(OpenOrClosed::Open(x)), Closed(0.0));
            } else {
                assert_eq!(a.pinch_left(Open(x)), Open(x));
                assert_eq!(a.pinch_left(OpenOrClosed::Open(x)), Open(x));
            }

            if x > 0.0 {
                assert_eq!(a.pinch_right(Open(x)), Closed(0.0));
                assert_eq!(a.pinch_right(OpenOrClosed::Open(x)), Closed(0.0));
            } else {
                assert_eq!(a.pinch_right(Open(x)), Open(x));
                assert_eq!(a.pinch_right(OpenOrClosed::Open(x)), Open(x));
            }
        }
    }
}

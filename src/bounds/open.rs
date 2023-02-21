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

// Core:
impl<V: PartialOrd> Bound for Open<V> {
    type Value = V;
    type WithLimit = Closed<V>;

    fn value(&self) -> Option<&Self::Value> { Some(&self.0) }

    fn is_open(&self) -> bool { true }

    fn is_closed(&self) -> bool { false }

    fn with_limit_point(self) -> Self::WithLimit { Closed(self.0) }
}

impl<V: PartialOrd> ProperBound for Open<V> {
    fn proper_value(&self) -> &Self::Value { &self.0 }
}

// Formatting:
impl<V: PartialOrd + fmt::Display> BoundDisplay for Open<V> {
    fn fmt_left(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}", self.0)
    }

    fn fmt_right(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{})", self.0)
    }
}

// Pinch:
impl<V: PartialOrd> Pinch<Open<V>> for Open<V> {
    type Left = Open<V>;
    type Right = Open<V>;

    fn pinch_left(self, other: Open<V>) -> Open<V> {
        if self.0 >= other.0 { self } else { other }
    }

    fn pinch_right(self, other: Open<V>) -> Open<V> {
        if self.0 <= other.0 { self } else { other }
    }
}

impl<V: PartialOrd> Pinch<Closed<V>> for Open<V> {
    type Left = OpenOrClosed<V>;
    type Right = OpenOrClosed<V>;

    fn pinch_left(self, other: Closed<V>) -> OpenOrClosed<V> {
        if self.0 >= other.0 {
            OpenOrClosed::Open(self.0)
        } else {
            OpenOrClosed::Closed(other.0)
        }
    }

    fn pinch_right(self, other: Closed<V>) -> OpenOrClosed<V> {
        if self.0 <= other.0 {
            OpenOrClosed::Open(self.0)
        } else {
            OpenOrClosed::Closed(other.0)
        }
    }
}

// Unroll:
impl<V: PartialOrd> Unroll<Open<V>> for Open<V> {
    type Left = Open<V>;
    type Right = Open<V>;

    fn unroll_left(self, other: Open<V>) -> Open<V> {
        if self.0 <= other.0 { self } else { other }
    }

    fn unroll_right(self, other: Open<V>) -> Open<V> {
        if self.0 >= other.0 { self } else { other }
    }
}

impl<V: PartialOrd> Unroll<Closed<V>> for Open<V> {
    type Left = OpenOrClosed<V>;
    type Right = OpenOrClosed<V>;

    fn unroll_left(self, other: Closed<V>) -> OpenOrClosed<V> {
        if self.0 < other.0 {
            OpenOrClosed::Open(self.0)
        } else {
            OpenOrClosed::Closed(other.0)
        }
    }

    fn unroll_right(self, other: Closed<V>) -> OpenOrClosed<V> {
        if self.0 > other.0 {
            OpenOrClosed::Open(self.0)
        } else {
            OpenOrClosed::Closed(other.0)
        }
    }
}

// Comparison:
impl<V> std::cmp::PartialEq<Closed<V>> for Open<V> {
    fn eq(&self, _: &Closed<V>) -> bool { false }
}

impl<V> std::cmp::PartialEq<NoBound<V>> for Open<V> {
    fn eq(&self, _: &NoBound<V>) -> bool { false }
}

impl<V: PartialEq> std::cmp::PartialEq<OpenOrClosed<V>> for Open<V> {
    fn eq(&self, rhs: &OpenOrClosed<V>) -> bool {
        match rhs {
            &OpenOrClosed::Open(ref inner) => self.0.eq(&inner),
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
            let a = Open(x);

            assert!(a.is_open());
            assert!(!a.is_closed());

            assert_eq!(a.proper_value(), &x);
            assert_eq!(a.value().unwrap(), &x);
            assert_eq!(a.with_limit_point(), Closed(x));
        }
    }

    #[test]
    fn test_pinch_nobound() {
        let a = Open(0.0f64);

        assert_eq!(a.pinch_left(NoBound::new()), a);
        assert_eq!(a.pinch_right(NoBound::new()), a);
    }

    #[test]
    fn test_pinch_open() {
        let a = Open(0.0f64);

        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            assert_eq!(a.pinch_left(Open(x)), Open(x.max(0.0)));
            assert_eq!(a.pinch_right(Open(x)), Open(x.min(0.0)));

            assert_eq!(a.pinch_left(OpenOrClosed::Open(x)), Open(x.max(0.0)));
            assert_eq!(a.pinch_right(OpenOrClosed::Open(x)), Open(x.min(0.0)));
        }
    }

    #[test]
    fn test_pinch_closed() {
        let a = Open(0.0f64);

        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            if x <= 0.0 {
                assert_eq!(a.pinch_left(Closed(x)), Open(0.0));
                assert_eq!(a.pinch_left(OpenOrClosed::Closed(x)), Open(0.0));
            } else {
                assert_eq!(a.pinch_left(Closed(x)), Closed(x));
                assert_eq!(a.pinch_left(OpenOrClosed::Closed(x)), Closed(x));
            }

            if x >= 0.0 {
                assert_eq!(a.pinch_right(Closed(x)), Open(0.0));
                assert_eq!(a.pinch_right(OpenOrClosed::Closed(x)), Open(0.0));
            } else {
                assert_eq!(a.pinch_right(Closed(x)), Closed(x));
                assert_eq!(a.pinch_right(OpenOrClosed::Closed(x)), Closed(x));
            }
        }
    }

    #[test]
    fn test_unroll_nobound() {
        let a = Open(0.0f64);

        assert_eq!(a.unroll_left(NoBound::new()), NoBound::new());
        assert_eq!(a.unroll_right(NoBound::new()), NoBound::new());
    }

    #[test]
    fn test_unroll_open() {
        let a = Open(0.0f64);

        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            assert_eq!(a.unroll_left(Open(x)), Open(x.min(0.0)));
            assert_eq!(a.unroll_right(Open(x)), Open(x.max(0.0)));

            assert_eq!(a.unroll_left(OpenOrClosed::Open(x)), Open(x.min(0.0)));
            assert_eq!(a.unroll_right(OpenOrClosed::Open(x)), Open(x.max(0.0)));
        }
    }

    #[test]
    fn test_unroll_closed() {
        let a = Open(0.0f64);

        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            if x <= 0.0 {
                assert_eq!(a.unroll_left(Closed(x)), Closed(x));
                assert_eq!(a.unroll_left(OpenOrClosed::Closed(x)), Closed(x));
            } else {
                assert_eq!(a.unroll_left(Closed(x)), Open(0.0));
                assert_eq!(a.unroll_left(OpenOrClosed::Closed(x)), Open(0.0));
            }

            if x >= 0.0 {
                assert_eq!(a.unroll_right(Closed(x)), Closed(x));
                assert_eq!(a.unroll_right(OpenOrClosed::Closed(x)), Closed(x));
            } else {
                assert_eq!(a.unroll_right(Closed(x)), Open(0.0));
                assert_eq!(a.unroll_right(OpenOrClosed::Closed(x)), Open(0.0));
            }
        }
    }
}

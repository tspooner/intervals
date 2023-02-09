use super::*;

/// Union type representing a bound that is either open or closed.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum OpenOrClosed<V> {
    /// The open bound variant.
    Open(V),

    /// The closed bound variant.
    Closed(V),
}

impl<V> OpenOrClosed<V> {
    pub fn unwrap(self) -> V {
        match self {
            OpenOrClosed::Open(x) | OpenOrClosed::Closed(x) => x,
        }
    }
}

impl<V> crate::private::Sealed for OpenOrClosed<V> {}

impl<V: PartialOrd> Bound for OpenOrClosed<V> {
    type Value = V;
    type WithLimit = Closed<V>;

    fn value(&self) -> Option<&Self::Value> {
        match self {
            OpenOrClosed::Open(ref v) | OpenOrClosed::Closed(ref v) => Some(v),
        }
    }

    fn is_open(&self) -> bool {
        match self {
            OpenOrClosed::Open(_) => true,
            OpenOrClosed::Closed(_) => false,
        }
    }

    fn is_closed(&self) -> bool {
        match self {
            OpenOrClosed::Open(_) => false,
            OpenOrClosed::Closed(_) => true,
        }
    }

    fn with_limit_point(self) -> Self::WithLimit {
        match self {
            OpenOrClosed::Open(v) | OpenOrClosed::Closed(v) => Closed(v),
        }
    }
}

impl<V: PartialOrd> ProperBound for OpenOrClosed<V> {
    fn proper_value(&self) -> &Self::Value {
        match self {
            OpenOrClosed::Open(ref v) | OpenOrClosed::Closed(ref v) => v,
        }
    }
}

impl<V: PartialOrd + fmt::Display> BoundDisplay for OpenOrClosed<V> {
    fn fmt_left(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpenOrClosed::Open(v) => Open(v).fmt_left(f),
            OpenOrClosed::Closed(v) => Closed(v).fmt_left(f),
        }
    }

    fn fmt_right(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpenOrClosed::Open(v) => Open(v).fmt_right(f),
            OpenOrClosed::Closed(v) => Closed(v).fmt_right(f),
        }
    }
}

impl<V: PartialOrd> Pinch<NoBound<V>> for OpenOrClosed<V> {
    type Left = OpenOrClosed<V>;
    type Right = OpenOrClosed<V>;

    fn pinch_left(self, _: NoBound<V>) -> OpenOrClosed<V> { self }

    fn pinch_right(self, _: NoBound<V>) -> OpenOrClosed<V> { self }
}

impl<V: PartialOrd> Pinch<Open<V>> for OpenOrClosed<V> {
    type Left = OpenOrClosed<V>;
    type Right = OpenOrClosed<V>;

    fn pinch_left(self, other: Open<V>) -> OpenOrClosed<V> {
        other.pinch_left(self)
    }

    fn pinch_right(self, other: Open<V>) -> OpenOrClosed<V> {
        other.pinch_right(self)
    }
}

impl<V: PartialOrd> Pinch<Closed<V>> for OpenOrClosed<V> {
    type Left = OpenOrClosed<V>;
    type Right = OpenOrClosed<V>;

    fn pinch_left(self, other: Closed<V>) -> OpenOrClosed<V> {
        other.pinch_left(self)
    }

    fn pinch_right(self, other: Closed<V>) -> OpenOrClosed<V> {
        other.pinch_right(self)
    }
}

impl<V: PartialOrd> Pinch<OpenOrClosed<V>> for OpenOrClosed<V> {
    type Left = OpenOrClosed<V>;
    type Right = OpenOrClosed<V>;

    fn pinch_left(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> {
        match (self, other) {
            (OpenOrClosed::Open(x), OpenOrClosed::Open(y)) => {
                OpenOrClosed::Open(Open(x).pinch_left(Open(y)).0)
            },
            (OpenOrClosed::Open(x), OpenOrClosed::Closed(y)) => {
                Open(x).pinch_left(Closed(y))
            },
            (OpenOrClosed::Closed(x), OpenOrClosed::Open(y)) => {
                Closed(x).pinch_left(Open(y))
            },
            (OpenOrClosed::Closed(x), OpenOrClosed::Closed(y)) => {
                OpenOrClosed::Closed(Closed(x).pinch_left(Closed(y)).0)
            },
        }
    }

    fn pinch_right(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> {
        match (self, other) {
            (OpenOrClosed::Open(x), OpenOrClosed::Open(y)) => {
                OpenOrClosed::Open(Open(x).pinch_right(Open(y)).0)
            },
            (OpenOrClosed::Open(x), OpenOrClosed::Closed(y)) => {
                Open(x).pinch_right(Closed(y))
            },
            (OpenOrClosed::Closed(x), OpenOrClosed::Open(y)) => {
                Closed(x).pinch_right(Open(y))
            },
            (OpenOrClosed::Closed(x), OpenOrClosed::Closed(y)) => {
                OpenOrClosed::Closed(Closed(x).pinch_right(Closed(y)).0)
            },
        }
    }
}

impl<V: PartialEq> std::cmp::PartialEq<Open<V>> for OpenOrClosed<V> {
    fn eq(&self, rhs: &Open<V>) -> bool {
        match self {
            &OpenOrClosed::Open(ref inner) => inner.eq(&rhs.0),
            _ => false,
        }
    }
}

impl<V: PartialEq> std::cmp::PartialEq<Closed<V>> for OpenOrClosed<V> {
    fn eq(&self, rhs: &Closed<V>) -> bool {
        match self {
            &OpenOrClosed::Closed(ref inner) => inner.eq(&rhs.0),
            _ => false,
        }
    }
}

impl<V> std::cmp::PartialEq<NoBound<V>> for OpenOrClosed<V> {
    fn eq(&self, _: &NoBound<V>) -> bool { false }
}

#[cfg(test)]
mod tests {
    use super::*;

    // OpenOrClosed::Open
    #[test]
    fn test_open_core_properties() {
        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            let a = OpenOrClosed::Open(x);

            assert!(a.is_open());
            assert!(!a.is_closed());

            assert_eq!(a.proper_value(), &x);
            assert_eq!(a.value().unwrap(), &x);
            assert_eq!(a.with_limit_point(), Closed(x));
        }
    }

    #[test]
    fn test_open_pinch_nobound() {
        let a = OpenOrClosed::Open(0.0f64);

        assert_eq!(a.pinch_left(NoBound::new()), a);
        assert_eq!(a.pinch_right(NoBound::new()), a);
    }

    #[test]
    fn test_open_pinch_open() {
        let a = OpenOrClosed::Open(0.0f64);

        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            assert_eq!(a.pinch_left(Open(x)), Open(x.max(0.0)));
            assert_eq!(a.pinch_right(Open(x)), Open(x.min(0.0)));

            assert_eq!(a.pinch_left(OpenOrClosed::Open(x)), Open(x.max(0.0)));
            assert_eq!(a.pinch_right(OpenOrClosed::Open(x)), Open(x.min(0.0)));
        }
    }

    #[test]
    fn test_open_pinch_closed() {
        let a = OpenOrClosed::Open(0.0f64);

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

    // OpenOrClosed::Closed
    #[test]
    fn test_closed_core_properties() {
        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            let a = OpenOrClosed::Closed(x);

            assert!(!a.is_open());
            assert!(a.is_closed());

            assert_eq!(a.proper_value(), &x);
            assert_eq!(a.value().unwrap(), &x);
            assert_eq!(a.with_limit_point(), a);
        }
    }

    #[test]
    fn test_closed_pinch_nobound() {
        let a = OpenOrClosed::Closed(0.0f64);

        assert_eq!(a.pinch_left(NoBound::new()), a);
        assert_eq!(a.pinch_right(NoBound::new()), a);
    }

    #[test]
    fn test_closed_pinch_closed() {
        let a = OpenOrClosed::Closed(0.0f64);

        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            assert_eq!(a.pinch_left(Closed(x)), Closed(x.max(0.0)));
            assert_eq!(a.pinch_right(Closed(x)), Closed(x.min(0.0)));

            assert_eq!(a.pinch_left(OpenOrClosed::Closed(x)), Closed(x.max(0.0)));
            assert_eq!(a.pinch_right(OpenOrClosed::Closed(x)), Closed(x.min(0.0)));
        }
    }

    #[test]
    fn test_closed_pinch_open() {
        let a = OpenOrClosed::Closed(0.0f64);

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

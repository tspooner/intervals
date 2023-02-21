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

impl<V> From<Open<V>> for OpenOrClosed<V> {
    fn from(bound: Open<V>) -> OpenOrClosed<V> { OpenOrClosed::Open(bound.0) }
}

impl<V> From<Closed<V>> for OpenOrClosed<V> {
    fn from(bound: Closed<V>) -> OpenOrClosed<V> { OpenOrClosed::Closed(bound.0) }
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

// Pinch:
macro_rules! impl_pinch {
    ($v:ident; $other:ty) => {
        impl<$v: PartialOrd> Pinch<$other> for OpenOrClosed<$v> {
            type Left = OpenOrClosed<$v>;
            type Right = OpenOrClosed<$v>;

            fn pinch_left(self, other: $other) -> OpenOrClosed<$v> {
                match self {
                    OpenOrClosed::Open(x) => Open(x).pinch_left(other).into(),
                    OpenOrClosed::Closed(x) => Closed(x).pinch_left(other).into(),
                }
            }

            fn pinch_right(self, other: $other) -> OpenOrClosed<$v> {
                match self {
                    OpenOrClosed::Open(x) => Open(x).pinch_right(other).into(),
                    OpenOrClosed::Closed(x) => Closed(x).pinch_right(other).into(),
                }
            }
        }

        impl<$v: PartialOrd> Pinch<OpenOrClosed<$v>> for $other {
            type Left = OpenOrClosed<$v>;
            type Right = OpenOrClosed<$v>;

            fn pinch_left(self, other: OpenOrClosed<$v>) -> OpenOrClosed<$v> {
                match other {
                    OpenOrClosed::Open(x) => self.pinch_left(Open(x)).into(),
                    OpenOrClosed::Closed(x) => self.pinch_left(Closed(x)).into(),
                }
            }

            fn pinch_right(self, other: OpenOrClosed<$v>) -> OpenOrClosed<$v> {
                match other {
                    OpenOrClosed::Open(x) => self.pinch_right(Open(x)).into(),
                    OpenOrClosed::Closed(x) => self.pinch_right(Closed(x)).into(),
                }
            }
        }
    };
}

impl_pinch!(V; Open<V>);
impl_pinch!(V; Closed<V>);
impl_pinch!(V; NoBound<V>);

impl<V: PartialOrd> Pinch<OpenOrClosed<V>> for OpenOrClosed<V> {
    type Left = OpenOrClosed<V>;
    type Right = OpenOrClosed<V>;

    fn pinch_left(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> {
        match self {
            OpenOrClosed::Open(x) => Open(x).pinch_left(other).into(),
            OpenOrClosed::Closed(x) => Closed(x).pinch_left(other).into(),
        }
    }

    fn pinch_right(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> {
        match self {
            OpenOrClosed::Open(x) => Open(x).pinch_right(other).into(),
            OpenOrClosed::Closed(x) => Closed(x).pinch_right(other).into(),
        }
    }
}

// Unroll:
impl<V: PartialOrd> Unroll<OpenOrClosed<V>> for OpenOrClosed<V> {
    type Left = OpenOrClosed<V>;
    type Right = OpenOrClosed<V>;

    fn unroll_left(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> {
        match self {
            OpenOrClosed::Open(x) => Open(x).unroll_left(other).into(),
            OpenOrClosed::Closed(x) => Closed(x).unroll_left(other).into(),
        }
    }

    fn unroll_right(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> {
        match self {
            OpenOrClosed::Open(x) => Open(x).unroll_right(other).into(),
            OpenOrClosed::Closed(x) => Closed(x).unroll_right(other).into(),
        }
    }
}

macro_rules! impl_unroll {
    ($v:ident; $other:ty) => {
        impl<$v: PartialOrd> Unroll<$other> for OpenOrClosed<$v> {
            type Left = OpenOrClosed<$v>;
            type Right = OpenOrClosed<$v>;

            fn unroll_left(self, other: $other) -> OpenOrClosed<$v> {
                match self {
                    OpenOrClosed::Open(x) => Open(x).unroll_left(other).into(),
                    OpenOrClosed::Closed(x) => Closed(x).unroll_left(other).into(),
                }
            }

            fn unroll_right(self, other: $other) -> OpenOrClosed<$v> {
                match self {
                    OpenOrClosed::Open(x) => Open(x).unroll_right(other).into(),
                    OpenOrClosed::Closed(x) => Closed(x).unroll_right(other).into(),
                }
            }
        }

        impl<$v: PartialOrd> Unroll<OpenOrClosed<$v>> for $other {
            type Left = OpenOrClosed<$v>;
            type Right = OpenOrClosed<$v>;

            fn unroll_left(self, other: OpenOrClosed<$v>) -> OpenOrClosed<$v> {
                match other {
                    OpenOrClosed::Open(x) => self.unroll_left(Open(x)).into(),
                    OpenOrClosed::Closed(x) => self.unroll_left(Closed(x)).into(),
                }
            }

            fn unroll_right(self, other: OpenOrClosed<$v>) -> OpenOrClosed<$v> {
                match other {
                    OpenOrClosed::Open(x) => self.unroll_right(Open(x)).into(),
                    OpenOrClosed::Closed(x) => self.unroll_right(Closed(x)).into(),
                }
            }
        }
    };
}

impl_unroll!(V; Open<V>);
impl_unroll!(V; Closed<V>);

// Comparison:
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

    #[test]
    fn test_open_unroll_nobound() {
        let a = OpenOrClosed::Open(0.0f64);

        assert_eq!(a.unroll_left(NoBound::new()), NoBound::new());
        assert_eq!(a.unroll_right(NoBound::new()), NoBound::new());
    }

    #[test]
    fn test_open_unroll_open() {
        let a = OpenOrClosed::Open(0.0f64);

        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            assert_eq!(a.unroll_left(Open(x)), Open(x.min(0.0)));
            assert_eq!(a.unroll_right(Open(x)), Open(x.max(0.0)));

            assert_eq!(a.unroll_left(OpenOrClosed::Open(x)), Open(x.min(0.0)));
            assert_eq!(a.unroll_right(OpenOrClosed::Open(x)), Open(x.max(0.0)));
        }
    }

    #[test]
    fn test_open_unroll_closed() {
        let a = OpenOrClosed::Open(0.0f64);

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

    #[test]
    fn test_closed_unroll_nobound() {
        let a = OpenOrClosed::Closed(0.0f64);

        assert_eq!(a.unroll_left(NoBound::new()), NoBound::new());
        assert_eq!(a.unroll_right(NoBound::new()), NoBound::new());
    }

    #[test]
    fn test_closed_unroll_closed() {
        let a = OpenOrClosed::Closed(0.0f64);

        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            assert_eq!(a.unroll_left(Closed(x)), Closed(x.min(0.0)));
            assert_eq!(a.unroll_right(Closed(x)), Closed(x.max(0.0)));

            assert_eq!(a.unroll_left(OpenOrClosed::Closed(x)), Closed(x.min(0.0)));
            assert_eq!(a.unroll_right(OpenOrClosed::Closed(x)), Closed(x.max(0.0)));
        }
    }

    #[test]
    fn test_closed_unroll_open() {
        let a = OpenOrClosed::Closed(0.0f64);

        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            if x < 0.0 {
                assert_eq!(a.unroll_left(Open(x)), Open(x));
                assert_eq!(a.unroll_left(OpenOrClosed::Open(x)), Open(x));
            } else {
                assert_eq!(a.unroll_left(Open(x)), Closed(0.0));
                assert_eq!(a.unroll_left(OpenOrClosed::Open(x)), Closed(0.0));
            }

            if x > 0.0 {
                assert_eq!(a.unroll_right(Open(x)), Open(x));
                assert_eq!(a.unroll_right(OpenOrClosed::Open(x)), Open(x));
            } else {
                assert_eq!(a.unroll_right(Open(x)), Closed(0.0));
                assert_eq!(a.unroll_right(OpenOrClosed::Open(x)), Closed(0.0));
            }
        }
    }
}

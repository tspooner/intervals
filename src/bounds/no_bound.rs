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

// Core:
impl<V: PartialOrd> Bound for NoBound<V> {
    type Value = V;
    type WithLimit = NoBound<V>;

    fn value(&self) -> Option<&Self::Value> { None }

    fn is_open(&self) -> bool { false }

    fn is_closed(&self) -> bool { false }

    fn with_limit_point(self) -> Self::WithLimit { self }
}

// Formatting:
impl<V: PartialOrd> BoundDisplay for NoBound<V> {
    fn fmt_left(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(\u{221E}")
    }

    fn fmt_right(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{221E})")
    }
}

// Pinch:
macro_rules! impl_pinch {
    ($v:ident; $l:ty, $r:ty; |$me:ident, $other:ident| -> $out:ty $code:block) => {
        impl<$v: PartialOrd> Pinch<$r> for $l {
            type Left = $out;
            type Right = $out;

            #[allow(unused)]
            fn pinch_left($me, $other: $r) -> Self::Left $code

            #[allow(unused)]
            fn pinch_right($me, $other: $r) -> Self::Right $code
        }
    }
}

impl_pinch!(V; NoBound<V>, NoBound<V>; |self, rhs| -> NoBound<V> { self });

impl_pinch!(V; NoBound<V>, Open<V>; |self, rhs| -> Open<V> { rhs });
impl_pinch!(V; Open<V>, NoBound<V>; |self, rhs| -> Open<V> { self });

impl_pinch!(V; NoBound<V>, Closed<V>; |self, rhs| -> Closed<V> { rhs });
impl_pinch!(V; Closed<V>, NoBound<V>; |self, rhs| -> Closed<V> { self });

// Unroll:
macro_rules! impl_unroll {
    ($v:ident; $l:ty, $r:ty) => {
        impl<$v: PartialOrd> Unroll<$r> for $l {
            type Left = NoBound<$v>;
            type Right = NoBound<$v>;

            #[allow(unused)]
            fn unroll_left(self, _: $r) -> Self::Left { NoBound::new() }

            #[allow(unused)]
            fn unroll_right(self, _: $r) -> Self::Right { NoBound::new() }
        }
    }
}

impl_unroll!(V; NoBound<V>, NoBound<V>);

impl_unroll!(V; NoBound<V>, Open<V>);
impl_unroll!(V; Open<V>, NoBound<V>);

impl_unroll!(V; NoBound<V>, Closed<V>);
impl_unroll!(V; Closed<V>, NoBound<V>);

impl_unroll!(V; NoBound<V>, OpenOrClosed<V>);
impl_unroll!(V; OpenOrClosed<V>, NoBound<V>);

// Comparison:
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

    #[test]
    fn test_unroll() {
        let a = NoBound::new();

        assert_eq!(a.unroll_left(a), a);
        assert_eq!(a.unroll_right(a), a);

        for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            assert_eq!(a.unroll_left(Open(x)), NoBound::new());
            assert_eq!(a.unroll_right(Open(x)), NoBound::new());

            assert_eq!(a.unroll_left(OpenOrClosed::Open(x)), NoBound::new());
            assert_eq!(a.unroll_right(OpenOrClosed::Open(x)), NoBound::new());

            assert_eq!(a.unroll_left(Closed(x)), NoBound::new());
            assert_eq!(a.unroll_right(Closed(x)), NoBound::new());

            assert_eq!(a.unroll_left(OpenOrClosed::Closed(x)), NoBound::new());
            assert_eq!(a.unroll_right(OpenOrClosed::Closed(x)), NoBound::new());
        }
    }
}

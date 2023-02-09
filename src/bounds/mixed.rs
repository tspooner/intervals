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

impl<V> Bound for OpenOrClosed<V> {
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

impl<V> ProperBound for OpenOrClosed<V> {
    fn proper_value(&self) -> &Self::Value {
        match self {
            OpenOrClosed::Open(ref v) | OpenOrClosed::Closed(ref v) => v,
        }
    }
}

impl<V: fmt::Display> BoundDisplay for OpenOrClosed<V> {
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
    type Up = OpenOrClosed<V>;
    type Down = OpenOrClosed<V>;

    fn pinch_up(self, _: NoBound<V>) -> OpenOrClosed<V> { self }

    fn pinch_down(self, _: NoBound<V>) -> OpenOrClosed<V> { self }
}

impl<V: PartialOrd> Pinch<Open<V>> for OpenOrClosed<V> {
    type Up = OpenOrClosed<V>;
    type Down = OpenOrClosed<V>;

    fn pinch_up(self, other: Open<V>) -> OpenOrClosed<V> {
        other.pinch_up(self)
    }

    fn pinch_down(self, other: Open<V>) -> OpenOrClosed<V> {
        other.pinch_down(self)
    }
}

impl<V: PartialOrd> Pinch<Closed<V>> for OpenOrClosed<V> {
    type Up = OpenOrClosed<V>;
    type Down = OpenOrClosed<V>;

    fn pinch_up(self, other: Closed<V>) -> OpenOrClosed<V> {
        other.pinch_up(self)
    }

    fn pinch_down(self, other: Closed<V>) -> OpenOrClosed<V> {
        other.pinch_down(self)
    }
}

impl<V: PartialOrd> Pinch<OpenOrClosed<V>> for OpenOrClosed<V> {
    type Up = OpenOrClosed<V>;
    type Down = OpenOrClosed<V>;

    fn pinch_up(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> {
        match (self, other) {
            (OpenOrClosed::Open(x), OpenOrClosed::Open(y)) => {
                OpenOrClosed::Open(Open(x).pinch_up(Open(y)).0)
            },
            (OpenOrClosed::Open(x), OpenOrClosed::Closed(y)) => {
                Open(x).pinch_up(Closed(y))
            },
            (OpenOrClosed::Closed(x), OpenOrClosed::Open(y)) => {
                Closed(x).pinch_up(Open(y))
            },
            (OpenOrClosed::Closed(x), OpenOrClosed::Closed(y)) => {
                OpenOrClosed::Closed(Closed(x).pinch_up(Closed(y)).0)
            },
        }
    }

    fn pinch_down(self, other: OpenOrClosed<V>) -> OpenOrClosed<V> {
        match (self, other) {
            (OpenOrClosed::Open(x), OpenOrClosed::Open(y)) => {
                OpenOrClosed::Open(Open(x).pinch_down(Open(y)).0)
            },
            (OpenOrClosed::Open(x), OpenOrClosed::Closed(y)) => {
                Open(x).pinch_down(Closed(y))
            },
            (OpenOrClosed::Closed(x), OpenOrClosed::Open(y)) => {
                Closed(x).pinch_down(Open(y))
            },
            (OpenOrClosed::Closed(x), OpenOrClosed::Closed(y)) => {
                OpenOrClosed::Closed(Closed(x).pinch_down(Closed(y)).0)
            },
        }
    }
}

